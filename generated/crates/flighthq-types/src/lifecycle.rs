// @generated from upstream/packages/types/src/Lifecycle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/Lifecycle.ts:4 (sha256:931d083a169061720cbac42f89074d27afca46d8a08ce35b69d484cf86048cb1)
pub type AppLifecycleState = String;

// Source: upstream/packages/types/src/Lifecycle.ts:8 (sha256:5a53941968e6de824e4edaa553463f20c24ed90b4606928adb697165bd019b9d)
pub type AppLaunchKind = String;

// Source: upstream/packages/types/src/Lifecycle.ts:12 (sha256:a1d378acb3277cc7ced84be8634c24b28dc9eec481f194eead4e9cff03590f45)
pub type AppMemoryPressure = String;

// Source: upstream/packages/types/src/Lifecycle.ts:17 (sha256:c78b414f06b22804ffbd774ef84fb3b479a2791cda41c26465698e6f3cb6b655)
#[derive(Clone)]
pub struct HostLifecycleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> AppLifecycleState + Send + 'static>>>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub get_launch_kind: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> AppLaunchKind + Send + 'static>>>,
    >,
    pub subscribe_memory_warning: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(AppMemoryPressure) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for HostLifecycleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Lifecycle.ts:31 (sha256:120b0d6702d00ae3c098ff8443218e8dd3f983efda310de37c4fcc24309cf050)
#[derive(Clone)]
pub struct AppLifecycle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_state_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppLifecycleState) -> () + Send + 'static>>>,
    >,
    pub on_resume:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_pause: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_back_button:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_memory_warning: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppMemoryPressure) -> () + Send + 'static>>>,
    >,
    pub on_save_state: Signal<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Vec<(String, crate::FlightValue)>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub on_restore_state: Signal<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Vec<(String, crate::FlightValue)>) -> () + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for AppLifecycle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AppLifecycle {
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

// Source: upstream/packages/types/src/Lifecycle.ts:47 (sha256:5a91dac399197b5fca02b62ae4a9b439e0ee939cfdcbeb4524fb21ec21d41ab3)
pub type LifecycleOperation = HostLifecycleCapability;
