// @generated from upstream/packages/types/src/AutoExposureEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AutoExposureEffect.ts:2 (sha256:834f79b4ccae06ffa00550eb4dcb4ec4d73ac6aa47f0e8dabbca2c0b469ae815)
#[derive(Clone)]
pub struct AutoExposureEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub adaptation_speed: Option<f64>,
    pub exposure_compensation: Option<f64>,
    pub max_exposure: Option<f64>,
    pub min_exposure: Option<f64>,
}
impl PartialEq for AutoExposureEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
