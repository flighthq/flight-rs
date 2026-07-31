// @generated from upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts; do not edit.
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

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:32 (sha256:b024ecf20addb9cf1fc01e3868192cfd3f3ff20ecc07fafba9b6fb269a1c6c38)
#[derive(Clone, Default)]
pub struct WgpuUnlitDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_color_map: bool,
}
impl PartialEq for WgpuUnlitDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:39 (sha256:13962e75a52f8bd0c83ad837857bb9e1a799458ef290fcc38635a558fc13b699)
#[derive(Clone, Default)]
pub struct WgpuUnlitPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuUnlitPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:45 (sha256:993815881cce179dfda6facb494c4ca103ac6a4221885fb50ec29e02e252de08)
pub fn bind_wgpu_unlit_surface(
    state: &mut WgpuRenderState,
    pipeline: &WgpuUnlitPipeline,
    material_key: crate::OpaqueHostValue,
    color: LinearColor,
    intensity: f64,
    alpha_cutoff: f64,
    color_map: Option<Texture>,
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
    (*_SCRATCH.lock().unwrap())[0.0_f64 as usize] = (color[0.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[1.0_f64 as usize] = (color[1.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[2.0_f64 as usize] = (color[2.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[3.0_f64 as usize] = (color[3.0_f64 as usize].clone()) as f32;
    (*_SCRATCH.lock().unwrap())[4.0_f64 as usize] = (intensity) as f32;
    (*_SCRATCH.lock().unwrap())[5.0_f64 as usize] = (alpha_cutoff) as f32;
    (*_SCRATCH.lock().unwrap())[6.0_f64 as usize] = (0.0_f64) as f32;
    (*_SCRATCH.lock().unwrap())[7.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    stash_wgpu_uv_transform(state, (color_map).clone());
    return (binding.as_mut().unwrap().bind_group).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:89 (sha256:e521ef30e49eaca7bc92016e34727da376133db9d7f66184f9581b061fd86b53)
pub fn build_wgpu_unlit_define_key(key: &WgpuUnlitDefineKey) -> String {
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
        if key.has_color_map {
            "c".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:96 (sha256:80fd0553827878ab8feee4a24cd83326748cd49b945a812620a1d7ea644b237c)
#[derive(Clone, Default)]
struct CompileWgpuUnlitPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CompileWgpuUnlitPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct CompileWgpuUnlitPipelineRecord6 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for CompileWgpuUnlitPipelineRecord6 {
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

pub fn compile_wgpu_unlit_pipeline(
    state: &mut WgpuRenderState,
    key: &WgpuUnlitDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuUnlitPipeline {
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
        WgpuUnlitPipeline {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            has_ibl_group: __flight_source.has_ibl_group,
            has_pbr_sample_group: __flight_source.has_pbr_sample_group,
            has_shadow_group: __flight_source.has_shadow_group,
            material_bind_group_layout: (__flight_source.material_bind_group_layout).clone(),
            pipeline: (__flight_source.pipeline).clone(),
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:115 (sha256:6e8df70b34dfe97cd76983399c5c60574a9156b82ea77da6e427e0185d75c72a)
pub fn ensure_wgpu_unlit_pipeline(
    mut state: WgpuRenderState,
    key: WgpuUnlitDefineKey,
    format: crate::OpaqueHostValue,
) -> WgpuUnlitPipeline {
    return {
        let __flight_argument_2 = (std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let key = key.clone();
            let mut state = state.clone();
            move || -> f64 { compile_wgpu_unlit_pipeline(&mut state, &key, (format).clone()) }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)))
        .clone();
        ensure_wgpu_scene_pipeline(
            &mut state,
            format!("unlit:{}|{}", format, build_wgpu_unlit_define_key(&key)),
            &__flight_argument_2,
        )
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:127 (sha256:766a3d06107e4edc895df8069297c4ff98e122f3c2c1b89198ac1ef8aca1dc98)
pub fn get_wgpu_unlit_module_source_for_key(key: &WgpuUnlitDefineKey) -> String {
    return (((format!(
        "const ALPHA_MASK : bool = {};\n",
        if key.alpha_mask_enabled {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    ) + format!(
        "const HAS_COLOR_MAP : bool = {};\n",
        if key.has_color_map {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    )) + wgpu_mesh_prelude_wgsl_constant)
        + UNLIT_WGSL_BODY);
}

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:138 (sha256:41ddbf5afecf9d769bc07c4221dfe03169f2865cc1a475e2f0844f3d59ce2f73)
const UNLIT_UNIFORM_BYTES: f64 = 32.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:140 (sha256:5cfe9e1aa91f19b00a56cc3830803af03b897be43e0281dc9b0d38c274b0aa9e)
const UNLIT_WGSL_BODY: &'static str = "\nstruct UnlitMaterial {\n  color : vec4f,   // linear rgba\n  params : vec4f,  // x = intensity, y = alphaCutoff\n};\n\n@group(2) @binding(0) var<uniform> material : UnlitMaterial;\n@group(2) @binding(1) var materialSampler : sampler;\n@group(2) @binding(2) var colorTexture : texture_2d<f32>;\n\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  var color = material.color;\n  if (HAS_COLOR_MAP) {\n    let sampled = textureSample(colorTexture, materialSampler, in.uv);\n    color = vec4f(color.rgb * srgbToLinear(sampled.rgb), color.a * sampled.a);\n  }\n  if (ALPHA_MASK && color.a < material.params.y) {\n    discard;\n  }\n  return vec4f(color.rgb * material.params.x, color.a);\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts:163 (sha256:f529581c040d024cba35c0cdff272e61a86858021738d9b419da761d1c52d271)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![0.0_f32; (UNLIT_UNIFORM_BYTES / 4.0_f64) as usize])
});
