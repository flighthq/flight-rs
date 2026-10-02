// @generated from upstream/packages/types/src/SliderController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiOrientation, GuiTransitionDescriptor, Node2D, Signal};

// Source: upstream/packages/types/src/SliderController.ts:8 (sha256:bb43c1b03d39dccee5431ff1083523f8a3d10a62f9f39b28bcbcb99ca2bd68b6)
#[derive(Clone, Default)]
pub struct SliderController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for SliderController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SliderController {
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

// Source: upstream/packages/types/src/SliderController.ts:12 (sha256:00f4cd0641a2417b665700f9dfef6cffdfd73f01b83c8d868207b76bbb7b7b6a)
#[derive(Clone, Default)]
pub struct SliderControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub orientation: Option<GuiOrientation>,
    pub step: Option<f64>,
    pub thumb: Node2D,
    pub track: Node2D,
    pub value: Option<f64>,
}
impl PartialEq for SliderControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SliderController.ts:22 (sha256:d9a819b07ed1c662de6de69bd954f83b6217472241fd32d60e805dec5b9d1731)
#[derive(Clone)]
pub struct SliderControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
}
impl PartialEq for SliderControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
