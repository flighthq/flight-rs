// @generated from upstream/packages/types/src/CanvasRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Matrix, Scene3DGraphSyncPolicy};

// Source: upstream/packages/types/src/CanvasRenderOptions.ts:4 (sha256:742d7acb3aa90e8039a21854757bd4536423f1f7d0cf44236befccaad70af785)
#[derive(Clone, Default)]
pub struct CanvasRenderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub background_color: Option<f64>,
    pub context_attributes: Option<crate::OpaqueHostValue>,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub pixel_ratio: Option<f64>,
    pub render_transform: Option<Matrix>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
}
impl PartialEq for CanvasRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
