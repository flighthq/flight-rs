// @generated from upstream/packages/types/src/HemisphereLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/HemisphereLightOptions.ts:1 (sha256:f48e11d8bbf94531f6345b7c6fb837bd4692b35a64107828f8df618c7885f7e5)
#[derive(Clone, Default)]
pub struct HemisphereLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ground_color: Option<f64>,
    pub intensity: Option<f64>,
    pub sky_color: Option<f64>,
}
impl PartialEq for HemisphereLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
