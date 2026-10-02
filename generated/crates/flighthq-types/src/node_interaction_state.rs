// @generated from upstream/packages/types/src/NodeInteractionState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Cursor, EntityRuntime, HitArea};

// Source: upstream/packages/types/src/NodeInteractionState.ts:19 (sha256:bff7477193712da4a801a4f1d766504584bdb8f8acf41e3179cd39add9ab1340)
#[derive(Clone, Default)]
pub struct NodeInteractionState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub hit_test_enabled: bool,
    pub children_hit_test_enabled: bool,
    pub hit_area: Option<HitArea>,
    pub cursor: Option<Cursor>,
    pub focusable: bool,
    pub pointer_double_click_enabled: bool,
    pub tab_index: f64,
}
impl PartialEq for NodeInteractionState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NodeInteractionState {
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
