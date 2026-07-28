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
    pub background_color: Option<Option<f64>>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
}
