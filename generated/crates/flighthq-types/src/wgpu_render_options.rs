// @generated from upstream/packages/types/src/WgpuRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{HostCanvasCapability, HostImageCapability, Scene3DGraphSyncPolicy};

// Source: upstream/packages/types/src/WgpuRenderOptions.ts:5 (sha256:bc55f71e56190449f22e72b5c1f786a653fee083c0e58f7a3a8f54b0737836a6)
#[derive(Clone, Default)]
pub struct WgpuRenderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_host: Option<HostCanvasCapability>,
    pub image_host: Option<HostImageCapability>,
    pub format: Option<crate::OpaqueHostValue>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
}
impl PartialEq for WgpuRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
