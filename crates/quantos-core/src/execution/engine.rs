use super::{arithmetic::*, types::*};
use crate::data::AdjustmentStatusV1;

#[derive(Clone, Copy)]
struct Position {
    id: u64,
    quantity: u64,
    basis: u64,
    stop: Option<u64>,
    target: Option<u64>,
}

struct State<'a> {
    input: &'a ExecutionInputV1,
    cash: u64,
    position: Option<Position>,
    realized: i64,
    fees: u64,
    slippage: u64,
    capacity: u64,
    positions_opened: u64,
    positions_closed: u64,
    sequence: u64,
    fills: Vec<ExecutionFillV1>,
    events: Vec<ExecutionEventV1>,
    snapshots: Vec<ExecutionSnapshotV1>,
}

impl<'a> State<'a> {
    fn new(input: &'a ExecutionInputV1) -> Self {
        Self {
            input,
            cash: input.config.initial_cash_paise,
            position: None,
            realized: 0,
            fees: 0,
            slippage: 0,
            capacity: 0,
            positions_opened: 0,
            positions_closed: 0,
            sequence: 0,
            fills: Vec::new(),
            events: Vec::new(),
            snapshots: Vec::new(),
        }
    }

    fn next_sequence(&mut self, ctx: Context) -> Calc<u64> {
        let sequence = self.sequence;
        self.sequence = ctx.amount(i128::from(sequence) + 1, "sequence")?;
        Ok(sequence)
    }

    fn event(
        &mut self,
        i: usize,
        phase: ExecutionPhaseV1,
        event: ExecutionEventKindV1,
    ) -> Calc<()> {
        let sequence = self.next_sequence(Context(i))?;
        self.events.push(ExecutionEventV1 {
            sequence,
            bar_index: i as u64,
            ts_utc_ms: self.input.candles[i].ts_utc_ms,
            phase,
            event,
        });
        Ok(())
    }

    fn attempt(
        &mut self,
        i: usize,
        phase: ExecutionPhaseV1,
        reason: ExecutionFillReasonV1,
        reference: u64,
    ) -> Calc<()> {
        let ctx = Context(i);
        let side = if reason == ExecutionFillReasonV1::EntrySignal {
            ExecutionSideV1::Buy
        } else {
            ExecutionSideV1::Sell
        };
        let quantity = match side {
            ExecutionSideV1::Buy => self.input.config.entry_quantity,
            ExecutionSideV1::Sell => {
                self.position
                    .ok_or_else(|| {
                        ctx.fail(ExecutionErrorCodeV1::ReconciliationFailure, "position")
                    })?
                    .quantity
            }
        };
        let limit = reason == ExecutionFillReasonV1::TakeProfit;
        // Quotes precede rejection gates; rejected fills do not compute brackets/totals.
        let q = quote(ctx, side, reference, quantity, limit, &self.input.config)?;
        let reject = |cause| ExecutionEventKindV1::OrderRejected {
            side,
            reason,
            quantity,
            cause,
        };
        if quantity > self.capacity {
            return self.event(
                i,
                phase,
                reject(ExecutionRejectionV1::InsufficientLiquidity),
            );
        }
        let change = match side {
            ExecutionSideV1::Buy => -i128::from(q.notional) - i128::from(q.fee),
            ExecutionSideV1::Sell => i128::from(q.notional) - i128::from(q.fee),
        };
        // The wider intermediate permits cash+sale notional >M when fees restore bounds.
        let signed_cash = ctx.add(self.cash.into(), change, "cash_paise")?;
        if signed_cash < 0 {
            return self.event(i, phase, reject(ExecutionRejectionV1::InsufficientCash));
        }
        let cash = ctx.amount(signed_cash, "cash_paise")?;
        let (position, realized, opened, closed, position_id) = match side {
            ExecutionSideV1::Buy => {
                let basis = ctx.amount(
                    i128::from(q.notional) + i128::from(q.fee),
                    "entry_basis_paise",
                )?;
                let (stop, target) = brackets(ctx, q.price, &self.input.config)?;
                let id = ctx.amount(i128::from(self.positions_opened) + 1, "position_id")?;
                (
                    Some(Position {
                        id,
                        quantity,
                        basis,
                        stop,
                        target,
                    }),
                    self.realized,
                    id,
                    self.positions_closed,
                    id,
                )
            }
            ExecutionSideV1::Sell => {
                let old = self.position.ok_or_else(|| {
                    ctx.fail(ExecutionErrorCodeV1::ReconciliationFailure, "position")
                })?;
                let pnl = i128::from(q.notional) - i128::from(q.fee) - i128::from(old.basis);
                let realized = ctx.signed(i128::from(self.realized) + pnl, "realized_pnl_paise")?;
                let closed =
                    ctx.amount(i128::from(self.positions_closed) + 1, "closed_positions")?;
                (None, realized, self.positions_opened, closed, old.id)
            }
        };
        let fees = ctx.amount(
            i128::from(self.fees) + i128::from(q.fee),
            "total_fees_paise",
        )?;
        let unit_slippage = if limit {
            0
        } else {
            match side {
                ExecutionSideV1::Buy => i128::from(q.price) - i128::from(reference),
                ExecutionSideV1::Sell => i128::from(reference) - i128::from(q.price),
            }
        };
        let fill_slippage = ctx.amount(
            ctx.mul(unit_slippage, quantity.into(), "slippage_paise")?,
            "slippage_paise",
        )?;
        let slippage = ctx.amount(
            i128::from(self.slippage) + i128::from(fill_slippage),
            "total_slippage_paise",
        )?;
        let sequence = self.next_sequence(ctx)?;
        // All candidate values are checked before the financial state commits.
        self.cash = cash;
        self.position = position;
        self.realized = realized;
        self.positions_opened = opened;
        self.positions_closed = closed;
        self.fees = fees;
        self.slippage = slippage;
        self.capacity -= quantity;
        self.fills.push(ExecutionFillV1 {
            sequence,
            bar_index: i as u64,
            ts_utc_ms: self.input.candles[i].ts_utc_ms,
            phase,
            reason,
            side,
            position_id,
            quantity,
            reference_price_paise: reference,
            fill_price_paise: q.price,
            notional_paise: q.notional,
            fee_paise: q.fee,
            slippage_paise: fill_slippage,
            cash_after_paise: cash,
            position_quantity_after: position.map_or(0, |p| p.quantity),
            entry_basis_after_paise: position.map_or(0, |p| p.basis),
            realized_pnl_after_paise: realized,
        });
        Ok(())
    }

