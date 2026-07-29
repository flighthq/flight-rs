// @generated from upstream/packages/types/src/Rectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Rectangle.ts:3 (sha256:a9ecbfbf0841ea1a0d47b8480c419819b40456e51a9e8b25618ffca45387048b)
#[derive(Clone, Default)]
pub struct Rectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Rectangle {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
}

// Source: upstream/packages/types/src/Rectangle.ts:10 (sha256:fb1c098e5031fbb279686c185089f1239ca093472a8c262566d2ac143f6809d5)
pub type RectangleLike = Rectangle;
