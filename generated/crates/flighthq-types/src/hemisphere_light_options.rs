// @generated from upstream/packages/types/src/HemisphereLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::LightUnit;

// Source: upstream/packages/types/src/HemisphereLightOptions.ts:3 (sha256:c90048e96743385186b09ea7502930d693373c6f8c47f74d2a697988f17cf614)
#[derive(Clone, Default)]
pub struct HemisphereLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: Option<bool>,
    pub ground_color: Option<f64>,
    pub intensity: Option<f64>,
    pub intensity_unit: Option<LightUnit>,
    pub sky_color: Option<f64>,
}
impl PartialEq for HemisphereLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