    fn protection(&self, i: usize, opening: bool) -> Option<(ExecutionFillReasonV1, u64)> {
        let p = self.position?;
        let c = &self.input.candles[i];
        let observed_low = if opening { c.open } else { c.low };
        let observed_high = if opening { c.open } else { c.high };
        if let Some(stop) = p.stop
            && observed_low <= stop
        {
            return Some((ExecutionFillReasonV1::StopLoss, c.open.min(stop)));
        }
        if let Some(target) = p.target
            && observed_high >= target
        {
            return Some((ExecutionFillReasonV1::TakeProfit, target));
        }
        None
    }

    fn snapshot(&mut self, i: usize) -> Calc<()> {
        let ctx = Context(i);
        let c = &self.input.candles[i];
        let quantity = self.position.map_or(0, |p| p.quantity);
        let basis = self.position.map_or(0, |p| p.basis);
        let value = ctx.amount(
            ctx.mul(quantity.into(), c.close.into(), "market_value_paise")?,
            "market_value_paise",
        )?;
        let equity = ctx.amount(i128::from(self.cash) + i128::from(value), "equity_paise")?;
        let unrealized = ctx.signed(
            i128::from(value) - i128::from(basis),
            "unrealized_pnl_paise",
        )?;
        if i128::from(equity)
            != i128::from(self.input.config.initial_cash_paise)
                + i128::from(self.realized)
                + i128::from(unrealized)
        {
            return Err(ctx.fail(ExecutionErrorCodeV1::ReconciliationFailure, "equity_paise"));
        }
        self.snapshots.push(ExecutionSnapshotV1 {
            bar_index: i as u64,
            ts_utc_ms: c.ts_utc_ms,
            cash_paise: self.cash,
            position_quantity: quantity,
            entry_basis_paise: basis,
            market_value_paise: value,
            equity_paise: equity,
            realized_pnl_paise: self.realized,
            unrealized_pnl_paise: unrealized,
            total_fees_paise: self.fees,
            total_slippage_paise: self.slippage,
            remaining_capacity: self.capacity,
        });
        Ok(())
    }

