//! Browser adapter. No browser IO or financial behavior in Phase 0.

use wasm_bindgen::prelude::*;

/// Exposes the shared core's build-stage information to future browser bindings.
#[wasm_bindgen]
pub fn foundation_status() -> String {
    quantos_core::foundation_status().to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn adapter_preserves_the_shared_core_status() {
        assert_eq!(
            super::foundation_status(),
            quantos_core::foundation_status()
        );
    }
}
