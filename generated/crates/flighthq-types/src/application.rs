// @generated from upstream/packages/types/src/Application.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ApplicationWindow, Signal};

// Source: upstream/packages/types/src/Application.ts:4 (sha256:1cdb6443a031c2dd6bb70f35acb48bfa8a63bbbc863664fa48bf00be2dd26184)
#[derive(Clone)]
pub struct Application {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub delta_time: f64,
    pub elapsed_time: f64,
    pub frame_count: f64,
    pub interpolation_alpha: f64,
    pub is_running: bool,
    pub on_activate:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_deactivate:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_error: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
            >,
        >,
    >,
    pub on_exit: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_fixed_update: Option<
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    >,
    pub on_render:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_update:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub windows: Vec<ApplicationWindow>,
}
impl PartialEq for Application {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