    fn close_decision(&mut self, i: usize) -> Calc<Option<SignalDecisionV1>> {
        let decision = self.input.decisions[i];
        if decision == SignalDecisionV1::Hold {
            return Ok(None);
        }
        let eligible = match decision {
            SignalDecisionV1::EnterLong => self.position.is_none(),
            SignalDecisionV1::ExitLong => self.position.is_some(),
            SignalDecisionV1::Hold => false,
        };
        let event = if !eligible {
            ExecutionEventKindV1::DecisionIgnored { decision }
        } else if i + 1 == self.input.candles.len() {
            ExecutionEventKindV1::SignalExpired { decision }
        } else {
            return Ok(Some(decision));
        };
        self.event(i, ExecutionPhaseV1::Close, event)?;
        Ok(None)
    }
}

pub(super) fn run(input: &ExecutionInputV1) -> Calc<ExecutionReportV1> {
    let mut state = State::new(input);
    let mut pending = None;
    for (i, candle) in input.candles.iter().enumerate() {
        let ctx = Context(i);
        state.capacity = if i == 0 {
            0
        } else {
            ctx.amount(
                ctx.mul(
                    input.candles[i - 1].volume.into(),
                    input.config.participation_bps.into(),
                    "remaining_capacity",
                )? / 10_000,
                "remaining_capacity",
            )?
        };
        let flat_at_start = state.position.is_none();
        let opening_protection = state.protection(i, true);
        let mut exit_attempted = false;
        if let Some((reason, reference)) = opening_protection {
            if pending == Some(SignalDecisionV1::ExitLong) {
                state.event(
                    i,
                    ExecutionPhaseV1::Open,
                    ExecutionEventKindV1::SignalSuperseded {
                        decision: SignalDecisionV1::ExitLong,
                    },
                )?;
            }
            state.attempt(i, ExecutionPhaseV1::Open, reason, reference)?;
            exit_attempted = true;
        } else {
            match pending {
                Some(SignalDecisionV1::ExitLong) if state.position.is_some() => {
                    state.attempt(
                        i,
                        ExecutionPhaseV1::Open,
                        ExecutionFillReasonV1::ExitSignal,
                        candle.open,
                    )?;
                    exit_attempted = true;
                }
                Some(SignalDecisionV1::EnterLong) if flat_at_start => {
                    state.attempt(
                        i,
                        ExecutionPhaseV1::Open,
                        ExecutionFillReasonV1::EntrySignal,
                        candle.open,
                    )?;
                }
                _ => {}
            }
        }
        if !exit_attempted && let Some((reason, reference)) = state.protection(i, false) {
            state.attempt(i, ExecutionPhaseV1::Range, reason, reference)?;
        }
        state.snapshot(i)?;
        pending = state.close_decision(i)?;
    }
    let last = state.snapshots.last().ok_or_else(|| {
        error(
            ExecutionErrorCodeV1::ReconciliationFailure,
            None,
            "snapshots",
        )
    })?;
    let summary = ExecutionSummaryV1 {
        initial_cash_paise: input.config.initial_cash_paise,
        ending_cash_paise: last.cash_paise,
        ending_equity_paise: last.equity_paise,
        open_quantity: last.position_quantity,
        open_entry_basis_paise: last.entry_basis_paise,
        realized_pnl_paise: last.realized_pnl_paise,
        unrealized_pnl_paise: last.unrealized_pnl_paise,
        total_fees_paise: state.fees,
        total_slippage_paise: state.slippage,
        fill_count: Context(input.candles.len() - 1)
            .amount(state.fills.len() as i128, "fill_count")?,
        closed_positions: state.positions_closed,
    };
    let mut warnings = vec![ExecutionWarningV1::LiquidityProxy];
    match input.metadata.adjustment_status {
        AdjustmentStatusV1::Unknown => warnings.push(ExecutionWarningV1::UnknownAdjustment),
        AdjustmentStatusV1::Unadjusted => warnings.push(ExecutionWarningV1::UnadjustedPrices),
        _ => {}
    }
    Ok(ExecutionReportV1 {
        metadata: input.metadata.clone(),
        bar_completion: input.bar_completion,
        config: input.config.clone(),
        decisions: input.decisions.clone(),
        warnings,
        fills: state.fills,
        events: state.events,
        snapshots: state.snapshots,
        summary,
    })
}
