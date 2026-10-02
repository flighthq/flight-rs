// @generated from upstream/packages/types/src/PhotometricLightLike.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::LightUnit;

// Source: upstream/packages/types/src/PhotometricLightLike.ts:11 (sha256:6d97d91b0a0cca23f24a6637904c7ef889c70499b424c931f2bed6e7b1343682)
#[derive(Clone, Default)]
pub struct PhotometricLightLike {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: f64,
    pub intensity_unit: LightUnit,
}
impl PartialEq for PhotometricLightLike {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
