// @generated from upstream/packages/types/src/HostInputIngress.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AttachInputOptions, InputGamepadAxisData, InputGamepadButtonData, InputGamepadConnectData,
    InputKeyboardData, InputPointerData, InputTextData,
};

// Source: upstream/packages/types/src/HostInputIngress.ts:11 (sha256:0c7206c0c9e4e3d068219b8a1add37daf78a9a168d347ea2c603944968d183eb)
pub type InputIngressSource = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/HostInputIngress.ts:14 (sha256:30cf39f3451679967caceece474803e3ee89a2af5510416bd78f182bf1e8d50e)
#[derive(Clone)]
pub struct InputIngressSink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub gamepad_axis_move: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadAxisData) -> () + Send + 'static>>,
    >,
    pub gamepad_button_down: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>>,
    >,
    pub gamepad_button_up: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>>,
    >,
    pub gamepad_connect: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>>,
    >,
    pub gamepad_disconnect: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>>,
    >,
    pub is_enabled: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub key_down:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>>>,
    pub key_up:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>>>,
    pub pointer_cancel:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    pub pointer_down:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    pub pointer_move:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    pub pointer_move_relative:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    pub pointer_up:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    pub text_edit:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputTextData) -> () + Send + 'static>>>,
    pub text_input:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputTextData) -> () + Send + 'static>>>,
    pub wheel:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
}
impl PartialEq for InputIngressSink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostInputIngress.ts:37 (sha256:104d016b9c4906e73b5d6025b33cbb4ec00fdc6334783767b52d955bf08eebf5)
#[derive(Clone)]
pub struct HostInputIngressCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach_gamepad: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputIngressSource,
                        InputIngressSink,
                        Option<AttachInputOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub attach_keyboard: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputIngressSource,
                        InputIngressSink,
                        Option<AttachInputOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub attach_pointer: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputIngressSource,
                        InputIngressSink,
                        Option<AttachInputOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub attach_relative_pointer: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputIngressSource,
                        InputIngressSink,
                        Option<AttachInputOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub attach_text: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputIngressSource,
                        InputIngressSink,
                        Option<AttachInputOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub attach_wheel: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        InputIngressSource,
                        InputIngressSink,
                        Option<AttachInputOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostInputIngressCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
