// @generated from upstream/packages/render-wgpu/src/wgpuBackground.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    acquire_wgpu_frame_capture_texture, encode_wgpu_frame_capture, get_wgpu_render_state_runtime,
};
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy,
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

// Source: upstream/packages/render-wgpu/src/wgpuBackground.ts:6 (sha256:b2278ad93210c0f048874d0687dba1556278f445333ca14d30be39c7aa1398e5)
fn ensure_wgpu_depth_stencil(state: &WgpuRenderState, width: f64, height: f64) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    if ((((runtime.inner.lock().unwrap().depth_stencil_texture).clone()).is_some())
        && (runtime.inner.lock().unwrap().depth_stencil_width == width))
        && (runtime.inner.lock().unwrap().depth_stencil_height == height)
    {
        return;
    }
    crate::host_value::<()>("host.destroy");
    let texture = crate::host_value::<()>("host.createTexture");
    {
        let __flight_runtime = runtime;
        let __flight_value = Some(texture);
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.depth_stencil_texture = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((texture.create_view)());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.depth_stencil_view = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = width;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.depth_stencil_width = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = height;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.depth_stencil_height = __flight_value;
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuBackground.ts:30 (sha256:46c0da140682aa3f0151b9a4a9ce4f96ae3dd7529eabab67c2de92437202defe)
#[derive(Clone, Default)]
struct RenderWgpuBackgroundRecord3 {
    __flight_identity: std::sync::Arc<()>,
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}
impl PartialEq for RenderWgpuBackgroundRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct RenderWgpuBackgroundSynthesizedRecord1596336968 {
    __flight_identity: std::sync::Arc<()>,
    a: f64,
    b: f64,
    g: f64,
    r: f64,
}
impl PartialEq for RenderWgpuBackgroundSynthesizedRecord1596336968 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn render_wgpu_background(state: &WgpuRenderState) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    if ((runtime.inner.lock().unwrap().render_pass).clone()).is_some() {
        crate::host_value::<()>("host.end");
        {
            let __flight_runtime = runtime;
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.render_pass = __flight_value;
        };
    }
    {
        let __flight_runtime = runtime;
        let __flight_value = 0.0_f64;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.uniform_offset = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = 0.0_f64;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.sprite_batch_buffer_cursor = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_blend_mode = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = 0.0_f64;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_mask_depth = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = false;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.mask_write_mode = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .current_scissor_rect = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = vec![];
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.wgpu_render_state_runtime.scissor_stack = __flight_value;
    };
    let device = (state.device).clone();
    let canvas = (state.canvas).clone();
    let context = (state.context).clone();
    let width = crate::host_value::<crate::OpaqueHostValue>("host.width");
    let height = crate::host_value::<crate::OpaqueHostValue>("host.height");
    ensure_wgpu_depth_stencil(state, (width).clone(), (height).clone());
    let canvas_texture =
        (acquire_wgpu_frame_capture_texture(state)).unwrap_or(crate::host_value::<
            crate::OpaqueHostValue,
        >("host.getCurrentTexture"));
    let canvas_view = crate::host_value::<()>("host.createView");
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((canvas_view).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.canvas_texture_view = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = true;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.canvas_view_cleared = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((state.format).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.current_color_format = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage
            .wgpu_render_state_runtime
            .render_target_viewport = __flight_value;
    };
    let clear_value = if ((state.background_color_rgba.len() as f64) >= 4.0_f64)
        && (state.background_color_rgba[3.0_f64 as usize].clone() > 0.0_f64)
    {
        RenderWgpuBackgroundSynthesizedRecord1596336968 {
            __flight_identity: std::sync::Arc::new(()),
            r: state.background_color_rgba[0.0_f64 as usize].clone(),
            g: state.background_color_rgba[1.0_f64 as usize].clone(),
            b: state.background_color_rgba[2.0_f64 as usize].clone(),
            a: state.background_color_rgba[3.0_f64 as usize].clone(),
        }
    } else {
        RenderWgpuBackgroundSynthesizedRecord1596336968 {
            __flight_identity: std::sync::Arc::new(()),
            r: 0.0_f64,
            g: 0.0_f64,
            b: 0.0_f64,
            a: 0.0_f64,
        }
    };
    let command_encoder = crate::host_value::<()>("host.createCommandEncoder");
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((command_encoder).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.command_encoder = __flight_value;
    };
    let render_pass = crate::host_value::<()>("host.beginRenderPass");
    crate::host_value::<()>("host.setViewport");
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((render_pass).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.render_pass = __flight_value;
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuBackground.ts:99 (sha256:e1844970c8d5b0d18f67e244eac4118e8f2346844274a095df3dd7e3bd9ab233)
pub fn submit_wgpu_render_pass(state: &WgpuRenderState) -> () {
    let mut runtime = get_wgpu_render_state_runtime(state);
    let render_pass = (runtime.inner.lock().unwrap().render_pass).clone();
    let command_encoder = (runtime.inner.lock().unwrap().command_encoder).clone();
    let uniform_buffer = (runtime.inner.lock().unwrap().uniform_buffer).clone();
    let uniform_offset = runtime.inner.lock().unwrap().uniform_offset;
    let device = (state.device).clone();
    if (render_pass).is_some() {
        crate::host_value::<()>("host.end");
        {
            let __flight_runtime = runtime;
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.render_pass = __flight_value;
        };
    }
    if (command_encoder).is_some() {
        if (uniform_offset > 0.0_f64) {
            crate::host_value::<()>("host.writeBuffer");
        }
        encode_wgpu_frame_capture(state, (command_encoder.as_ref().unwrap()).clone());
        crate::host_value::<()>("host.submit");
        {
            let __flight_runtime = runtime;
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.command_encoder = __flight_value;
        };
        let mut retired = (runtime.inner.lock().unwrap().retired_buffers).clone();
        if ((retired).is_some()) && ((retired.as_ref().unwrap().len() as f64) > 0.0_f64) {
            {
                let mut i = 0.0_f64;
                while (i < (retired.as_ref().unwrap().len() as f64)) {
                    crate::host_value::<()>("host.destroy");
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            retired.as_mut().unwrap().length = 0.0_f64;
        }
    }
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.canvas_texture_view = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = false;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.canvas_view_cleared = __flight_value;
    };
}
