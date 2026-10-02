// @generated from upstream/packages/types/src/ScrollViewController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiTransitionDescriptor, Node2D, ScrollBarController, Signal};

// Source: upstream/packages/types/src/ScrollViewController.ts:9 (sha256:3d63c51c503b40e00fdb11db323a7f89ede06c3c3b061d3e818275ff67fd4e8e)
#[derive(Clone, Default)]
pub struct ScrollViewController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for ScrollViewController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ScrollViewController {
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

// Source: upstream/packages/types/src/ScrollViewController.ts:13 (sha256:88ec534b1bcfef8cbc342d34bb2bbba4aebfc44e968dcab6ac7f37307543ca76)
#[derive(Clone, Default)]
pub struct ScrollViewControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub content: Node2D,
    pub horizontal_scroll_bar: Option<ScrollBarController>,
    pub mouse_wheel_enabled: Option<bool>,
    pub viewport: Node2D,
    pub vertical_scroll_bar: Option<ScrollBarController>,
}
impl PartialEq for ScrollViewControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ScrollViewController.ts:21 (sha256:18c4c167127f322acfda9ce86159f887da74a5aa231744fa38dc54f5470af273)
#[derive(Clone)]
pub struct ScrollViewControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_scroll:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
}
impl PartialEq for ScrollViewControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
