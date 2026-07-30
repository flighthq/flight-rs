// @generated from upstream/packages/types/src/InteractionSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FocusEventData, KeyboardEventData, PointerEventData, Signal};

// Source: upstream/packages/types/src/InteractionSignals.ts:6 (sha256:f69ab51c1ad10e742d8cc8e2f3720122c3c28d5f9fa84ec4b855944258e8e2df)
#[derive(Clone)]
pub struct InteractionSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_click: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_context_menu: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_double_click: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_focus_in: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(FocusEventData) -> () + Send + 'static>>>,
    >,
    pub on_focus_out: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(FocusEventData) -> () + Send + 'static>>>,
    >,
    pub on_key_down: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(KeyboardEventData) -> () + Send + 'static>>>,
    >,
    pub on_key_up: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(KeyboardEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_cancel: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_down: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_move: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_out: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_over: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_roll_out: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_roll_over: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_up: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_release_outside: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_wheel: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for InteractionSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
