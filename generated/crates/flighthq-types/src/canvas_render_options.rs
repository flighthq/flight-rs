// @generated from upstream/packages/types/src/CanvasRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Scene3DGraphSyncPolicy;

// Source: upstream/packages/types/src/CanvasRenderOptions.ts:3 (sha256:a8e989d6ec1a7c47845cf0d968f588a532f4f116dc748ae38b1507cd4e2e0fe1)
#[derive(Clone, Default)]
pub struct CanvasRenderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
}
impl PartialEq for CanvasRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
