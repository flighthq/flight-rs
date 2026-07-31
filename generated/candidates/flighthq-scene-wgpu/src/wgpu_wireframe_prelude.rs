// @generated from upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts; do not edit.
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

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:21 (sha256:a32c6690e3dcdc92088069a5cfce507767868fc512249bf6464bc4e23c1b2c1e)
#[derive(Clone, Default)]
pub struct WgpuWireframePipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuWireframePipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:25 (sha256:4505349aed4ffff9a3456baaa70f97fbcce2a96dab9ad6f3ed86ab8521f0475a)
pub fn bind_wgpu_wireframe_color(
    state: &mut WgpuRenderState,
    pipeline: &WgpuWireframePipeline,
    material_key: crate::OpaqueHostValue,
    color: LinearColor,
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
    crate::host_value::<()>("host.writeBuffer");
    stash_wgpu_uv_transform(state, None);
    return (binding.as_mut().unwrap().bind_group).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:60 (sha256:c9449b6efa3a1c9eceada50b3ef679cdfe29c611d84b64f7cb7d4540b44d85f6)
#[derive(Clone, Default)]
struct CompileWgpuWireframePipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for CompileWgpuWireframePipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct OptionsContextRecord6 {
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
impl PartialEq for OptionsContextRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn compile_wgpu_wireframe_pipeline(
    state: &mut WgpuRenderState,
    format: crate::OpaqueHostValue,
) -> WgpuWireframePipeline {
    let device = (state.device).clone();
    let module = crate::host_value::<()>("host.createShaderModule");
    let material_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    return {
        let __flight_source = &(create_wgpu_mesh_pipeline(
            state,
            &OptionsContextRecord6 {
                __flight_identity: std::sync::Arc::new(()),
                double_sided: true,
                format: (format).clone(),
                material_bind_group_layout: (material_bind_group_layout).clone(),
                module: (module).clone(),
                topology: Some(crate::OpaqueHostValue::String("line-list".to_owned())),
                ibl_bind_group_layout: None,
                pbr_sample_bind_group_layout: None,
                shadow_bind_group_layout: None,
            },
        ));
        WgpuWireframePipeline {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            has_ibl_group: __flight_source.has_ibl_group,
            has_pbr_sample_group: __flight_source.has_pbr_sample_group,
            has_shadow_group: __flight_source.has_shadow_group,
            material_bind_group_layout: (__flight_source.material_bind_group_layout).clone(),
            pipeline: (__flight_source.pipeline).clone(),
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:77 (sha256:c017692b251427896b20920b7b40ed53ffb47f0a2d3caef584ff0c3065105c0f)
pub fn ensure_wgpu_wireframe_pipeline(
    mut state: WgpuRenderState,
    format: crate::OpaqueHostValue,
) -> WgpuWireframePipeline {
    return {
        let __flight_argument_2 = (std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut state = state.clone();
            move || -> f64 { compile_wgpu_wireframe_pipeline(&mut state, (format).clone()) }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)))
        .clone();
        ensure_wgpu_scene_pipeline(
            &mut state,
            format!("wireframe:{}", format),
            &__flight_argument_2,
        )
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:83 (sha256:f2ddbe6d90cb69efe27832b4db5adf70ca2f12f4c2d72a9ffffaa865eef0c0ff)
pub fn get_wgpu_wireframe_module_source() -> String {
    return (wgpu_mesh_prelude_wgsl_constant + WIREFRAME_WGSL_BODY);
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:88 (sha256:78f5621272702d217d4156cdde8df4d919266ca3b54b96ee2295ec3e97900e07)
const WIREFRAME_UNIFORM_BYTES: f64 = 16.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:90 (sha256:6e025d453877b9897262885ed3507a8e28e8b431460384bf6ecac14678a18896)
const WIREFRAME_WGSL_BODY: &'static str = "\nstruct WireframeMaterial {\n  color : vec4f,  // linear rgba\n};\n\n@group(2) @binding(0) var<uniform> material : WireframeMaterial;\n\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  return material.color;\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts:102 (sha256:0f1aa3244f88910cab194d1c43bfc32fa1f868f11837f4fdee1a495f0f45c9fb)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![0.0_f32; (WIREFRAME_UNIFORM_BYTES / 4.0_f64) as usize])
});
