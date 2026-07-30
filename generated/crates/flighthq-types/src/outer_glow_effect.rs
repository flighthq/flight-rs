// @generated from upstream/packages/types/src/OuterGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EffectSourceMode, Kind};

// Source: upstream/packages/types/src/OuterGlowEffect.ts:7 (sha256:ec80a0d48ff955f3df6a19bad5643e1e576384f33c4c3ba299bd5e14d2253eff)
#[derive(Clone, Default)]
pub struct OuterGlowEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub alpha: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for OuterGlowEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
