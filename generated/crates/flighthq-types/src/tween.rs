// @generated from upstream/packages/types/src/Tween.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EasingFunction, EntityRuntime, Signal, TweenPropertyDetail};

// Source: upstream/packages/types/src/Tween.ts:6 (sha256:d732852a88003908bfaac103eead6e12d9e95c61b91d5a9bfc4d24ef22fb5d37)
pub struct NumericProps<T>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (T,)>,
);
impl<T> Clone for NumericProps<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}

// Source: upstream/packages/types/src/Tween.ts:8 (sha256:b1af3648a4eef16a13d6f1c355b1e6551dae8ea818df48f304ec7f5b865094d8)
#[derive(Clone)]
pub struct Tween<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
    pub property_map: NumericProps<T>,
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
impl<T: Clone + Send + Sync + 'static> crate::FlightEntity for Tween<T> {
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
