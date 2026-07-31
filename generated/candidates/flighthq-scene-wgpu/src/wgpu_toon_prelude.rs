// @generated from upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WGPU_MESH_PRELUDE_WGSL as wgpu_mesh_prelude_wgsl_constant, WgpuMaterialBinding,
    create_wgpu_mesh_pipeline, ensure_wgpu_placeholder_texture_view, ensure_wgpu_scene_pipeline,
    ensureWgpuShadowSampleLayout, get_wgpu_scene_runtime, stash_wgpu_uv_transform,
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

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:48 (sha256:f398168e5cf1af145fc5549817da699d6921686ffdae42534676bf3609d37599)
#[derive(Clone, Default)]
pub struct WgpuToonDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_base_color_map: bool,
    pub has_ramp: bool,
}
impl PartialEq for WgpuToonDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:56 (sha256:f181dc70570bd7dbff5c2288f5993da0a58149a85f37b24bbf1fd56b7c80910b)
#[derive(Clone, Default)]
pub struct WgpuToonPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuToonPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:63 (sha256:5d0e0934f694c440b61ec636f927276b1e3898c0db03cda36dab7de2977c030f)
pub fn bind_wgpu_toon_surface(
    state: &mut WgpuRenderState,
    pipeline: &WgpuToonPipeline,
    material_key: crate::OpaqueHostValue,
    base_color: &Vec<f64>,
    steps: f64,
    alpha_cutoff: f64,
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
        let placeholder = ensure_wgpu_placeholder_texture_view(state);
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
    (*_SCRATCH.lock().unwrap())[0.0_f64 as usize] = (base_color[0.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[1.0_f64 as usize] = (base_color[1.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[2.0_f64 as usize] = (base_color[2.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[3.0_f64 as usize] = (base_color[3.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[4.0_f64 as usize] = (steps) as f32;
    (*_SCRATCH.lock().unwrap())[5.0_f64 as usize] = (alpha_cutoff) as f32;
    (*_SCRATCH.lock().unwrap())[6.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH.lock().unwrap())[7.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    stash_wgpu_uv_transform(state, None);
    return (binding.as_mut().unwrap().bind_group).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:110 (sha256:0b4449422ef45ed658105cb8a5e3ec5d3c78446047d52f2e3fff09c2f823b10b)
pub fn build_wgpu_toon_define_key(key: &WgpuToonDefineKey) -> String {
    return format!(
        "{}{}{}{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.double_sided {
            "d".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_base_color_map {
            "b".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_ramp {
            "r".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:119 (sha256:fbead217343c91bfa0034133f4bddfcc60779eb1c09fab4771f357c17eef86d5)
#[derive(Clone, Default)]
struct CompileWgpuToonPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CompileWgpuToonPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CompileWgpuToonPipelineRecord6 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for CompileWgpuToonPipelineRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CompileWgpuToonPipelineSynthesizedRecord1880144846 {
    __flight_identity: std::sync::Arc<()>,
    double_sided: bool,
    format: crate::OpaqueHostValue,
    material_bind_group_layout: crate::OpaqueHostValue,
    module: crate::OpaqueHostValue,
    shadow_bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for CompileWgpuToonPipelineSynthesizedRecord1880144846 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn compile_wgpu_toon_pipeline(
    state: &mut WgpuRenderState,
    key: &WgpuToonDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuToonPipeline {
    let device = (state.device).clone();
    let module = crate::host_value::<()>("host.createShaderModule");
    let material_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    return {
        let __flight_source = &({
            let __flight_argument_1 = (CompileWgpuToonPipelineSynthesizedRecord1880144846 {
                __flight_identity: std::sync::Arc::new(()),
                double_sided: key.double_sided,
                format: (format).clone(),
                material_bind_group_layout: (material_bind_group_layout).clone(),
                module: (module).clone(),
                shadow_bind_group_layout: ensure_wgpu_shadow_sample_layout(state),
            })
            .clone();
            create_wgpu_mesh_pipeline(state, &__flight_argument_1)
        });
        WgpuToonPipeline {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            has_ibl_group: __flight_source.has_ibl_group,
            has_pbr_sample_group: __flight_source.has_pbr_sample_group,
            has_shadow_group: __flight_source.has_shadow_group,
            material_bind_group_layout: (__flight_source.material_bind_group_layout).clone(),
            pipeline: (__flight_source.pipeline).clone(),
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:147 (sha256:ddf1a14c8c96e5fa3f1c4a0782950d81f92ccfeb9b251a30045fc6757602e341)
pub fn ensure_wgpu_toon_pipeline(
    mut state: WgpuRenderState,
    key: WgpuToonDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuToonPipeline {
    return {
        let __flight_argument_2 = (std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let key = key.clone();
            let mut state = state.clone();
            move || -> f64 { compile_wgpu_toon_pipeline(&mut state, &key, (format).clone()) }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)))
        .clone();
        ensure_wgpu_scene_pipeline(
            &mut state,
            format!("toon:{}|{}", format, build_wgpu_toon_define_key(&key)),
            &__flight_argument_2,
        )
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:159 (sha256:6e8a9c2b8844834479451b7606aa6bfdc33613784e3db7879c3731f8cbd01a36)
pub fn get_wgpu_toon_module_source_for_key(key: &WgpuToonDefineKey) -> String {
    return (((((format!(
        "const ALPHA_MASK : bool = {};\n",
        if key.alpha_mask_enabled {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    ) + format!(
        "const DOUBLE_SIDED : bool = {};\n",
        if key.double_sided {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + format!(
        "const HAS_BASE_COLOR_MAP : bool = {};\n",
        if key.has_base_color_map {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + format!(
        "const HAS_RAMP : bool = {};\n",
        if key.has_ramp {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + wgpu_mesh_prelude_wgsl_constant)
        + TOON_WGSL_BODY);
}

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:172 (sha256:41596500c4fe8ca47d5dda787e47580ebc772728f36c66b5b85460c19d4254cc)
const TOON_UNIFORM_BYTES: f64 = 32.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:174 (sha256:8a33838d5b9effcc4f52739cd39ac80a756863afeafbb1fe87f697886bc39f70)
const TOON_WGSL_BODY: &'static str = "\nstruct ToonMaterial {\n  baseColor : vec4f,  // linear rgba\n  params : vec4f,     // x = steps, y = alphaCutoff\n};\n\n@group(2) @binding(0) var<uniform> material : ToonMaterial;\n@group(2) @binding(1) var materialSampler : sampler;\n@group(2) @binding(2) var baseColorTexture : texture_2d<f32>;\n@group(2) @binding(3) var rampTexture : texture_2d<f32>;\n\n// The directional shadow inputs (group 3), the shared shadow-sample layout ensureWgpuShadowSampleLayout\n// builds and beginWgpuMeshDraw binds. matrix is the light view-projection (world -> shadow clip);\n// params.x is the enabled flag. The WGSL mirror of scene-gl's shadow uniforms and wgpuPbrPrelude's Shadow.\nstruct Shadow {\n  matrix : mat4x4f,\n  params : vec4f,   // x = enabled (0 or 1)\n};\n\n@group(3) @binding(0) var<uniform> shadow : Shadow;\n@group(3) @binding(1) var shadowMap : texture_depth_2d;\n@group(3) @binding(2) var shadowSampler : sampler_comparison;\n\n// Directional shadow factor with 3x3 PCF — identical to wgpuPbrPrelude's copy. UV flips Y (WebGPU\n// top-left origin), depthRef remaps GL-convention clip Z (-1..1) into WebGPU's 0..1 range; the comparison\n// sampler ('less-equal') yields \"current <= closest\" per tap. Outside the frustum / no map bound = lit.\nfn sampleDirectionalShadow(worldPos : vec3f) -> f32 {\n  if (shadow.params.x < 0.5) {\n    return 1.0;\n  }\n  let clip = shadow.matrix * vec4f(worldPos, 1.0);\n  let ndc = clip.xyz / clip.w;\n  let uv = vec2f(ndc.x * 0.5 + 0.5, 1.0 - (ndc.y * 0.5 + 0.5));\n  let depthRef = ndc.z * 0.5 + 0.5 - 0.0025;\n  if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || depthRef > 1.0) {\n    return 1.0;\n  }\n  let texel = 1.0 / vec2f(textureDimensions(shadowMap, 0));\n  var sum = 0.0;\n  for (var x = -1; x <= 1; x = x + 1) {\n    for (var y = -1; y <= 1; y = y + 1) {\n      let offset = vec2f(f32(x), f32(y)) * texel;\n      sum = sum + textureSampleCompareLevel(shadowMap, shadowSampler, uv + offset, depthRef);\n    }\n  }\n  return sum / 9.0;\n}\n\n@fragment fn fs_main(in : VertexOutput, @builtin(front_facing) isFront : bool) -> @location(0) vec4f {\n  var baseColor = material.baseColor;\n  if (HAS_BASE_COLOR_MAP) {\n    let sampled = textureSample(baseColorTexture, materialSampler, in.uv);\n    baseColor = vec4f(baseColor.rgb * srgbToLinear(sampled.rgb), baseColor.a * sampled.a);\n  }\n\n  if (ALPHA_MASK && baseColor.a < material.params.y) {\n    discard;\n  }\n\n  var normal = normalize(in.worldNormal);\n  // Double-sided materials flip the normal for back faces so both sides shade correctly.\n  if (DOUBLE_SIDED && !isFront) {\n    normal = -normal;\n  }\n\n  var radiance = vec3f(0.0);\n\n  // Directional light: -direction is the surface-to-light vector (light travels along direction). The\n  // raw N·L is quantized into cel bands — a 1D ramp lookup when bound, else a stepped floor over steps —\n  // then scales the base color and the directional radiance. The banded contribution is shadow-mapped\n  // like the classic/PBR directional term; sampleDirectionalShadow is 1.0 when no map is bound.\n  if (frame.lightDirection.w > 0.5) {\n    let lightDir = normalize(-frame.lightDirection.xyz);\n    let nDotL = clamp(dot(normal, lightDir), 0.0, 1.0);\n    var direct = vec3f(0.0);\n    if (HAS_RAMP) {\n      let band = textureSample(rampTexture, materialSampler, vec2f(nDotL, 0.5)).rgb;\n      direct = baseColor.rgb * band * frame.directionalRadiance.rgb;\n    } else {\n      let steps = material.params.x;\n      let band = floor(nDotL * steps) / max(steps, 1.0);\n      direct = baseColor.rgb * band * frame.directionalRadiance.rgb;\n    }\n    radiance = radiance + direct * sampleDirectionalShadow(in.worldPosition);\n  }\n\n  // Ambient term: flat irradiance over the base color (unbanded).\n  if (frame.ambientRadiance.w > 0.5) {\n    radiance = radiance + baseColor.rgb * frame.ambientRadiance.rgb;\n  }\n\n  return vec4f(radiance, baseColor.a);\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts:269 (sha256:a8761476198a6a341b7722543c2fa31eda75d4e94f0173712249a77b005f6138)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![0.0_f32; (TOON_UNIFORM_BYTES / 4.0_f64) as usize])
});
