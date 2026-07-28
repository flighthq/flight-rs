// @generated from upstream/packages/types/src/GlRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SceneGraphSyncPolicy;

// Source: upstream/packages/types/src/GlRenderOptions.ts:3 (sha256:7f50935c919761fcfd91bcdfd7b7f034bd1aed9b68274069b5cd5b0ce2fe4c0e)
#[derive(Clone)]
pub struct GlRenderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub antialias: Option<bool>,
    pub background_color: Option<f64>,
    pub context_attributes: Option<crate::OpaqueHostValue>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub power_preference: Option<crate::OpaqueHostValue>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
}
impl PartialEq for GlRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
