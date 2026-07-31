// @generated from upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WGPU_MESH_PRELUDE_WGSL as wgpu_mesh_prelude_wgsl_constant, WgpuMaterialBinding,
    create_wgpu_mesh_pipeline, ensure_wgpu_scene_pipeline, get_wgpu_scene_runtime,
    stash_wgpu_uv_transform,
};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, ImageResource, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Option<Vector2>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2>,
    pub color_space: Option<TextureColorSpace>,
    pub image: Option<ImageResource>,
    pub resource: Option<SceneResourceRef>,
    pub sampler: Option<Sampler>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:39 (sha256:b1a085782a24ad6c51ca8ed571649c0ca66678fd2b8c7269042a832ee8f19386)
#[derive(Clone, Default)]
pub struct WgpuDebugDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_normal_map: bool,
    pub mode: String,
}
impl PartialEq for WgpuDebugDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:45 (sha256:9f44488e7bfb03ef89ea229771bbef597d62294f069ffcab6bacc1edff7e2c11)
#[derive(Clone, Default)]
pub struct WgpuDebugPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuDebugPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:53 (sha256:e2a2ad9f7f66e4624d7d5f1b46e53be4a763033badc24120ccfa7aac48b396c2)
pub fn bind_wgpu_debug_surface(
    state: &mut WgpuRenderState,
    pipeline: &WgpuDebugPipeline,
    material_key: crate::OpaqueHostValue,
    near: f64,
    far: f64,
    normal_scale: f64,
) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    let mut binding: Option<WgpuMaterialBinding> = scene
        .material_bind_groups
        .iter()
        .find(|(key, _)| key == &(material_key).clone())
        .map(|(_, value)| value.clone());
    if (binding).is_none() {
        let state_runtime = get_wgpu_render_state_runtime(state);
        let buffer = crate::host_value::<()>("host.createBuffer");
        let bind_group = crate::host_value::<()>("host.createBindGroup");
        binding = Some(WgpuMaterialBinding {
            __flight_identity: std::sync::Arc::new(()),
            bind_group: bind_group,
            buffer: buffer,
        });
        {
            let __flight_key = (material_key).clone();
            let __flight_value = (binding).clone().unwrap();
            if let Some((_, value)) = scene
                .material_bind_groups
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                scene
                    .material_bind_groups
                    .push((__flight_key, __flight_value));
            }
        };
    }
    (*_SCRATCH.lock().unwrap())[0.0_f64 as usize] = (near) as f32;
    (*_SCRATCH.lock().unwrap())[1.0_f64 as usize] = (far) as f32;
    (*_SCRATCH.lock().unwrap())[2.0_f64 as usize] = (normal_scale) as f32;
    (*_SCRATCH.lock().unwrap())[3.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    stash_wgpu_uv_transform(state, None);
    return (binding.as_mut().unwrap().bind_group).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:95 (sha256:87144f3b5b5c4600bf6c03a20230c1c26c35849bb0c50d99ac9d8080ee61c6ce)
pub fn build_wgpu_debug_define_key(key: &WgpuDebugDefineKey) -> String {
    return format!(
        "{}{}",
        if ((key.mode).clone() == "depth") {
            "d".to_owned()
        } else {
            "n".to_owned()
        },
        if key.has_normal_map {
            "m".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:103 (sha256:92180fcb6bcee0de36248b39a069081474b9291b04ea78be341661a3e56ed5bb)
#[derive(Clone, Default)]
struct CompileWgpuDebugPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CompileWgpuDebugPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CompileWgpuDebugPipelineRecord6 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for CompileWgpuDebugPipelineRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct OptionsContextRecord7 {
    __flight_identity: std::sync::Arc<()>,
    double_sided: bool,
    format: crate::OpaqueHostValue,
    ibl_bind_group_layout: Option<crate::OpaqueHostValue>,
    material_bind_group_layout: crate::OpaqueHostValue,
    module: crate::OpaqueHostValue,
    pbr_sample_bind_group_layout: Option<crate::OpaqueHostValue>,
    shadow_bind_group_layout: Option<crate::OpaqueHostValue>,
    topology: Option<crate::OpaqueHostValue>,
}
impl PartialEq for OptionsContextRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn compile_wgpu_debug_pipeline(
    state: &mut WgpuRenderState,
    key: &WgpuDebugDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuDebugPipeline {
    let device = (state.device).clone();
    let module = crate::host_value::<()>("host.createShaderModule");
    let material_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    return {
        let __flight_source = &(create_wgpu_mesh_pipeline(
            state,
            &OptionsContextRecord7 {
                __flight_identity: std::sync::Arc::new(()),
                double_sided: false,
                format: (format).clone(),
                material_bind_group_layout: (material_bind_group_layout).clone(),
                module: (module).clone(),
                ibl_bind_group_layout: None,
                pbr_sample_bind_group_layout: None,
                shadow_bind_group_layout: None,
                topology: None,
            },
        ));
        WgpuDebugPipeline {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            has_ibl_group: __flight_source.has_ibl_group,
            has_pbr_sample_group: __flight_source.has_pbr_sample_group,
            has_shadow_group: __flight_source.has_shadow_group,
            material_bind_group_layout: (__flight_source.material_bind_group_layout).clone(),
            pipeline: (__flight_source.pipeline).clone(),
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:125 (sha256:d0687469aeb0e3659fd5e01549ae5dbfcefacbc456dce8f096660e073d591f95)
pub fn ensure_wgpu_debug_pipeline(
    mut state: WgpuRenderState,
    key: WgpuDebugDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuDebugPipeline {
    return {
        let __flight_argument_2 = (std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let key = key.clone();
            let mut state = state.clone();
            move || -> f64 { compile_wgpu_debug_pipeline(&mut state, &key, (format).clone()) }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)))
        .clone();
        ensure_wgpu_scene_pipeline(
            &mut state,
            format!("debug:{}|{}", format, build_wgpu_debug_define_key(&key)),
            &__flight_argument_2,
        )
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:138 (sha256:bd1a12f801cf13046b5a0d87b69c2e0a28f431cc9a370ec9317a1f5ed3045424)
pub fn get_wgpu_debug_module_source_for_key(key: &WgpuDebugDefineKey) -> String {
    return ((((format!(
        "const MODE : i32 = {};\n",
        if ((key.mode).clone() == "depth") {
            "DEPTH_MODE".to_owned()
        } else {
            "NORMAL_MODE".to_owned()
        }
    ) + format!(
        "const HAS_NORMAL_MAP : bool = {};\n",
        if key.has_normal_map {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + DEBUG_MODE_CONSTS_WGSL)
        + wgpu_mesh_prelude_wgsl_constant)
        + DEBUG_WGSL_BODY);
}

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:150 (sha256:fe312076a7f617bb0444cc518d03b8c526af7adb23197c789a5afbe9d1074de6)
const DEBUG_UNIFORM_BYTES: f64 = 16.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:154 (sha256:7dd229e2489ee1987c72335b8c1283872e4d5e711cd0b56ad2b509949efaf7e2)
const DEBUG_MODE_CONSTS_WGSL: &'static str =
    "\nconst DEPTH_MODE : i32 = 0;\nconst NORMAL_MODE : i32 = 1;\n";

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:159 (sha256:3257db89eb250d2f6504c49e82b8b1962199786555dac52ae61bdd0a6daf5859)
const DEBUG_WGSL_BODY: &'static str = "\nstruct DebugMaterial {\n  params : vec4f,  // x = near, y = far (depth); z = normalScale (normal)\n};\n\n@group(2) @binding(0) var<uniform> material : DebugMaterial;\n@group(2) @binding(1) var materialSampler : sampler;\n@group(2) @binding(2) var normalTexture : texture_2d<f32>;\n\n@fragment fn fs_main(in : VertexOutput, @builtin(front_facing) frontFacing : bool) -> @location(0) vec4f {\n  if (MODE == DEPTH_MODE) {\n    // Linear view-space distance is the perspective w: in.clipPosition is the @builtin(position), whose\n    // .w in the fragment stage is 1 / w_clip, so 1 / in.clipPosition.w == w_clip == eye distance. This\n    // is camera-agnostic (no camera near/far needed); map it across the material's [near, far]\n    // visualization window to grayscale [0, 1].\n    let near = material.params.x;\n    let far = material.params.y;\n    let eyeDepth = 1.0 / in.clipPosition.w;\n    let d = clamp((eyeDepth - near) / max(far - near, 1e-6), 0.0, 1.0);\n    return vec4f(vec3f(d), 1.0);\n  }\n\n  // NORMAL_MODE: visualize the WORLD-space surface normal — the geometric normal carried through\n  // draw.normalMatrix in vs_main. The normal-map branch is gated by HAS_NORMAL_MAP but stays inert on\n  // wgpu until map upload lands (see the prelude note); normalScale is read so the binding is live.\n  var geometricNormal = normalize(in.worldNormal);\n  if (!frontFacing) {\n    geometricNormal = -geometricNormal;\n  }\n\n  var normal = geometricNormal;\n  if (HAS_NORMAL_MAP) {\n    let tangent = normalize(in.worldTangent.xyz);\n    let bitangent = cross(geometricNormal, tangent) * in.worldTangent.w;\n    var tangentNormal = textureSample(normalTexture, materialSampler, in.uv).xyz * 2.0 - 1.0;\n    tangentNormal = vec3f(tangentNormal.xy * material.params.z, tangentNormal.z);\n    let tbn = mat3x3f(tangent, bitangent, geometricNormal);\n    normal = normalize(tbn * tangentNormal);\n  }\n\n  return vec4f(normal * 0.5 + 0.5, 1.0);\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts:203 (sha256:8e50f70c308103e00a87cbc7b5167463651b8a5ba0d37b20b795f2c3beb871a6)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![0.0_f32; (DEBUG_UNIFORM_BYTES / 4.0_f64) as usize])
});
