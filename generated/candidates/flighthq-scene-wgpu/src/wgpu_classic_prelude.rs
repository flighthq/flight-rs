// @generated from upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WGPU_MESH_PRELUDE_WGSL as wgpu_mesh_prelude_wgsl_constant, WgpuMaterialBinding,
    create_wgpu_mesh_pipeline, ensure_wgpu_scene_pipeline, ensureWgpuShadowSampleLayout,
    get_wgpu_scene_runtime, stash_wgpu_uv_transform,
};
use flighthq_color::LinearColor;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, ImageResource, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, Texture, TextureColorSpace, TextureFilter, TextureWrap,
    Vector2, WgpuRenderState,
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

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:40 (sha256:af5eb146a079d945bf08d07c1bd3a5e32598423ed6bdac412f5af9cd9031e60c)
pub type WgpuClassicLightingModel = String;

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:43 (sha256:d33a6ae6c9ca4677439cf4ca374f46b18a1913d8897d75c3ac72cbff54b288df)
#[derive(Clone, Default)]
pub struct WgpuClassicPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuClassicPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:50 (sha256:a513f7e73269b2a17cce705705f8ef767d15570a54515e335389d22979d8e5f5)
#[derive(Clone, Default)]
pub struct WgpuClassicDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_diffuse_map: bool,
    pub has_normal_map: bool,
    pub has_specular_map: bool,
    pub lighting_model: WgpuClassicLightingModel,
}
impl PartialEq for WgpuClassicDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:64 (sha256:a18c09e1a26bf932fe6a133ef8cbb297fb9e14d976ae824d286c08b5d5fd07eb)
pub fn bind_wgpu_classic_surface(
    state: &mut WgpuRenderState,
    pipeline: &WgpuClassicPipeline,
    material_key: crate::OpaqueHostValue,
    diffuse: LinearColor,
    specular: LinearColor,
    shininess: f64,
    alpha_cutoff: f64,
    diffuse_map: Option<Texture>,
    specular_map: Option<Texture>,
    normal_map: Option<Texture>,
) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    let mut binding: Option<WgpuMaterialBinding> = scene
        .material_bind_groups
        .iter()
        .find(|(key, _)| key == &(material_key).clone())
        .map(|(_, value)| value.clone());
    if (binding).is_none() {
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
    (*_SCRATCH.lock().unwrap())[0.0_f64 as usize] = (diffuse[0.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[1.0_f64 as usize] = (diffuse[1.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[2.0_f64 as usize] = (diffuse[2.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[3.0_f64 as usize] = (diffuse[3.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[4.0_f64 as usize] = (specular[0.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[5.0_f64 as usize] = (specular[1.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[6.0_f64 as usize] = (specular[2.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[7.0_f64 as usize] = (specular[3.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[8.0_f64 as usize] = (shininess) as f32;
    (*_SCRATCH.lock().unwrap())[9.0_f64 as usize] = (alpha_cutoff) as f32;
    (*_SCRATCH.lock().unwrap())[10.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH.lock().unwrap())[11.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    stash_wgpu_uv_transform(state, (diffuse_map).clone());
    return (binding.as_mut().unwrap().bind_group).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:119 (sha256:eeec0ffc90fe5ac25d861cfbef2f01491ec0a03ded21a56fb2f8eb9cfe9ef15a)
pub fn build_wgpu_classic_define_key(key: &WgpuClassicDefineKey) -> String {
    let model = if ((key.lighting_model).clone() == "phong") {
        "p".to_owned()
    } else {
        if ((key.lighting_model).clone() == "blinnphong") {
            "b".to_owned()
        } else {
            "l".to_owned()
        }
    };
    return format!(
        "{}{}{}{}{}{}",
        model,
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
        if key.has_diffuse_map {
            "d".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_specular_map {
            "s".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_normal_map {
            "n".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:129 (sha256:5cfce9c959ada360b67f57bd558f61827639a36d676402cd5365bb629781f3a7)
#[derive(Clone, Default)]
struct CompileWgpuClassicPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CompileWgpuClassicPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CompileWgpuClassicPipelineRecord6 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for CompileWgpuClassicPipelineRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CompileWgpuClassicPipelineSynthesizedRecord1880144846 {
    __flight_identity: std::sync::Arc<()>,
    double_sided: bool,
    format: crate::OpaqueHostValue,
    material_bind_group_layout: crate::OpaqueHostValue,
    module: crate::OpaqueHostValue,
    shadow_bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for CompileWgpuClassicPipelineSynthesizedRecord1880144846 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn compile_wgpu_classic_pipeline(
    state: &mut WgpuRenderState,
    key: &WgpuClassicDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuClassicPipeline {
    let device = (state.device).clone();
    let module = crate::host_value::<()>("host.createShaderModule");
    let material_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    return {
        let __flight_source = &({
            let __flight_argument_1 = (CompileWgpuClassicPipelineSynthesizedRecord1880144846 {
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
        WgpuClassicPipeline {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            has_ibl_group: __flight_source.has_ibl_group,
            has_pbr_sample_group: __flight_source.has_pbr_sample_group,
            has_shadow_group: __flight_source.has_shadow_group,
            material_bind_group_layout: (__flight_source.material_bind_group_layout).clone(),
            pipeline: (__flight_source.pipeline).clone(),
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:160 (sha256:a529f6961c55fe419f1b12324f0888f7b8e12db1078185dc5850a7401f26d677)
pub fn ensure_wgpu_classic_pipeline(
    mut state: WgpuRenderState,
    key: WgpuClassicDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuClassicPipeline {
    return {
        let __flight_argument_2 = (std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let key = key.clone();
            let mut state = state.clone();
            move || -> f64 { compile_wgpu_classic_pipeline(&mut state, &key, (format).clone()) }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)))
        .clone();
        ensure_wgpu_scene_pipeline(
            &mut state,
            format!("classic:{}|{}", format, build_wgpu_classic_define_key(&key)),
            &__flight_argument_2,
        )
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:172 (sha256:4e24b6acc1539de01797be20815d7ec35739fee6d3b106ba2938671e8ad3291c)
pub fn get_wgpu_classic_module_source_for_key(key: &WgpuClassicDefineKey) -> String {
    return ((((((((format!(
        "const LIGHTING_PHONG : bool = {};\n",
        if ((key.lighting_model).clone() == "phong") {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    ) + format!(
        "const LIGHTING_BLINNPHONG : bool = {};\n",
        if ((key.lighting_model).clone() == "blinnphong") {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + format!(
        "const ALPHA_MASK : bool = {};\n",
        if key.alpha_mask_enabled {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + format!(
        "const DOUBLE_SIDED : bool = {};\n",
        if key.double_sided {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + format!(
        "const HAS_DIFFUSE_MAP : bool = {};\n",
        if key.has_diffuse_map {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + format!(
        "const HAS_SPECULAR_MAP : bool = {};\n",
        if key.has_specular_map {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + format!(
        "const HAS_NORMAL_MAP : bool = {};\n",
        if key.has_normal_map {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + wgpu_mesh_prelude_wgsl_constant)
        + CLASSIC_WGSL_BODY);
}

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:188 (sha256:8f9112787e94cfb21d884931c27652c8c2d6948b196307bc2e48a273ac05e4bf)
const CLASSIC_UNIFORM_BYTES: f64 = 48.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:190 (sha256:997471067f2e6106db69e0770fe07b5faa455d0696380d8c2762361830396b90)
const CLASSIC_WGSL_BODY: &'static str = "\nstruct ClassicMaterial {\n  diffuse : vec4f,   // linear rgba\n  specular : vec4f,  // linear rgb; a unused\n  params : vec4f,    // x = shininess, y = alphaCutoff\n};\n\n@group(2) @binding(0) var<uniform> material : ClassicMaterial;\n@group(2) @binding(1) var materialSampler : sampler;\n@group(2) @binding(2) var diffuseTexture : texture_2d<f32>;\n@group(2) @binding(3) var specularTexture : texture_2d<f32>;\n@group(2) @binding(4) var normalTexture : texture_2d<f32>;\n\n// The directional shadow inputs (group 3), the shared shadow-sample layout ensureWgpuShadowSampleLayout\n// builds and beginWgpuMeshDraw binds. matrix is the light view-projection (world -> shadow clip);\n// params.x is the enabled flag (0 or 1). The WGSL mirror of scene-gl's u_shadowMap / u_shadowMatrix /\n// u_shadowEnabled and wgpuPbrPrelude's Shadow.\nstruct Shadow {\n  matrix : mat4x4f,\n  params : vec4f,   // x = enabled (0 or 1)\n};\n\n@group(3) @binding(0) var<uniform> shadow : Shadow;\n@group(3) @binding(1) var shadowMap : texture_depth_2d;\n@group(3) @binding(2) var shadowSampler : sampler_comparison;\n\n// Directional shadow factor at a world position: 1.0 fully lit, 0.0 fully shadowed, with 3x3 PCF —\n// identical to wgpuPbrPrelude's copy. UV flips Y (WebGPU top-left origin), depthRef remaps GL-convention\n// clip Z (-1..1) into WebGPU's 0..1 range; the comparison sampler ('less-equal') yields \"current <=\n// closest\" per tap. Fragments outside the shadow frustum, or when no map is bound, read as lit.\nfn sampleDirectionalShadow(worldPos : vec3f) -> f32 {\n  if (shadow.params.x < 0.5) {\n    return 1.0;\n  }\n  let clip = shadow.matrix * vec4f(worldPos, 1.0);\n  let ndc = clip.xyz / clip.w;\n  let uv = vec2f(ndc.x * 0.5 + 0.5, 1.0 - (ndc.y * 0.5 + 0.5));\n  let depthRef = ndc.z * 0.5 + 0.5 - 0.0025;\n  if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || depthRef > 1.0) {\n    return 1.0;\n  }\n  let texel = 1.0 / vec2f(textureDimensions(shadowMap, 0));\n  var sum = 0.0;\n  for (var x = -1; x <= 1; x = x + 1) {\n    for (var y = -1; y <= 1; y = y + 1) {\n      let offset = vec2f(f32(x), f32(y)) * texel;\n      sum = sum + textureSampleCompareLevel(shadowMap, shadowSampler, uv + offset, depthRef);\n    }\n  }\n  return sum / 9.0;\n}\n\n@fragment fn fs_main(in : VertexOutput, @builtin(front_facing) isFront : bool) -> @location(0) vec4f {\n  var diffuse = material.diffuse;\n  if (HAS_DIFFUSE_MAP) {\n    let sampled = textureSample(diffuseTexture, materialSampler, in.uv);\n    diffuse = vec4f(diffuse.rgb * srgbToLinear(sampled.rgb), diffuse.a * sampled.a);\n  }\n\n  if (ALPHA_MASK && diffuse.a < material.params.y) {\n    discard;\n  }\n\n  var geometricNormal = normalize(in.worldNormal);\n  // Double-sided materials flip the normal for back faces so both sides shade correctly.\n  if (DOUBLE_SIDED && !isFront) {\n    geometricNormal = -geometricNormal;\n  }\n\n  var normal = geometricNormal;\n  if (HAS_NORMAL_MAP) {\n    let tangent = normalize(in.worldTangent.xyz);\n    let bitangent = cross(geometricNormal, tangent) * in.worldTangent.w;\n    var tangentNormal = textureSample(normalTexture, materialSampler, in.uv).xyz * 2.0 - vec3f(1.0);\n    let tbn = mat3x3f(tangent, bitangent, geometricNormal);\n    normal = normalize(tbn * tangentNormal);\n  }\n\n  // Specular color is resolved here in UNIFORM control flow. WGSL forbids textureSample inside the\n  // per-pixel lighting branch below (it depends on nDotL, a non-uniform value), so the map sample is\n  // hoisted out. Maps are deferred on wgpu (placeholder bound), so this stays the material specular\n  // until texture upload lands.\n  var specularColor = material.specular.rgb;\n  if (HAS_SPECULAR_MAP) {\n    let sampledSpecular = textureSample(specularTexture, materialSampler, in.uv);\n    specularColor = specularColor * srgbToLinear(sampledSpecular.rgb);\n  }\n\n  var radiance = vec3f(0.0);\n\n  // Directional light: -direction is the surface-to-light vector (light travels along direction).\n  // The whole directional contribution (diffuse + specular) is PCF shadow-mapped, mirroring the PBR path;\n  // sampleDirectionalShadow returns 1.0 when no shadow map is bound, so an unshadowed scene is unchanged.\n  if (frame.lightDirection.w > 0.5) {\n    let lightDir = normalize(-frame.lightDirection.xyz);\n    let nDotL = max(dot(normal, lightDir), 0.0);\n    var direct = diffuse.rgb * nDotL * frame.directionalRadiance.rgb;\n\n    if ((LIGHTING_PHONG || LIGHTING_BLINNPHONG) && nDotL > 0.0) {\n      let viewDir = normalize(frame.cameraPosition.xyz - in.worldPosition);\n      var specAngle = 0.0;\n      if (LIGHTING_PHONG) {\n        // Phong: reflection-vector specular.\n        let reflectDir = reflect(-lightDir, normal);\n        specAngle = max(dot(reflectDir, viewDir), 0.0);\n      } else {\n        // BlinnPhong: half-vector specular.\n        let halfVec = normalize(lightDir + viewDir);\n        specAngle = max(dot(normal, halfVec), 0.0);\n      }\n      let specular = pow(specAngle, max(material.params.x, 1.0));\n      direct = direct + specular * specularColor * frame.directionalRadiance.rgb;\n    }\n\n    radiance = radiance + direct * sampleDirectionalShadow(in.worldPosition);\n  }\n\n  // Ambient term: flat irradiance over the diffuse albedo.\n  if (frame.ambientRadiance.w > 0.5) {\n    radiance = radiance + diffuse.rgb * frame.ambientRadiance.rgb;\n  }\n\n  return vec4f(radiance, diffuse.a);\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts:316 (sha256:e690f0f071bc3323cd6d51ab6454854353cd0c180309502dbc5cea2c08f5de20)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![0.0_f32; (CLASSIC_UNIFORM_BYTES / 4.0_f64) as usize])
});
