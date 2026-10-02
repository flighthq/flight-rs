// @generated from upstream/packages/types/src/GuiDialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, FocusManager, GuiTransitionDescriptor, Node2D, Signal};

// Source: upstream/packages/types/src/GuiDialog.ts:9 (sha256:14a6c1614e605d0ca264e09ab612c6fe86d991e4c6a2f8ac2882098f8b3fd7c0)
#[derive(Clone, Default)]
pub struct GuiDialog {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for GuiDialog {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GuiDialog {
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

// Source: upstream/packages/types/src/GuiDialog.ts:13 (sha256:308544562f33c4221313faeb9f7a8de9f9712f97c8dc38745c206acda5e6e97f)
pub type GuiDialogCloseReason = String;

// Source: upstream/packages/types/src/GuiDialog.ts:15 (sha256:f85bee04ee920ddcb67d8aed3782e56eda1ae7ae1db48812680cc532793dd609)
#[derive(Clone, Default)]
pub struct GuiDialogCloseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry_id: String,
    pub reason: GuiDialogCloseReason,
    pub value: Option<crate::FlightValue>,
}
impl PartialEq for GuiDialogCloseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GuiDialog.ts:21 (sha256:316f03272933f7fa112d68a0020ae44168b24e8ae7a0f9433df9d93eb0e42ae5)
#[derive(Clone, Default)]
pub struct GuiDialogEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub dismiss_on_backdrop: Option<bool>,
    pub id: String,
    pub initial_focus: Option<Node2D>,
    pub root: Node2D,
}
impl PartialEq for GuiDialogEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GuiDialog.ts:28 (sha256:45b1edf372271783762c605941843d96bc9aad1010daaf882ba36a485fbeb200)
#[derive(Clone, Default)]
pub struct GuiDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub backdrop: Option<Node2D>,
    pub focus_manager: Option<FocusManager<Node2D>>,
}
impl PartialEq for GuiDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GuiDialog.ts:33 (sha256:6869f4f4050b01cfcd188bde2318066315475671539aa170db86ea06615f7a35)
#[derive(Clone)]
pub struct GuiDialogSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_active_change: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Option<GuiDialogEntry>) -> () + Send + 'static>>,
        >,
    >,
    pub on_close: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(GuiDialogCloseResult) -> () + Send + 'static>>,
        >,
    >,
    pub on_queue_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for GuiDialogSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
