// @generated from upstream/packages/flow/src/replaceFlowState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{FlowStack, FlowState};

// Source: upstream/packages/flow/src/replaceFlowState.ts:7 (sha256:45909cbe992bf399b284f4d6ccc462d9c05306eb7e0d39a516feab708232d35d)
pub fn replace_flow_state(stack: &mut FlowStack, state: &FlowState) -> () {
    if ((stack.states.len() as f64) > 0.0_f64) {
        let previous_top = stack
            .states
            .pop()
            .expect("TypeScript Array.pop returned undefined");
        {
            let __flight_callback = (previous_top.on_exit).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()())
        };
    }
    stack.states.push(((*state).clone()).clone());
    {
        let __flight_callback = (state.on_enter).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
}
