// @generated from upstream/packages/input/src/inputManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{InputManager, InputSignals};

// Source: upstream/packages/input/src/inputManager.ts:249 (sha256:e9e3c2ef2dc2a0db5c673e08733b17f07461c1fe2ed675ff90c54a569417fd5e)
pub fn create_input_manager() -> InputManager {
    let out = allocate_entity();
    initialize_input_manager((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/input/src/inputManager.ts:255 (sha256:8f69d0370e699f97bfd78198c61dbba72bd57aa81e938f737649e22abd969bf7)
pub fn create_input_signals() -> InputSignals {
    let out = allocate_entity();
    initialize_input_signals((out).clone());
    return finish_entity((out).clone());
}
