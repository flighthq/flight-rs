// @generated from upstream/packages/flow/src/pushFlowState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{FlowStack, FlowState};

// Source: upstream/packages/flow/src/pushFlowState.ts:6 (sha256:3e3f6d6dc3ce40bdbb2e9add91b1fe41bda7b33e690b532d08bf38efce660305)
pub fn push_flow_state(stack: &mut FlowStack, state: &FlowState) -> () {
    let previous_top = if ((stack.states.len() as f64) > 0.0_f64) {
        stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone()
    } else {
        None
    };
    {
        let __flight_callback = (previous_top.on_pause).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    stack.states.push(((*state).clone()).clone());
    {
        let __flight_callback = (state.on_enter).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
}
