// @generated from upstream/packages/types/src/CanvasRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Matrix, SceneGraphSyncPolicy};

// Source: upstream/packages/types/src/CanvasRenderOptions.ts:4 (sha256:11d2b4fc4d995793ae8fce447563e9b4d4eef417364ba3290f680e23dce49c59)
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
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
}
impl PartialEq for CanvasRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
