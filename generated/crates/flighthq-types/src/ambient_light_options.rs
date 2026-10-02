// @generated from upstream/packages/types/src/AmbientLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::LightUnit;

// Source: upstream/packages/types/src/AmbientLightOptions.ts:3 (sha256:ae4107aa85402ac4d2cb375e8df6d37f1c0ce3ec2b1b3d460c302a91498cef91)
#[derive(Clone, Default)]
pub struct AmbientLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Option<f64>,
    pub enabled: Option<bool>,
    pub intensity: Option<f64>,
    pub intensity_unit: Option<LightUnit>,
}
impl PartialEq for AmbientLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
