// @generated from upstream/packages/types/src/InnerGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{InnerEffectSourceMode, Kind};

// Source: upstream/packages/types/src/InnerGlowEffect.ts:7 (sha256:8301e42403f279dd1544f2cfcd45fc89ac70c49225e2760882b1c6511f02657c)
#[derive(Clone, Default)]
pub struct InnerGlowEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub alpha: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<InnerEffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for InnerGlowEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
