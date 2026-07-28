// @generated from upstream/packages/flow/src/createFlowStack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FlowStack;

// Source: upstream/packages/flow/src/createFlowStack.ts:5 (sha256:559dc3b60dfa4f8b221042c9de38a615a1da1be1f893c16828715200449a30ca)
pub fn create_flow_stack() -> FlowStack {
    return FlowStack {
        __flight_identity: std::sync::Arc::new(()),
        states: vec![],
    };
}
