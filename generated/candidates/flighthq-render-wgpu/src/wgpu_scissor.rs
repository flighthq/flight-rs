// @generated from upstream/packages/render-wgpu/src/wgpuScissor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy,
    WgpuRenderState, WgpuScissorRect,
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

// Source: upstream/packages/render-wgpu/src/wgpuScissor.ts:8 (sha256:5b08aa5b601255a705253a163749042b4f74d43d75e9d29a712e185f19d28c69)
pub fn apply_wgpu_scissor_rect(state: &WgpuRenderState, pass: crate::OpaqueHostValue) -> () {
    let runtime = get_wgpu_render_state_runtime(state);
    let rect = (runtime
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .current_scissor_rect)
        .clone();
    if (rect).is_none() {
        return;
    }
    let x = (0.0_f64).max((rect.as_ref().unwrap().x).floor());
    let y = (0.0_f64).max((rect.as_ref().unwrap().y).floor());
    let w = (1.0_f64).max((rect.as_ref().unwrap().width).ceil());
    let h = (1.0_f64).max((rect.as_ref().unwrap().height).ceil());
    crate::host_value::<()>("host.setScissorRect");
}

// Source: upstream/packages/render-wgpu/src/wgpuScissor.ts:24 (sha256:395df6103a88558bb9d7a7e398923e945feb60f39a011eac91300deb212bfa91)
pub fn pop_wgpu_scissor_rect(state: &WgpuRenderState) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    let prev = runtime
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .scissor_stack
        .pop()
        .expect("TypeScript Array.pop returned undefined");
    {
        let __flight_runtime = runtime;
        let __flight_value = prev;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .current_scissor_rect = __flight_value;
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuScissor.ts:34 (sha256:61b6a0f4a5197a962249e68a22544326341d208148de23dfd33182b68645d0cc)
pub fn push_wgpu_scissor_rect(state: &WgpuRenderState, rect: &WgpuScissorRect) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    if ((runtime
        .inner
        .lock()
        .unwrap()
        .wgpu_render_state_runtime
        .current_scissor_rect)
        .clone())
    .is_some()
    {
        runtime
            .inner
            .lock()
            .unwrap()
            .wgpu_render_state_runtime
            .scissor_stack
            .push(
                ((runtime
                    .inner
                    .lock()
                    .unwrap()
                    .wgpu_render_state_runtime
                    .current_scissor_rect)
                    .clone())
                .unwrap(),
            );
    }
    {
        let __flight_runtime = runtime;
        let __flight_value = Some(WgpuScissorRect {
            __flight_identity: std::sync::Arc::new(()),
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        });
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .current_scissor_rect = __flight_value;
    };
}
