// @generated from upstream/packages/types/src/BitmapShadowBlurOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapShadowBlurOptions.ts:1 (sha256:03add61a0272a18aaac9149a5db369e1491b5c0c1aec3b78ec09b36bc3dd6a8b)
#[derive(Clone, Default)]
pub struct BitmapShadowBlurOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
}
impl PartialEq for BitmapShadowBlurOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
