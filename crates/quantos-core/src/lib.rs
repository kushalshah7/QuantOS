//! Shared foundation boundary and approved core data normalization.

pub mod data;

/// Describes the foundation without claiming that an engine exists.
pub const FOUNDATION_STATUS: &str = "QuantOS P0 foundation; market functionality unavailable";

/// Returns static build-stage information; does not process data or compute results.
pub fn foundation_status() -> &'static str {
    FOUNDATION_STATUS
}
