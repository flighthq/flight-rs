// @generated from upstream/packages/types/src/FocusManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, InputSignals, Node};

// Source: upstream/packages/types/src/FocusManager.ts:5 (sha256:b6364ae797462e127ef510c7f5ae8e457e58e9220be2670bc2323ea432914101)
pub type FocusDirection = String;

// Source: upstream/packages/types/src/FocusManager.ts:20 (sha256:a230758fbdbe4fc4081de9ab81e8e05653c961c3be5685b605441c32c0790a51)
#[derive(Clone)]
pub struct FocusManager<N = Node> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub focused: Option<N>,
    pub root: N,
    pub wrap: bool,
}
impl<N> PartialEq for FocusManager<N> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl<N: Clone + Send + Sync + 'static> crate::FlightEntity for FocusManager<N> {
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

// Source: upstream/packages/types/src/FocusManager.ts:26 (sha256:f7ba8b429aae2588af29d1706a7960c1210c571898c5e9aee9b3644104e82811)
#[derive(Clone, Default)]
pub struct FocusManagerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub wrap: Option<bool>,
}
impl PartialEq for FocusManagerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FocusManager.ts:32 (sha256:68ddc2493463eb67824ddeca7e1ac85925757a209b4332deeb6440949341897a)
#[derive(Clone, Default)]
pub struct FocusNavigationOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub arrow_keys: Option<bool>,
}
impl PartialEq for FocusNavigationOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FocusManager.ts:40 (sha256:993a1607a7ad0aae944fc6743ad2357c20aa18b84ba327f36b8446e304586add)
pub type FocusNavigationInput = InputSignals;
