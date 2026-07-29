// @generated from upstream/packages/render-wgpu/src/wgpuSurface.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy, Surface,
    WgpuRenderState,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render-wgpu/src/wgpuSurface.ts:16 (sha256:7cddbfb84036f57b1d0cdc6c3541829e8559976951df39b147fcfedc3ffcb4d5)
pub fn acquire_wgpu_frame_capture_texture(
    state: &WgpuRenderState,
) -> Option<crate::OpaqueHostValue> {
    let mut runtime = get_wgpu_render_state_runtime(state);
    if (!runtime.inner.lock().unwrap().frame_capture_enabled) {
        return None;
    }
    let width = (1.0_f64).max(crate::host_value::<crate::OpaqueHostValue>("host.width"));
    let height = (1.0_f64).max(crate::host_value::<crate::OpaqueHostValue>("host.height"));
    let existing = (runtime.inner.lock().unwrap().frame_capture_texture).clone();
    if ((((existing).is_some()) && ((existing).is_some()))
        && (crate::host_value::<crate::OpaqueHostValue>("host.width") == width))
        && (crate::host_value::<crate::OpaqueHostValue>("host.height") == height)
    {
        return (existing).clone();
    }
    crate::host_value::<()>("host.destroy");
    let texture = crate::host_value::<()>("host.createTexture");
    {
        let __flight_runtime = runtime;
        let __flight_value = Some(texture);
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.frame_capture_texture = __flight_value;
    };
    return Some(texture);
}

// Source: upstream/packages/render-wgpu/src/wgpuSurface.ts:42 (sha256:e104a245694e6ad30130cf83e51abcdd5ae2b9cee9c725396f948fc6b64ca994)
pub fn create_surface_from_wgpu_render_state(state: &WgpuRenderState) -> crate::Promise<Surface> {
    Default::default()
}

// Source: upstream/packages/render-wgpu/src/wgpuSurface.ts:90 (sha256:7250fcdbaa45e879574899cafeea8b5ba2b24c20c02e4dc18dcc1a2587ff00ca)
pub fn enable_wgpu_frame_capture(state: &WgpuRenderState) -> () {
    {
        let __flight_runtime = get_wgpu_render_state_runtime(state);
        let __flight_value = true;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.frame_capture_enabled = __flight_value;
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuSurface.ts:97 (sha256:15a80eaf205762a0e834cb8ff84283edae2409bc24e76db67d9db572e6588f0e)
pub fn encode_wgpu_frame_capture(state: &WgpuRenderState, encoder: crate::OpaqueHostValue) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    let texture = (runtime.inner.lock().unwrap().frame_capture_texture).clone();
    if ((!runtime.inner.lock().unwrap().frame_capture_enabled) || ((texture).is_none()))
        || ((texture).is_none())
    {
        return;
    }
    let width = crate::host_value::<crate::OpaqueHostValue>("host.width");
    let height = crate::host_value::<crate::OpaqueHostValue>("host.height");
    let bytes_per_row = (((width * 4.0_f64) / 256.0_f64).ceil() * 256.0_f64);
    if (((((runtime.inner.lock().unwrap().frame_capture_buffer).clone()).is_none())
        || (((runtime.inner.lock().unwrap().frame_capture_buffer).clone()).is_none()))
        || (runtime.inner.lock().unwrap().frame_capture_width != width))
        || (runtime.inner.lock().unwrap().frame_capture_height != height)
    {
        crate::host_value::<()>("host.destroy");
        {
            let __flight_runtime = runtime;
            let __flight_value = Some(crate::host_value::<crate::OpaqueHostValue>(
                "host.createBuffer",
            ));
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.frame_capture_buffer = __flight_value;
        };
        {
            let __flight_runtime = runtime;
            let __flight_value = bytes_per_row;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.frame_capture_bytes_per_row = __flight_value;
        };
        {
            let __flight_runtime = runtime;
            let __flight_value = width;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.frame_capture_width = __flight_value;
        };
        {
            let __flight_runtime = runtime;
            let __flight_value = height;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.frame_capture_height = __flight_value;
        };
    }
    crate::host_value::<()>("host.copyTextureToBuffer");
}
