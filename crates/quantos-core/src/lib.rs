//! Shared foundation, normalization, deterministic execution and SMA decisions.

pub mod data;
pub mod execution;
pub mod sma;

/// Describes the unchanged P0 build-stage surface; domain APIs are separate.
pub const FOUNDATION_STATUS: &str = "QuantOS P0 foundation; market functionality unavailable";

/// Returns static build-stage information; does not process data or compute results.
pub fn foundation_status() -> &'static str {
    FOUNDATION_STATUS
}
