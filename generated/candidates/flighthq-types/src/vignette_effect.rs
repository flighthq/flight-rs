// @generated from upstream/packages/types/src/VignetteEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/VignetteEffect.ts:3 (sha256:f41110d11eecb97849c3d3c836b7c17e79b58e5fa50dc2486f4e58366b4f3fbe)
#[derive(Clone)]
pub struct VignetteEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub intensity: Option<f64>,
    pub radius: Option<f64>,
    pub softness: Option<f64>,
    pub color: Option<f64>,
}
impl PartialEq for VignetteEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
