// @generated from upstream/packages/types/src/SplitPaneController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiOrientation, GuiTransitionDescriptor, Node2D, Signal};

// Source: upstream/packages/types/src/SplitPaneController.ts:8 (sha256:42011f897c12ebc83989a0b0513611055fbf3d885e02a7f08e7c7fee4e66db03)
#[derive(Clone, Default)]
pub struct SplitPaneController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for SplitPaneController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SplitPaneController {
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

// Source: upstream/packages/types/src/SplitPaneController.ts:12 (sha256:fdaa1078c717d34ffd1675673b2911340b9d644da219e23efeb1ef6675c916fb)
#[derive(Clone, Default)]
pub struct SplitPaneControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub divider: Node2D,
    pub first_region: Node2D,
    pub maximum_first: Option<f64>,
    pub minimum_first: Option<f64>,
    pub minimum_second: Option<f64>,
    pub orientation: Option<GuiOrientation>,
    pub position: Option<f64>,
    pub second_region: Node2D,
    pub total_size: Option<f64>,
}
impl PartialEq for SplitPaneControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SplitPaneController.ts:24 (sha256:639642321e1c7ba417740ecc9e59c0a1d80b69f4916f11ab0a1eceef41178cfd)
#[derive(Clone)]
pub struct SplitPaneControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
}
impl PartialEq for SplitPaneControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
