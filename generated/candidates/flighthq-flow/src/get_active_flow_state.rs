// @generated from upstream/packages/flow/src/getActiveFlowState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{FlowStack, FlowState};

// Source: upstream/packages/flow/src/getActiveFlowState.ts:5 (sha256:2cf869d1898ccb66ba939fdb648b6c9e49d16aa544aa157d81ec455698dc4b5d)
pub fn get_active_flow_state(stack: &FlowStack) -> Option<FlowState> {
    return if ((stack.states.len() as f64) > 0.0_f64) {
        Some(stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone())
    } else {
        None
    };
}
