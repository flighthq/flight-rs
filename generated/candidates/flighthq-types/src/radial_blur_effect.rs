// @generated from upstream/packages/types/src/RadialBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RadialBlurEffect.ts:3 (sha256:22f272013c3655070dc964707dbebbc4b5eb6699cfe0e632dba09392544d5981)
#[derive(Clone)]
pub struct RadialBlurEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub center_x: Option<f64>,
    pub center_y: Option<f64>,
    pub strength: Option<f64>,
    pub samples: Option<f64>,
}
impl PartialEq for RadialBlurEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
