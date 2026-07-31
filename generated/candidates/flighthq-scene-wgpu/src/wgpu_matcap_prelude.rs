// @generated from upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts; do not edit.
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
use flighthq_color::LinearColor;
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

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:40 (sha256:ea8fd240bb20620b1aba45c2dda7ddba7ee0802a0fd121e0455624f38f7d28bc)
pub fn bind_wgpu_matcap_surface(
    state: &mut WgpuRenderState,
    pipeline: &WgpuMatcapPipeline,
    material_key: crate::OpaqueHostValue,
    tint: LinearColor,
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
    (*_SCRATCH.lock().unwrap())[0.0_f64 as usize] = (tint[0.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[1.0_f64 as usize] = (tint[1.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[2.0_f64 as usize] = (tint[2.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[3.0_f64 as usize] = (tint[3.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[4.0_f64 as usize] = (alpha_cutoff) as f32;
    (*_SCRATCH.lock().unwrap())[5.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH.lock().unwrap())[6.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH.lock().unwrap())[7.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    stash_wgpu_uv_transform(state, None);
    return (binding.as_mut().unwrap().bind_group).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:84 (sha256:bb5d359765558fcfc52c51bd83a810990b9428192d8e7f38a68fa5a3bdccbb53)
pub fn build_wgpu_matcap_define_key(key: &WgpuMatcapDefineKey) -> String {
    return format!(
        "{}{}{}",
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
        if key.has_matcap {
            "t".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:91 (sha256:9ff54a189cfc350b1f53c61f3322d63d8777b48277c69f586fdbc9811743d171)
#[derive(Clone, Default)]
struct CompileWgpuMatcapPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CompileWgpuMatcapPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CompileWgpuMatcapPipelineRecord6 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for CompileWgpuMatcapPipelineRecord6 {
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

pub fn compile_wgpu_matcap_pipeline(
    state: &mut WgpuRenderState,
    key: &WgpuMatcapDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuMatcapPipeline {
    let device = (state.device).clone();
    let module = crate::host_value::<()>("host.createShaderModule");
    let material_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    return {
        let __flight_source = &(create_wgpu_mesh_pipeline(
            state,
            &OptionsContextRecord7 {
                __flight_identity: std::sync::Arc::new(()),
                double_sided: key.double_sided,
                format: (format).clone(),
                material_bind_group_layout: (material_bind_group_layout).clone(),
                module: (module).clone(),
                ibl_bind_group_layout: None,
                pbr_sample_bind_group_layout: None,
                shadow_bind_group_layout: None,
                topology: None,
            },
        ));
        WgpuMatcapPipeline {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            has_ibl_group: __flight_source.has_ibl_group,
            has_pbr_sample_group: __flight_source.has_pbr_sample_group,
            has_shadow_group: __flight_source.has_shadow_group,
            material_bind_group_layout: (__flight_source.material_bind_group_layout).clone(),
            pipeline: (__flight_source.pipeline).clone(),
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:110 (sha256:6cc65c06f9fb84ef7cf737f2b0bf353a0e26c0d4521a67017b32ca6ad87215cd)
pub fn ensure_wgpu_matcap_pipeline(
    mut state: WgpuRenderState,
    key: WgpuMatcapDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuMatcapPipeline {
    return {
        let __flight_argument_2 = (std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let key = key.clone();
            let mut state = state.clone();
            move || -> f64 { compile_wgpu_matcap_pipeline(&mut state, &key, (format).clone()) }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)))
        .clone();
        ensure_wgpu_scene_pipeline(
            &mut state,
            format!("matcap:{}|{}", format, build_wgpu_matcap_define_key(&key)),
            &__flight_argument_2,
        )
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:122 (sha256:94d2727f4c925e118368f53b25cd139814d7af93991f4ec5c77e23f1bb31753b)
pub fn get_wgpu_matcap_module_source_for_key(key: &WgpuMatcapDefineKey) -> String {
    return (((format!(
        "const ALPHA_MASK : bool = {};\n",
        if key.alpha_mask_enabled {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    ) + format!(
        "const HAS_MATCAP : bool = {};\n",
        if key.has_matcap {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + wgpu_mesh_prelude_wgsl_constant)
        + MATCAP_WGSL_BODY);
}

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:134 (sha256:f5bf1fd0107ae533cb5257481380ec77fd36bb3253350b757f466f92e76a7aef)
#[derive(Clone, Default)]
pub struct WgpuMatcapDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_matcap: bool,
}
impl PartialEq for WgpuMatcapDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:141 (sha256:75902b1447676b35b111e2189f3a23d194034440b7bca6592cc1e7b4603022a8)
#[derive(Clone, Default)]
pub struct WgpuMatcapPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuMatcapPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:145 (sha256:8ad894e36ba75f2f210bc0538fffab012260f27866bc5ac19c226fdd0a4c94f6)
const MATCAP_UNIFORM_BYTES: f64 = 32.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:147 (sha256:b5b0749f3a1fffdd940b9c740479a238071b1e0de8d29442cd0a82a5cde5fa6a)
const MATCAP_WGSL_BODY: &'static str = "\nstruct MatcapMaterial {\n  tint : vec4f,    // linear rgba\n  params : vec4f,  // x = alphaCutoff\n};\n\n@group(2) @binding(0) var<uniform> material : MatcapMaterial;\n@group(2) @binding(1) var materialSampler : sampler;\n@group(2) @binding(2) var matcapTexture : texture_2d<f32>;\n\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  var color = material.tint;\n  if (HAS_MATCAP) {\n    // View-space-normal approximation: the shared Frame uniform carries no view matrix, so face the\n    // world normal toward the camera and project to 2D for the matcap lookup (uv = n.xy * 0.5 + 0.5).\n    // Present-but-unused while hasMatcap is false; the true view-space normal arrives with a view\n    // matrix in Frame + wgpu texture upload.\n    let worldNormal = normalize(in.worldNormal);\n    let viewDir = normalize(frame.cameraPosition.xyz - in.worldPosition);\n    let viewNormal = normalize(reflect(-viewDir, worldNormal));\n    let matcapUv = viewNormal.xy * 0.5 + 0.5;\n    let sampled = textureSample(matcapTexture, materialSampler, matcapUv);\n    color = vec4f(color.rgb * srgbToLinear(sampled.rgb), color.a * sampled.a);\n  }\n  if (ALPHA_MASK && color.a < material.params.x) {\n    discard;\n  }\n  return color;\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts:178 (sha256:aefdacf9aa59e52b89d16b98c475902cdaf573422f7bb9ea4dd1ef9d07598e6c)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![0.0_f32; (MATCAP_UNIFORM_BYTES / 4.0_f64) as usize])
});
