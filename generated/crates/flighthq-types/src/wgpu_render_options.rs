// @generated from upstream/packages/types/src/WgpuRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Scene3DGraphSyncPolicy;

// Source: upstream/packages/types/src/WgpuRenderOptions.ts:3 (sha256:cfc1746d136ddf34026e1c7e71cf59ee94dfc90edf8e66824bacfcdd8682dac6)
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
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
}
impl PartialEq for WgpuRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
