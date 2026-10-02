// @generated from upstream/packages/types/src/ScrollBarController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiOrientation, GuiTransitionDescriptor, Node2D, Signal};

// Source: upstream/packages/types/src/ScrollBarController.ts:8 (sha256:098cbf2fb2326addb780cb6cd337975b535cf8b2067701dba35e7c5cf2c720ee)
#[derive(Clone, Default)]
pub struct ScrollBarController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for ScrollBarController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ScrollBarController {
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

// Source: upstream/packages/types/src/ScrollBarController.ts:12 (sha256:340495e53486e00fe204dd5bada4910083d4d1c1518eae1e540ded91485f00ea)
#[derive(Clone, Default)]
pub struct ScrollBarControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub down_button: Option<Node2D>,
    pub line_size: Option<f64>,
    pub maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub orientation: Option<GuiOrientation>,
    pub page_size: Option<f64>,
    pub repeat_interval: Option<f64>,
    pub thumb: Node2D,
    pub track: Node2D,
    pub up_button: Option<Node2D>,
    pub value: Option<f64>,
}
impl PartialEq for ScrollBarControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ScrollBarController.ts:26 (sha256:be1347d50a8be13c2d393385e7fb96f40df707d8bc88d57b10795519ca5ce742)
#[derive(Clone)]
pub struct ScrollBarControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
}
impl PartialEq for ScrollBarControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
