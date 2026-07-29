// @generated from upstream/packages/render-wgpu/src/wgpuMipmap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_render_state_runtime;
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

// Source: upstream/packages/render-wgpu/src/wgpuMipmap.ts:10 (sha256:a4972f59cb05a989f441ed3fbb081e26355428cba4ec62df1ccd82fea4aff742)
#[derive(Clone, Default)]
struct GenerateWgpuMipmapsRecord3 {
    __flight_identity: std::sync::Arc<()>,
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}
impl PartialEq for GenerateWgpuMipmapsRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn generate_wgpu_mipmaps(
    state: &WgpuRenderState,
    texture: crate::OpaqueHostValue,
    width: f64,
    height: f64,
    format: crate::OpaqueHostValue,
) -> () {
    let level_count = get_wgpu_mip_level_count(width, height);
    if (level_count <= 1.0_f64) {
        return;
    }
    let device = (state.device).clone();
    let runtime = get_wgpu_render_state_runtime(state);
    let pipeline = ensure_wgpu_mipmap_pipeline(state, (format).clone());
    let layout = ((runtime.inner.lock().unwrap().mipmap_bind_group_layout).clone()).unwrap();
    let encoder = crate::host_value::<()>("host.createCommandEncoder");
    {
        let mut level = 1.0_f64;
        while (level < level_count) {
            let src_view = crate::host_value::<()>("host.createView");
            let mut dst_view = crate::host_value::<()>("host.createView");
            let bind_group = crate::host_value::<()>("host.createBindGroup");
            let pass = crate::host_value::<()>("host.beginRenderPass");
            crate::host_value::<()>("host.setPipeline");
            crate::host_value::<()>("host.setBindGroup");
            crate::host_value::<()>("host.draw");
            crate::host_value::<()>("host.end");
            {
                dst_view.base_mip_level += 1.0;
                dst_view.base_mip_level
            };
        }
    }
    crate::host_value::<()>("host.submit");
}

// Source: upstream/packages/render-wgpu/src/wgpuMipmap.ts:47 (sha256:92badde5b973fc60810e25826dc0335ebff822df2953528c8f058f97c4a559ea)
pub fn get_wgpu_mip_level_count(width: f64, height: f64) -> f64 {
    return (1.0_f64 + ((((1.0_f64).max(width)).max(height)).log2()).floor());
}

// Source: upstream/packages/render-wgpu/src/wgpuMipmap.ts:55 (sha256:e048353661d25d9bf98d6385302212166bc79e367e525044663808d065d57c15)
#[derive(Clone, Default)]
struct EnsureWgpuMipmapPipelineRecord3 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for EnsureWgpuMipmapPipelineRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuMipmapPipelineRecord4 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureWgpuMipmapPipelineRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuMipmapPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    topology: String,
}
impl PartialEq for EnsureWgpuMipmapPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_wgpu_mipmap_pipeline(
    state: &WgpuRenderState,
    format: crate::OpaqueHostValue,
) -> crate::OpaqueHostValue {
    let mut runtime = get_wgpu_render_state_runtime(state);
    if ((runtime.inner.lock().unwrap().mipmap_pipeline).clone()).is_some() {
        return ((runtime.inner.lock().unwrap().mipmap_pipeline).clone()).unwrap();
    }
    let device = (state.device).clone();
    let bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let module = crate::host_value::<()>("host.createShaderModule");
    let pipeline = crate::host_value::<()>("host.createRenderPipeline");
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((bind_group_layout).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.mipmap_bind_group_layout = __flight_value;
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = Some((pipeline).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.mipmap_pipeline = __flight_value;
    };
    return pipeline;
}

// Source: upstream/packages/render-wgpu/src/wgpuMipmap.ts:77 (sha256:9c0bafb6d7d598fcbb447a4ca8e532d0abb29a26401a466e65f4034d51390c5f)
const MIPMAP_WGSL: &'static str = "\nstruct VsOut {\n  @builtin(position) pos : vec4f,\n  @location(0) uv : vec2f,\n}\n\n@vertex\nfn vs_main(@builtin(vertex_index) vi : u32) -> VsOut {\n  // Full-screen triangle covering the clip rect; uv in [0,1] with v flipped into texture space.\n  let x = f32((vi & 1u) << 2u) - 1.0;\n  let y = f32((vi & 2u) << 1u) - 1.0;\n  var out : VsOut;\n  out.pos = vec4f(x, y, 0.0, 1.0);\n  out.uv = vec2f((x + 1.0) * 0.5, (1.0 - y) * 0.5);\n  return out;\n}\n\n@group(0) @binding(0) var srcTexture : texture_2d<f32>;\n@group(0) @binding(1) var srcSampler : sampler;\n\n@fragment\nfn fs_main(@location(0) uv : vec2f) -> @location(0) vec4f {\n  return textureSample(srcTexture, srcSampler, uv);\n}\n";
