// @generated from upstream/packages/types/src/BevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EffectSourceMode;

// Source: upstream/packages/types/src/BevelEffect.ts:7 (sha256:58ebca8ad2f0cc535020211940a5e2321e01db30093d6a5988a44efb977cdd04)
#[derive(Clone)]
pub struct BevelEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub angle: Option<f64>,
    pub bevel_type: Option<String>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub distance: Option<f64>,
    pub highlight_alpha: Option<f64>,
    pub highlight_color: Option<f64>,
    pub quality: Option<f64>,
    pub shadow_alpha: Option<f64>,
    pub shadow_color: Option<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
impl PartialEq for BevelEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
