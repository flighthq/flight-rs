// @generated from upstream/packages/types/src/NodeOrderList.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Node, NodeTraits};

// Source: upstream/packages/types/src/NodeOrderList.ts:3 (sha256:34d9610273d6d5549f5251139cd3fa36c98c0cf3fe8ba346c354fe0eab3168b9)
#[derive(Clone, Default)]
pub struct NodeOrderList {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub entry_count: f64,
    pub nodes: Vec<Node>,
    pub sort_keys: Vec<f64>,
}
impl PartialEq for NodeOrderList {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NodeOrderList {
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

// Source: upstream/packages/types/src/NodeOrderList.ts:8 (sha256:6bda418823620eb1c5f5a2bc699b56573eaea0706178505538b22c024a0f59f6)
pub struct NodeOrderListEntryVisitor<Traits = NodeTraits>(
    pub  std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Node, f64, f64) -> crate::FlightUnion2<bool, ()> + Send + 'static>,
        >,
    >,
    pub core::marker::PhantomData<fn() -> (Traits,)>,
);
impl<Traits> Clone for NodeOrderListEntryVisitor<Traits> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}
