// @generated from upstream/packages/types/src/DomRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SceneGraphSyncPolicy;

// Source: upstream/packages/types/src/DomRenderOptions.ts:2 (sha256:1701b24fbfde13aa4e2817401a96890ba1a6f52001f929abc32a1eaf8259cd47)
#[derive(Clone)]
pub struct DomRenderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub background_color: Option<f64>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
}
impl PartialEq for DomRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
