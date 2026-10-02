// @generated from upstream/packages/types/src/TreeViewController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiTransitionDescriptor, Node2D, Signal};

// Source: upstream/packages/types/src/TreeViewController.ts:8 (sha256:c800919f171ffce2b5e181441a32fc072bedea54b09ddaf594af8f42f41d574f)
#[derive(Clone, Default)]
pub struct TreeViewController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for TreeViewController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TreeViewController {
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

// Source: upstream/packages/types/src/TreeViewController.ts:12 (sha256:c7b2c2cee77a99bd23bde62a5e669d833515a9e2e330062c784e650ff330fce3)
#[derive(Clone, Default)]
pub struct TreeViewControllerItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub children: Option<Vec<TreeViewControllerItem>>,
    pub expanded: Option<bool>,
    pub visual: Node2D,
}
impl PartialEq for TreeViewControllerItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TreeViewController.ts:18 (sha256:3e34698304c2160ea8b2e67a0a38512ae6cbb395f9fc1237daac93cb3c6ad2a4)
#[derive(Clone, Default)]
pub struct TreeViewControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub items: Vec<TreeViewControllerItem>,
    pub selected_item: Option<TreeViewControllerItem>,
}
impl PartialEq for TreeViewControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TreeViewController.ts:23 (sha256:46bc487866b6519fcb8256a52ace9cb5298962178da7d18321dec6b920890245)
#[derive(Clone)]
pub struct TreeViewControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_activate: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(TreeViewControllerItem) -> () + Send + 'static>>,
        >,
    >,
    pub on_expand_change: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(TreeViewControllerItem, bool) -> () + Send + 'static>>,
        >,
    >,
    pub on_select: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Option<TreeViewControllerItem>) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for TreeViewControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
