// @generated from upstream/packages/flow/src/popFlowState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{FlowStack, FlowState};

// Source: upstream/packages/flow/src/popFlowState.ts:6 (sha256:d91786875c92cc6fbc3cc9a868990e775b286c0cc40fe97501a7e1f73467d59b)
pub fn pop_flow_state(stack: &mut FlowStack) -> Option<FlowState> {
    if ((stack.states.len() as f64) == 0.0_f64) {
        return None;
    }
    let popped = stack
        .states
        .pop()
        .expect("TypeScript Array.pop returned undefined");
    {
        let __flight_callback = (popped.on_exit).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let revealed = if ((stack.states.len() as f64) > 0.0_f64) {
        stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone()
    } else {
        None
    };
    {
        let __flight_callback = (revealed.on_resume).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    return Some((popped).clone());
}
