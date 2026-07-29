// @generated from upstream/packages/types/src/Lifecycle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Lifecycle.ts:3 (sha256:931d083a169061720cbac42f89074d27afca46d8a08ce35b69d484cf86048cb1)
pub type AppLifecycleState = String;

// Source: upstream/packages/types/src/Lifecycle.ts:7 (sha256:5a53941968e6de824e4edaa553463f20c24ed90b4606928adb697165bd019b9d)
pub type AppLaunchKind = String;

// Source: upstream/packages/types/src/Lifecycle.ts:11 (sha256:a1d378acb3277cc7ced84be8634c24b28dc9eec481f194eead4e9cff03590f45)
pub type AppMemoryPressure = String;

// Source: upstream/packages/types/src/Lifecycle.ts:16 (sha256:03ca8563ffb8efc48fd12319d2e69063bba9996dbda8ac0dbe4eb56caac633e9)
#[derive(Clone)]
pub struct LifecycleBackend {
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
impl PartialEq for LifecycleBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Lifecycle.ts:30 (sha256:e1e5b2e2eae928794ef748e91dc725ac0315d3ea36f3e762f005931fb53e093f)
#[derive(Clone)]
pub struct AppLifecycle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
                Box<dyn FnMut(Vec<(String, crate::OpaqueHostValue)>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub on_restore_state: Signal<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Vec<(String, crate::OpaqueHostValue)>) -> () + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for AppLifecycle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
