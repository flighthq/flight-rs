// @generated from upstream/packages/types/src/NodeOrderList.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Node;

// Source: upstream/packages/types/src/NodeOrderList.ts:2 (sha256:ac6d71dd26dbaa9e99b676efee5645d01bc94b02da6199e93c5b674e43b77e92)
#[derive(Clone, Default)]
pub struct NodeOrderList {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry_count: f64,
    pub nodes: Vec<Node>,
    pub sort_keys: Vec<f64>,
}
impl PartialEq for NodeOrderList {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NodeOrderList.ts:7 (sha256:6bda418823620eb1c5f5a2bc699b56573eaea0706178505538b22c024a0f59f6)
pub type NodeOrderListEntryVisitor = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(Node, f64, f64) -> crate::FlightUnion2<bool, ()> + Send + 'static>,
    >,
>;
