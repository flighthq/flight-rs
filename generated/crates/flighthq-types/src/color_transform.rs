// @generated from upstream/packages/types/src/ColorTransform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/ColorTransform.ts:3 (sha256:f9f9ced8012658f172b2ddf822467b3392b7396be923f4e6df88055a25371ce4)
#[derive(Clone, Default)]
pub struct ColorTransform {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub alpha_multiplier: f64,
    pub alpha_offset: f64,
    pub blue_multiplier: f64,
    pub blue_offset: f64,
    pub green_multiplier: f64,
    pub green_offset: f64,
    pub red_multiplier: f64,
    pub red_offset: f64,
}
impl PartialEq for ColorTransform {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ColorTransform {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
}

// Source: upstream/packages/types/src/ColorTransform.ts:14 (sha256:2c59f1c583a0a6c81c65afc99c795f5eba9bea26a0f29d2a2e881eed00f759d3)
pub type ColorTransformLike = ColorTransform;
