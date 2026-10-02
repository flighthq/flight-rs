// @generated from upstream/packages/types/src/SelectionState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, HierarchyNodeAny, Signal};

// Source: upstream/packages/types/src/SelectionState.ts:7 (sha256:12b77718bf8b330e09080f1203c017ab2928975970397f8d09069604c6953f73)
#[derive(Clone, Default)]
pub struct SelectionModifierState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alt_key: bool,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub shift_key: bool,
}
impl PartialEq for SelectionModifierState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SelectionState.ts:14 (sha256:951fb30a9303bdf8dba6a53f77507b33ee2800e96c2f0c51c0852807c2a61bc6)
#[derive(Clone)]
pub struct SelectionSignals<NodeType = HierarchyNodeAny> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_active_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<NodeType>) -> () + Send + 'static>>>,
    >,
    pub on_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<NodeType>) -> () + Send + 'static>>>,
    >,
}
impl<NodeType> PartialEq for SelectionSignals<NodeType> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SelectionState.ts:20 (sha256:77db5d6fb823150c4fd2fdd17f073fc5065d98a438c771cdf98d7199ecb64157)
#[derive(Clone, Default)]
pub struct SelectionState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for SelectionState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SelectionState {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
