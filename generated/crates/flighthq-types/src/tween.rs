// @generated from upstream/packages/types/src/Tween.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EasingFunction, Signal, TweenPropertyDetail};

// Source: upstream/packages/types/src/Tween.ts:5 (sha256:d732852a88003908bfaac103eead6e12d9e95c61b91d5a9bfc4d24ef22fb5d37)
pub type NumericProps = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Tween.ts:7 (sha256:6903b4fa8a509237f7ff329abd18f797ff86f22fe5115266aa530240bdad1859)
#[derive(Clone)]
pub struct Tween<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub complete: bool,
    pub delay: f64,
    pub duration: f64,
    pub ease: EasingFunction,
    pub elapsed: f64,
    pub initialized: bool,
    pub on_complete:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_repeat:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_update:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_yoyo: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub paused: bool,
    pub properties: Vec<TweenPropertyDetail>,
    pub property_map: NumericProps,
    pub reflect: bool,
    pub repeat: f64,
    pub reverse: bool,
    pub smart_rotation: bool,
    pub snapping: bool,
    pub target: T,
}
impl<T> PartialEq for Tween<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
