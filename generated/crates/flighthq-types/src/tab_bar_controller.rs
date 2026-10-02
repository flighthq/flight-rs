// @generated from upstream/packages/types/src/TabBarController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiTransitionDescriptor, Node2D, Signal};

// Source: upstream/packages/types/src/TabBarController.ts:8 (sha256:5b34bd894af94a54b833a171647bbf37f5e832fd0a9a634530b4f972746f239a)
#[derive(Clone, Default)]
pub struct TabBarController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for TabBarController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TabBarController {
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

// Source: upstream/packages/types/src/TabBarController.ts:12 (sha256:b949cbf2fcf3c3fbade49fbe2639ce4206beaa68da28174714348d6ed9f06eb4)
#[derive(Clone, Default)]
pub struct TabBarControllerItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub selected_state: Node2D,
    pub unselected_state: Node2D,
}
impl PartialEq for TabBarControllerItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TabBarController.ts:17 (sha256:8cfefc6f4a55c128703e77088a1d969eea92ee29f5d401ffc1ea7de49adddf24)
#[derive(Clone, Default)]
pub struct TabBarControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub selected_index: Option<f64>,
    pub tabs: Vec<TabBarControllerItem>,
}
impl PartialEq for TabBarControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TabBarController.ts:22 (sha256:14ca4d41af074633810004a7e1b2bb70eb45051927e4e6c3553c692fe906ad3a)
#[derive(Clone)]
pub struct TabBarControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
}
impl PartialEq for TabBarControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
