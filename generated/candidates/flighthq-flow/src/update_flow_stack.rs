// @generated from upstream/packages/flow/src/updateFlowStack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FlowStack;

// Source: upstream/packages/flow/src/updateFlowStack.ts:8 (sha256:eb688c48071945a1aabcc22f1874b54a7c97c5f3319211d91ef76baf7a74855f)
pub fn update_flow_stack(stack: &FlowStack, delta_time: f64) -> () {
    let mut index = ((stack.states.len() as f64) - 1.0_f64);
    if (index < 0.0_f64) {
        return;
    }
    {
        let __flight_callback = (stack.states[index as usize].on_update).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()(delta_time))
    };
    while ((index > 0.0_f64) && stack.states[index as usize].update_below) {
        {
            index -= 1.0;
            index
        };
        {
            let __flight_callback = (stack.states[index as usize].on_update).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()(delta_time))
        };
    }
}
