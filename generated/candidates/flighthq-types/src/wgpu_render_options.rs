// @generated from upstream/packages/types/src/WgpuRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SceneGraphSyncPolicy;

// Source: upstream/packages/types/src/WgpuRenderOptions.ts:3 (sha256:ea94e5de542002d884aee75a4010cdd86ac2934f0b3d88c2b7202f8228e80c8a)
#[derive(Clone, Default)]
pub struct WgpuRenderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub antialias: Option<bool>,
    pub background_color: Option<f64>,
    pub format: Option<crate::OpaqueHostValue>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub power_preference: Option<crate::OpaqueHostValue>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
}
impl PartialEq for WgpuRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
