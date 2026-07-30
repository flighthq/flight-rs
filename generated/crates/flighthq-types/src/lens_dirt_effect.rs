// @generated from upstream/packages/types/src/LensDirtEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/LensDirtEffect.ts:5 (sha256:e7bd18362130d985500ce1df2f7602792550595aa8ca8c015abc295b26b00ed7)
#[derive(Clone, Default)]
pub struct LensDirtEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub intensity: Option<f64>,
    pub threshold: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for LensDirtEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
