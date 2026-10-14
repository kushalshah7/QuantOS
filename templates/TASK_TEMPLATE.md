# QOS-XXXX — <single verifiable task>

- **Status:** DRAFT (change to APPROVED only after owner review)
- **Phase:** P#
- **Branch:** `feature/qos-xxxx-short-name` (from `develop`)
- **PR target:** `develop`
- **Risk:** low | medium | high

## Goal
One sentence describing what works when done.

## Read first (maximum 2–5 relevant files)
- `MODULE_MAP.md`
- `<specific relevant contract>`
- `<specific implementation path or test>`

## Allowed edits
- `<owning/module/**>`
- `<matching/module/tests/**>`
- `docs/tasks/QOS-XXXX-*.md` (handoff status only)
- `PROGRESS.md` (one short evidence entry)

## Locked contract / invariants
- Inputs: ...
- Outputs: ...
- API/type signatures: ...
- Edge cases: ...
- Must not: new dependencies, public API changes, unrelated refactoring, private data commits (unless separately approved).

## Exact implementation
1. ...
2. ...
3. ...

## Verification and proof
- `command`: expected outcome
- `command`: expected outcome
- User-observable demonstration: ...

## Review/stop gate
- All relevant tests passed, required CI green, commits pushed and PR opened.
- No merges, releases or next-phase tasks.
- If the contract needs amendment, stop with `BLOCKED` and a proposed one-line change.

## Handoff (Codex fills after work)
- Status: NOT STARTED | BLOCKED | REVIEW
- Paths changed:
- Tests/results:
- Remote commit SHA:
- PR link:
- Risks / next decision:
