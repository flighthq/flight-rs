// @generated from upstream/packages/flow/src/getFlowStackVisibleStates.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{FlowStack, FlowState};

// Source: upstream/packages/flow/src/getFlowStackVisibleStates.ts:9 (sha256:1db2c021ed4c1ea7e5cfe34c5df44f206dd6f94d22c9b61b469f523112288ab5)
pub fn get_flow_stack_visible_states(stack: &FlowStack, out: &mut Vec<FlowState>) -> () {
    out.clear();
    let mut top = ((stack.states.len() as f64) - 1.0_f64);
    if (top < 0.0_f64) {
        return;
    }
    let mut lowest = top;
    while (lowest > 0.0_f64) && ((stack.states[lowest as usize].render_below).unwrap_or(false)) {
        {
            lowest -= 1.0;
            lowest
        };
    }
    {
        let mut i = lowest;
        while (i <= top) {
            out.push(stack.states[i as usize].clone());
            {
                i += 1.0;
                i
            };
        }
    }
}
