// @generated from upstream/packages/types/src/SpotLightConeAngles.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SpotLightConeAngles.ts:1 (sha256:568f8b40451fc552029c1d925406e49260d65cb59925a33e673446d908ae2c3a)
#[derive(Clone, Default)]
pub struct SpotLightConeAngles {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub inner_degrees: f64,
    pub outer_degrees: f64,
}
impl PartialEq for SpotLightConeAngles {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
