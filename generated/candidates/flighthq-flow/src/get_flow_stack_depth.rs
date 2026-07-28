// @generated from upstream/packages/flow/src/getFlowStackDepth.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FlowStack;

// Source: upstream/packages/flow/src/getFlowStackDepth.ts:4 (sha256:859731f97f3b4ad8ec9d9a781a9b4760bca6ac828425799a581c852010847a4a)
pub fn get_flow_stack_depth(stack: &FlowStack) -> f64 {
    return (stack.states.len() as f64);
}
