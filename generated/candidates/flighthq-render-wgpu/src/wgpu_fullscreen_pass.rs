// @generated from upstream/packages/render-wgpu/src/wgpuFullscreenPass.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Matrix, SceneGraphSyncPolicy,
    WgpuFullscreenPipeline, WgpuRenderState, WgpuRenderTarget,
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

// Source: upstream/packages/render-wgpu/src/wgpuFullscreenPass.ts:10 (sha256:6bec83e513152861e37bdfdbb1ebde99dfc59a03540ca2f1ab80a879251ed94d)
#[derive(Clone, Default)]
struct CreateWgpuFullscreenPipelineRecord3 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CreateWgpuFullscreenPipelineRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateWgpuFullscreenPipelineRecord4 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for CreateWgpuFullscreenPipelineRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CreateWgpuFullscreenPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    topology: String,
}
impl PartialEq for CreateWgpuFullscreenPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_wgpu_fullscreen_pipeline(
    state: &WgpuRenderState,
    fragment_wgsl: String,
    texture_input_count: Option<f64>,
    format: Option<crate::OpaqueHostValue>,
) -> WgpuFullscreenPipeline {
    let texture_input_count = texture_input_count.unwrap_or(1.0_f64);
    let format = format.unwrap_or((state.format).clone());
    let device = (state.device).clone();
    let uniform_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let mut texture_bind_group_layouts: Vec<crate::OpaqueHostValue> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < texture_input_count) {
            texture_bind_group_layouts.push(crate::host_value::<crate::OpaqueHostValue>(
                "host.createBindGroupLayout",
            ));
            {
                i += 1.0;
                i
            };
        }
    }
    let pipeline_layout = crate::host_value::<()>("host.createPipelineLayout");
    let vs_module = crate::host_value::<()>("host.createShaderModule");
    let fs_module = crate::host_value::<()>("host.createShaderModule");
    let pipeline = crate::host_value::<()>("host.createRenderPipeline");
    return WgpuFullscreenPipeline {
        __flight_identity: std::sync::Arc::new(()),
        pipeline: (pipeline).clone(),
        pipeline_layout: (pipeline_layout).clone(),
        uniform_bind_group_layout: (uniform_bind_group_layout).clone(),
        texture_bind_group_layouts: (texture_bind_group_layouts).clone(),
    };
}

// Source: upstream/packages/render-wgpu/src/wgpuFullscreenPass.ts:58 (sha256:a3554ce3dc1d5fb633eb8a26bd866abc41f072289cbbd5249a6242ea5473b36f)
pub fn destroy_wgpu_fullscreen_pipeline(
    _state: &WgpuRenderState,
    _pipeline: &WgpuFullscreenPipeline,
) -> () {
}

// Source: upstream/packages/render-wgpu/src/wgpuFullscreenPass.ts:70 (sha256:6c7af7a99a2a012b5c7006e9c2e6d74526a44d18e5b32c580ff8b697d0f9b866)
pub fn draw_wgpu_fullscreen_pass(
    state: &WgpuRenderState,
    wgpu_pipeline: &WgpuFullscreenPipeline,
    inputs: &Vec<WgpuRenderTarget>,
    dest: Option<WgpuRenderTarget>,
    set_uniforms: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(WgpuRenderState, crate::OpaqueHostValue) -> crate::OpaqueHostValue
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
) -> () {
    let runtime = get_wgpu_render_state_runtime(state);
    let pass = if (dest).is_some() {
        (runtime.inner.lock().unwrap().render_pass).clone()
    } else {
        (runtime.inner.lock().unwrap().render_pass).clone()
    };
    if (pass).is_none() {
        return;
    }
    crate::host_value::<()>("host.setPipeline");
    if (set_uniforms).is_some() {
        let uniform_bind_group = {
            let __flight_callback = (set_uniforms.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                (*state).clone(),
                (wgpu_pipeline.uniform_bind_group_layout).clone(),
            );
            __flight_result
        };
        crate::host_value::<()>("host.setBindGroup");
    }
    let runtime2 = get_wgpu_render_state_runtime(state);
    {
        let mut i = 0.0_f64;
        while (i < (inputs.len() as f64)) {
            let input = inputs[i as usize].clone();
            let layout = wgpu_pipeline.texture_bind_group_layouts[i as usize].clone();
            if (layout).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let sampler = if state.allow_smoothing {
                (runtime2.inner.lock().unwrap().linear_sampler).clone()
            } else {
                (runtime2.inner.lock().unwrap().nearest_sampler).clone()
            };
            let bind_group = crate::host_value::<()>("host.createBindGroup");
            crate::host_value::<()>("host.setBindGroup");
            {
                i += 1.0;
                i
            };
        }
    }
    crate::host_value::<()>("host.draw");
}

// Source: upstream/packages/render-wgpu/src/wgpuFullscreenPass.ts:114 (sha256:ed8694df8c14ba7847290a33108eeb2ba286cc23e787507b4c54d8a6d6b1c512)
const FULLSCREEN_VERTEX_WGSL: &'static str = "\n@vertex\nfn vs_main(@builtin(vertex_index) vi : u32) -> @builtin(position) vec4f {\n  // Full-screen triangle: three vertices covering the clip rect.\n  let x = f32((vi & 1u) << 2u) - 1.0;\n  let y = f32((vi & 2u) << 1u) - 1.0;\n  return vec4f(x, y, 0.0, 1.0);\n}\n";
