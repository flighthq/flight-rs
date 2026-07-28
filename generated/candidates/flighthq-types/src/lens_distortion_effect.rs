// @generated from upstream/packages/types/src/LensDistortionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/LensDistortionEffect.ts:3 (sha256:caab57cf7cd9afb95ddc9436e45f88db58557d7699891f9fb93503fccea47650)
#[derive(Clone)]
pub struct LensDistortionEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub amount: Option<f64>,
    pub scale: Option<f64>,
}
impl PartialEq for LensDistortionEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
