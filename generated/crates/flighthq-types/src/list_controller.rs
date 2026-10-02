// @generated from upstream/packages/types/src/ListController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiTransitionDescriptor, Node2D, ScrollBarController, Signal};

// Source: upstream/packages/types/src/ListController.ts:9 (sha256:be7b343d1697aaa87b7fe61c7b830ab0fee316478d9495b81b36ce1556c7d9bd)
#[derive(Clone, Default)]
pub struct ListController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for ListController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ListController {
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

// Source: upstream/packages/types/src/ListController.ts:13 (sha256:3c164da6d3dae334d0c967e81411e59dd0c7e7732a2e91b39369be0e13d612c1)
#[derive(Clone, Default)]
pub struct ListControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub content: Node2D,
    pub items: Vec<Node2D>,
    pub scroll_bar: Option<ScrollBarController>,
    pub selectable: Option<bool>,
    pub selected_index: Option<f64>,
    pub viewport: Node2D,
}
impl PartialEq for ListControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ListController.ts:22 (sha256:8df38e026823809ba95fe4ea4eaffffc2dfd277f405ca4d75597bd7ebfbb9405)
#[derive(Clone)]
pub struct ListControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_activate:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_select:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
}
impl PartialEq for ListControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
