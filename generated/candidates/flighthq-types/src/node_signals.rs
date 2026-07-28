// @generated from upstream/packages/types/src/NodeSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{NodeAny, Signal};

// Source: upstream/packages/types/src/NodeSignals.ts:4 (sha256:3debcb6bbfc793c4ea95337f8afd26ae83f81485392ba271ba5300c0a5eb35fa)
#[derive(Clone)]
pub struct NodeSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_child_added:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(NodeAny) -> () + Send + 'static>>>>,
    pub on_child_removed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(NodeAny) -> () + Send + 'static>>>>,
    pub on_children_changed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_children_order_changed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_parent_changed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for NodeSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
