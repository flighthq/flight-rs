// @generated from upstream/packages/types/src/InputKeyRepeatTimer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InputKeyRepeatTimer.ts:7 (sha256:12a602a0900998e64d810088c79f26b8246bedf16aa92f99691104e87081a884)
#[derive(Clone)]
pub struct InputKeyRepeatTimer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub start: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub stop: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for InputKeyRepeatTimer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
