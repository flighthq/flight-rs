// @generated from upstream/packages/types/src/Flow.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Flow.ts:16 (sha256:1dcf59ee7b59c493adf889564e539b8495ed2a5e9f0b6145954c455cb28fe588)
#[derive(Clone)]
pub struct FlowState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: Option<String>,
    pub on_enter: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_exit: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_pause: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_resume:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_update:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub render_below: Option<bool>,
    pub update_below: Option<bool>,
}
impl PartialEq for FlowState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Flow.ts:29 (sha256:3e145f4a7645c5e37bb6f4d5be12c006ea332396a6f233562d12006faa6dd9e0)
#[derive(Clone)]
pub struct FlowStack {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub states: Vec<FlowState>,
}
impl PartialEq for FlowStack {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
