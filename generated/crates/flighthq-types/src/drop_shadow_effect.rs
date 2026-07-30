// @generated from upstream/packages/types/src/DropShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EffectSourceMode, Kind};

// Source: upstream/packages/types/src/DropShadowEffect.ts:7 (sha256:6848511a980718c7082335e4839bb93de5a772a3a8714f0f1a720796bc2ca393)
#[derive(Clone, Default)]
pub struct DropShadowEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub alpha: Option<f64>,
    pub angle: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub distance: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for DropShadowEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
