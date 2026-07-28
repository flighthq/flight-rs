// @generated from upstream/packages/flow/src/clearFlowStack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FlowStack;

// Source: upstream/packages/flow/src/clearFlowStack.ts:6 (sha256:c807f627ebaf016391700043b587ec56a4c00cf6083c98ae65e4bb8738751c96)
pub fn clear_flow_stack(stack: &mut FlowStack) -> () {
    {
        let mut i = ((stack.states.len() as f64) - 1.0_f64);
        while (i >= 0.0_f64) {
            {
                let __flight_callback = (stack.states[i as usize].on_exit).clone();
                __flight_callback
                    .as_ref()
                    .map(|callback| callback.lock().unwrap()())
            };
            {
                i -= 1.0;
                i
            };
        }
    }
    stack.states.clear();
}
