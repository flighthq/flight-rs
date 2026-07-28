// @generated from upstream/packages/types/src/BarrelDistortionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BarrelDistortionEffect.ts:2 (sha256:9e117efd63fa2fad7c82cce1b3e314dfbd79509135df5a472aeab88153e8b802)
#[derive(Clone)]
pub struct BarrelDistortionEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub amount: Option<f64>,
    pub scale: Option<f64>,
}
impl PartialEq for BarrelDistortionEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
