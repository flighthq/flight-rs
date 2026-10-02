// @generated from upstream/packages/types/src/SelectionStateRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SelectionSignals;

// Source: upstream/packages/types/src/SelectionStateRuntime.ts:5 (sha256:d7e946519059317b42a07bc2570c16142aba211fc513b45efe95aefbc4f87525)
#[doc(hidden)]
pub struct SelectionStateRuntimeStorage<NodeType> {
    pub active_node: Option<NodeType>,
    pub selected_node_set: Vec<NodeType>,
    pub selected_nodes: Vec<NodeType>,
    pub signals: Option<SelectionSignals<NodeType>>,
    #[doc(hidden)]
    pub __flight_marker: std::marker::PhantomData<NodeType>,
}
impl<NodeType> Default for SelectionStateRuntimeStorage<NodeType> {
    fn default() -> Self {
        Self {
            active_node: Default::default(),
            selected_node_set: Default::default(),
            selected_nodes: Default::default(),
            signals: Default::default(),
            __flight_marker: std::marker::PhantomData,
        }
    }
}
pub type SelectionStateRuntime<NodeType> =
    <std::marker::PhantomData<NodeType> as crate::FlightEntityRuntimeMarker>::Runtime;
