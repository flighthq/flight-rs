// @generated from upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ensure_wgpu_mesh_upload, get_wgpu_scene_runtime};
use flighthq_camera::get_camera_view_projection_matrix4;
use flighthq_geometry::{create_matrix3, create_matrix4, get_matrix4_position, inverse_matrix4};
use flighthq_image::has_image_resource_pixels;
use flighthq_render_wgpu::{
    bind_wgpu_image_resource_texture, get_wgpu_render_state_runtime, get_wgpu_sampler,
};
use flighthq_texture::{get_texture_uv_matrix, has_texture_uv_transform};
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, ImageResource,
    MAX_FORWARD_LIGHTS as max_forward_lights_constant, Matrix, Matrix3, Matrix4, Matrix4Like,
    MeshGeometry, SCENE_LIGHT_HEMISPHERE_OFFSET as scene_light_hemisphere_offset_constant,
    SCENE_LIGHT_HEMISPHERE_STRIDE as scene_light_hemisphere_stride_constant,
    SCENE_LIGHT_POINT_OFFSET as scene_light_point_offset_constant,
    SCENE_LIGHT_POINT_STRIDE as scene_light_point_stride_constant,
    SCENE_LIGHT_SPOT_OFFSET as scene_light_spot_offset_constant,
    SCENE_LIGHT_SPOT_STRIDE as scene_light_spot_stride_constant, Sampler, SceneGraphSyncPolicy,
    SceneLightBlock, SceneRenderProxy, SceneResourceRef, Texture, TextureColorSpace, TextureFilter,
    TextureLike, TextureWrap, Vector2, Vector3, WgpuRenderState,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub double_sided: bool,
    pub format: crate::OpaqueHostValue,
    pub ibl_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub module: crate::OpaqueHostValue,
    pub pbr_sample_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub shadow_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub topology: Option<crate::OpaqueHostValue>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
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
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Option<Vector2>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2>,
    pub color_space: Option<TextureColorSpace>,
    pub image: Option<ImageResource>,
    pub resource: Option<SceneResourceRef>,
    pub sampler: Option<Sampler>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord2902218824 {
    pub __flight_identity: std::sync::Arc<()>,
    pub array_stride: f64,
    pub attributes: Vec<ModuleSynthesizedRecord928826179>,
}
impl PartialEq for ModuleSynthesizedRecord2902218824 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord928826179 {
    pub __flight_identity: std::sync::Arc<()>,
    pub format: String,
    pub offset: f64,
    pub shader_location: f64,
}
impl PartialEq for ModuleSynthesizedRecord928826179 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:41 (sha256:c2e659aa9c576d5ec7f4ca6d8f9fd29844e3b7eb08e1fd654386cdf1d1baa696)
#[derive(Clone, Default)]
pub struct WgpuMeshPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuMeshPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:50 (sha256:b2933252b90ce6f369dd2753a01a568247121ca4dd437e18a43d1963df0e1ef9)
#[derive(Clone, Default)]
pub struct WgpuSceneLayouts {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_bind_group_layout: crate::OpaqueHostValue,
    pub frame_bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for WgpuSceneLayouts {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:58 (sha256:1b2e364e06383c2567a4eceadca1c0057088a2dfef3224b4111ab22f1a9e6755)
pub fn begin_wgpu_mesh_draw(state: &mut WgpuRenderState, pipeline: &WgpuMeshPipeline) -> () {
    let state_runtime = get_wgpu_render_state_runtime(state);
    let pass = (state_runtime.inner.lock().unwrap().render_pass).clone();
    if (pass).is_none() {
        return;
    }
    let mut scene = get_wgpu_scene_runtime(state);
    scene.active_mesh_pipeline = Some((*pipeline).clone());
    crate::host_value::<()>("host.setPipeline");
    crate::host_value::<()>("host.setBindGroup");
    if pipeline.has_pbr_sample_group {
        crate::host_value::<()>("host.setBindGroup");
    } else {
        if pipeline.has_shadow_group {
            crate::host_value::<()>("host.setBindGroup");
        }
    }
    if pipeline.has_ibl_group {
        crate::host_value::<()>("host.setBindGroup");
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:87 (sha256:9de870a7c3608ec18e9d13ef1a08e1839f43fa0d06461b32d92b90b1a63199ff)
pub fn create_wgpu_mesh_pipeline(
    state: &mut WgpuRenderState,
    options: &SharedStructuralRecord1,
) -> WgpuMeshPipeline {
    let device = (state.device).clone();
    let layouts = ensure_wgpu_scene_layouts(state);
    let mut bind_group_layouts: Vec<crate::OpaqueHostValue> = vec![
        (layouts.frame_bind_group_layout).clone(),
        (layouts.draw_bind_group_layout).clone(),
        (options.material_bind_group_layout).clone(),
    ];
    if ((options.pbr_sample_bind_group_layout).clone()).is_some() {
        bind_group_layouts.push(((options.pbr_sample_bind_group_layout).clone()).unwrap());
    } else {
        if ((options.shadow_bind_group_layout).clone()).is_some() {
            bind_group_layouts.push(((options.shadow_bind_group_layout).clone()).unwrap());
        }
        if ((options.ibl_bind_group_layout).clone()).is_some() {
            bind_group_layouts.push(((options.ibl_bind_group_layout).clone()).unwrap());
        }
    }
    let mut layout = crate::host_value::<()>("host.createPipelineLayout");
    let pipeline = crate::host_value::<()>("host.createRenderPipeline");
    return WgpuMeshPipeline {
        __flight_identity: std::sync::Arc::new(()),
        has_ibl_group: ((options.ibl_bind_group_layout).clone()).is_some(),
        has_pbr_sample_group: ((options.pbr_sample_bind_group_layout).clone()).is_some(),
        has_shadow_group: ((options.shadow_bind_group_layout).clone()).is_some(),
        material_bind_group_layout: (options.material_bind_group_layout).clone(),
        pipeline: (pipeline).clone(),
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:141 (sha256:897836acb886d2610027b9e451737af8ea08328b730972dd340f7f194a6f7968)
pub fn draw_wgpu_mesh_subset(
    state: &mut WgpuRenderState,
    proxy: &SceneRenderProxy,
    geometry: &mut MeshGeometry,
) -> () {
    let state_runtime = get_wgpu_render_state_runtime(state);
    let pass = (state_runtime.inner.lock().unwrap().render_pass).clone();
    let scene = get_wgpu_scene_runtime(state);
    if ((pass).is_none()) || (((scene.active_mesh_pipeline).clone()).is_none()) {
        return;
    }
    if (proxy.subset.index_count == 0.0_f64) {
        return;
    }
    let upload = ensure_wgpu_mesh_upload(state, geometry);
    if ((upload).is_none()) || (((upload.as_ref().unwrap().index_buffer).clone()).is_none()) {
        return;
    }
    let draw_bind_group = write_wgpu_draw_uniform(state, proxy);
    (*_DYNAMIC_OFFSETS.lock().unwrap())[0.0_f64 as usize] = (scene.pending_draw_offset) as u32;
    crate::host_value::<()>("host.setBindGroup");
    crate::host_value::<()>("host.setVertexBuffer");
    crate::host_value::<()>("host.setIndexBuffer");
    crate::host_value::<()>("host.drawIndexed");
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:169 (sha256:773b8c838c39933f5151794da93afa96d666ea4dd9de261d5cbb97abb2d30b5e)
pub fn ensure_wgpu_frame_bind_group(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.frame_buffer).clone()).is_none() {
        scene.frame_buffer = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBuffer",
        ));
    }
    if ((scene.frame_bind_group).clone()).is_none() {
        scene.frame_bind_group = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroup",
        ));
    }
    return ((scene.frame_bind_group).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:193 (sha256:d74fb015ce38cc3f49b2ed85f051f6f1c7637ff9d1b3bd6035d48006c12a537e)
#[derive(Clone, Default)]
struct EnsureWgpuIblSampleBindGroupRecord6 {
    __flight_identity: std::sync::Arc<()>,
    mag_filter: String,
    min_filter: String,
    mipmap_filter: String,
}
impl PartialEq for EnsureWgpuIblSampleBindGroupRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuIblSampleBindGroupRecord7 {
    __flight_identity: std::sync::Arc<()>,
    dimension: String,
}
impl PartialEq for EnsureWgpuIblSampleBindGroupRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_ibl_sample_bind_group(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    let device = (state.device).clone();
    if ((scene.ibl_uniform_buffer).clone()).is_none() {
        scene.ibl_uniform_buffer = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBuffer",
        ));
    }
    if ((scene.ibl_sampler).clone()).is_none() {
        scene.ibl_sampler = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createSampler",
        ));
    }
    if ((scene.ibl_dummy_cube_view).clone()).is_none() {
        scene.ibl_dummy_cube_texture = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createTexture",
        ));
        scene.ibl_dummy_cube_view = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createView",
        ));
        scene.ibl_dummy_lut_texture = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createTexture",
        ));
        scene.ibl_dummy_lut_view = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createView",
        ));
    }
    let ibl = (scene.ibl).clone();
    if (ibl).is_some() {
        _IBL_SAMPLE_SCRATCH[0.0_f64 as usize] = (1.0_f64) as f32;
        _IBL_SAMPLE_SCRATCH[1.0_f64 as usize] = (ibl.as_ref().unwrap().intensity) as f32;
        _IBL_SAMPLE_SCRATCH[2.0_f64 as usize] =
            (ibl.as_ref().unwrap().prefiltered_mip_count - 1.0_f64) as f32;
    } else {
        _IBL_SAMPLE_SCRATCH[0.0_f64 as usize] = (0.0_f64) as f32;
        _IBL_SAMPLE_SCRATCH[1.0_f64 as usize] = (1.0_f64) as f32;
        _IBL_SAMPLE_SCRATCH[2.0_f64 as usize] = (0.0_f64) as f32;
    }
    _IBL_SAMPLE_SCRATCH[3.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    let irradiance_view = if (ibl).is_some() {
        (ibl.as_ref().unwrap().irradiance_cube_view).clone()
    } else {
        ((scene.ibl_dummy_cube_view).clone()).unwrap()
    };
    let prefiltered_view = if (ibl).is_some() {
        (ibl.as_ref().unwrap().prefiltered_cube_view).clone()
    } else {
        ((scene.ibl_dummy_cube_view).clone()).unwrap()
    };
    let brdf_view = if (ibl).is_some() {
        (ibl.as_ref().unwrap().brdf_lut_view).clone()
    } else {
        ((scene.ibl_dummy_lut_view).clone()).unwrap()
    };
    if (((scene.ibl_sample_bind_group).clone()).is_none())
        || (!(((scene.ibl_sample_cube_view).clone()) == Some((irradiance_view).clone())))
    {
        scene.ibl_sample_bind_group = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroup",
        ));
        scene.ibl_sample_cube_view = Some((irradiance_view).clone());
    }
    return ((scene.ibl_sample_bind_group).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:265 (sha256:d7ebb430f9f8f1d2da337b97005fa34610e43086e17c6aeffdb7a09a9fd04260)
#[derive(Clone, Default)]
struct EnsureWgpuIblSampleLayoutRecord6 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureWgpuIblSampleLayoutRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuIblSampleLayoutRecord7 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
    view_dimension: String,
}
impl PartialEq for EnsureWgpuIblSampleLayoutRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuIblSampleLayoutRecord8 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for EnsureWgpuIblSampleLayoutRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_ibl_sample_layout(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.ibl_sample_layout).clone()).is_none() {
        scene.ibl_sample_layout = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroupLayout",
        ));
    }
    return ((scene.ibl_sample_layout).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:283 (sha256:6149a40444b5114f35408db88a9b127039ba73270eb675d94472e910c16b537c)
#[derive(Clone, Default)]
struct EnsureWgpuPbrSampleBindGroupRecord6 {
    __flight_identity: std::sync::Arc<()>,
    compare: String,
}
impl PartialEq for EnsureWgpuPbrSampleBindGroupRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuPbrSampleBindGroupRecord7 {
    __flight_identity: std::sync::Arc<()>,
    mag_filter: String,
    min_filter: String,
    mipmap_filter: String,
}
impl PartialEq for EnsureWgpuPbrSampleBindGroupRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuPbrSampleBindGroupRecord8 {
    __flight_identity: std::sync::Arc<()>,
    dimension: String,
}
impl PartialEq for EnsureWgpuPbrSampleBindGroupRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_pbr_sample_bind_group(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    let device = (state.device).clone();
    if ((scene.shadow_uniform_buffer).clone()).is_none() {
        scene.shadow_uniform_buffer = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBuffer",
        ));
    }
    if ((scene.shadow_comparison_sampler).clone()).is_none() {
        scene.shadow_comparison_sampler = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createSampler",
        ));
    }
    if ((scene.shadow_dummy_view).clone()).is_none() {
        scene.shadow_dummy_texture = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createTexture",
        ));
        scene.shadow_dummy_view = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createView",
        ));
    }
    if ((scene.ibl_uniform_buffer).clone()).is_none() {
        scene.ibl_uniform_buffer = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBuffer",
        ));
    }
    if ((scene.ibl_sampler).clone()).is_none() {
        scene.ibl_sampler = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createSampler",
        ));
    }
    if ((scene.ibl_dummy_cube_view).clone()).is_none() {
        scene.ibl_dummy_cube_texture = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createTexture",
        ));
        scene.ibl_dummy_cube_view = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createView",
        ));
        scene.ibl_dummy_lut_texture = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createTexture",
        ));
        scene.ibl_dummy_lut_view = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createView",
        ));
    }
    let shadow = (scene.shadow).clone();
    if (shadow).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < 16.0_f64) {
                (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[i as usize] =
                    (shadow.as_ref().unwrap().matrix.m[i as usize] as f64) as f32;
                {
                    i += 1.0;
                    i
                };
            }
        }
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[16.0_f64 as usize] = (1.0_f64) as f32;
    } else {
        {
            let mut i = 0.0_f64;
            while (i < 16.0_f64) {
                (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[i as usize] = (0.0_f64) as f32;
                {
                    i += 1.0;
                    i
                };
            }
        }
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[0.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[5.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[10.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[15.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[16.0_f64 as usize] = (0.0_f64) as f32;
    }
    (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[17.0_f64 as usize] = (0.0_f64) as f32;
    (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[18.0_f64 as usize] = (0.0_f64) as f32;
    (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[19.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    let ibl = (scene.ibl).clone();
    if (ibl).is_some() {
        _IBL_SAMPLE_SCRATCH[0.0_f64 as usize] = (1.0_f64) as f32;
        _IBL_SAMPLE_SCRATCH[1.0_f64 as usize] = (ibl.as_ref().unwrap().intensity) as f32;
        _IBL_SAMPLE_SCRATCH[2.0_f64 as usize] =
            (ibl.as_ref().unwrap().prefiltered_mip_count - 1.0_f64) as f32;
    } else {
        _IBL_SAMPLE_SCRATCH[0.0_f64 as usize] = (0.0_f64) as f32;
        _IBL_SAMPLE_SCRATCH[1.0_f64 as usize] = (1.0_f64) as f32;
        _IBL_SAMPLE_SCRATCH[2.0_f64 as usize] = (0.0_f64) as f32;
    }
    _IBL_SAMPLE_SCRATCH[3.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    let shadow_view = if (shadow).is_some() {
        (shadow.as_ref().unwrap().depth_view).clone()
    } else {
        ((scene.shadow_dummy_view).clone()).unwrap()
    };
    let irradiance_view = if (ibl).is_some() {
        (ibl.as_ref().unwrap().irradiance_cube_view).clone()
    } else {
        ((scene.ibl_dummy_cube_view).clone()).unwrap()
    };
    let prefiltered_view = if (ibl).is_some() {
        (ibl.as_ref().unwrap().prefiltered_cube_view).clone()
    } else {
        ((scene.ibl_dummy_cube_view).clone()).unwrap()
    };
    let brdf_view = if (ibl).is_some() {
        (ibl.as_ref().unwrap().brdf_lut_view).clone()
    } else {
        ((scene.ibl_dummy_lut_view).clone()).unwrap()
    };
    if ((((scene.pbr_sample_bind_group).clone()).is_none())
        || (!(((scene.pbr_sample_shadow_view).clone()) == Some((shadow_view).clone()))))
        || (!(((scene.pbr_sample_ibl_cube_view).clone()) == Some((irradiance_view).clone())))
    {
        scene.pbr_sample_bind_group = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroup",
        ));
        scene.pbr_sample_shadow_view = Some((shadow_view).clone());
        scene.pbr_sample_ibl_cube_view = Some((irradiance_view).clone());
    }
    return ((scene.pbr_sample_bind_group).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:394 (sha256:638f007a3988ab7469019aac46c7f427b6cfb282746859eee0f3058aa0da71a7)
#[derive(Clone, Default)]
struct EnsureWgpuPbrSampleLayoutRecord6 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureWgpuPbrSampleLayoutRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuPbrSampleLayoutRecord7 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for EnsureWgpuPbrSampleLayoutRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuPbrSampleLayoutRecord8 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
    view_dimension: String,
}
impl PartialEq for EnsureWgpuPbrSampleLayoutRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_pbr_sample_layout(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.pbr_sample_layout).clone()).is_none() {
        scene.pbr_sample_layout = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroupLayout",
        ));
    }
    return ((scene.pbr_sample_layout).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:416 (sha256:77745e75834707a3d47256e702b5da5192396614effdbe62c9e2e6d57a145f4c)
#[derive(Clone, Default)]
struct EnsureWgpuPlaceholderTextureViewRecord6 {
    __flight_identity: std::sync::Arc<()>,
    bytes_per_row: f64,
}
impl PartialEq for EnsureWgpuPlaceholderTextureViewRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_placeholder_texture_view(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    let mut view = (scene.placeholder_view).clone();
    if (view).is_none() {
        let texture = crate::host_value::<()>("host.createTexture");
        crate::host_value::<()>("host.writeTexture");
        view = Some((texture.create_view)());
        scene.placeholder_view = (view).clone();
    }
    return ((view).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:435 (sha256:00630e240983aa4b8a830711a3718203a55f529e7ab60a4612d60bff4fabf36b)
#[derive(Clone, Default)]
struct EnsureWgpuSceneLayoutsRecord6 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureWgpuSceneLayoutsRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuSceneLayoutsRecord7 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
    has_dynamic_offset: bool,
}
impl PartialEq for EnsureWgpuSceneLayoutsRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_scene_layouts(state: &mut WgpuRenderState) -> WgpuSceneLayouts {
    let mut scene = get_wgpu_scene_runtime(state);
    if (((scene.frame_bind_group_layout).clone()).is_none())
        || (((scene.draw_bind_group_layout).clone()).is_none())
    {
        let device = (state.device).clone();
        scene.frame_bind_group_layout = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroupLayout",
        ));
        scene.draw_bind_group_layout = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroupLayout",
        ));
    }
    return WgpuSceneLayouts {
        __flight_identity: std::sync::Arc::new(()),
        draw_bind_group_layout: ((scene.draw_bind_group_layout).clone()).unwrap(),
        frame_bind_group_layout: ((scene.frame_bind_group_layout).clone()).unwrap(),
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:456 (sha256:67a19249faa1300ac96bb4c05fc53ec60e3e2469c70d081ac099a767cab5ee39)
pub fn ensure_wgpu_scene_pipeline<T: Clone>(
    state: &mut WgpuRenderState,
    key: String,
    compile: &mut impl FnMut() -> T,
) -> T {
    let mut runtime = get_wgpu_scene_runtime(state);
    let mut pipeline = runtime
        .pipeline_cache
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (pipeline).is_none() {
        pipeline = Some(compile());
        {
            let __flight_key = (key).clone();
            let __flight_value = (pipeline).clone().unwrap();
            if let Some((_, value)) = runtime
                .pipeline_cache
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime.pipeline_cache.push((__flight_key, __flight_value));
            }
        };
    }
    return (pipeline).clone().unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:478 (sha256:a9c542ea2c79a622c8140f0fb3171c61c41d71d0d19a0c6cfe13ddbbd128af07)
#[derive(Clone, Default)]
struct EnsureWgpuShadowSampleBindGroupRecord6 {
    __flight_identity: std::sync::Arc<()>,
    compare: String,
}
impl PartialEq for EnsureWgpuShadowSampleBindGroupRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_shadow_sample_bind_group(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    let device = (state.device).clone();
    if ((scene.shadow_uniform_buffer).clone()).is_none() {
        scene.shadow_uniform_buffer = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBuffer",
        ));
    }
    if ((scene.shadow_comparison_sampler).clone()).is_none() {
        scene.shadow_comparison_sampler = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createSampler",
        ));
    }
    if ((scene.shadow_dummy_view).clone()).is_none() {
        scene.shadow_dummy_texture = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createTexture",
        ));
        scene.shadow_dummy_view = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createView",
        ));
    }
    let shadow = (scene.shadow).clone();
    if (shadow).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < 16.0_f64) {
                (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[i as usize] =
                    (shadow.as_ref().unwrap().matrix.m[i as usize] as f64) as f32;
                {
                    i += 1.0;
                    i
                };
            }
        }
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[16.0_f64 as usize] = (1.0_f64) as f32;
    } else {
        {
            let mut i = 0.0_f64;
            while (i < 16.0_f64) {
                (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[i as usize] = (0.0_f64) as f32;
                {
                    i += 1.0;
                    i
                };
            }
        }
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[0.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[5.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[10.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[15.0_f64 as usize] = (1.0_f64) as f32;
        (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[16.0_f64 as usize] = (0.0_f64) as f32;
    }
    (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[17.0_f64 as usize] = (0.0_f64) as f32;
    (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[18.0_f64 as usize] = (0.0_f64) as f32;
    (*_SHADOW_SAMPLE_SCRATCH.lock().unwrap())[19.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    let view = if (shadow).is_some() {
        (shadow.as_ref().unwrap().depth_view).clone()
    } else {
        ((scene.shadow_dummy_view).clone()).unwrap()
    };
    if (((scene.shadow_sample_bind_group).clone()).is_none())
        || (!(((scene.shadow_sample_view).clone()) == Some((view).clone())))
    {
        scene.shadow_sample_bind_group = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroup",
        ));
        scene.shadow_sample_view = Some((view).clone());
    }
    return ((scene.shadow_sample_bind_group).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:540 (sha256:5324c80ab1f4f8bf485c75b6021d9e6da2eaebe99b2c22d61f76e8063b6a0588)
#[derive(Clone, Default)]
struct EnsureWgpuShadowSampleLayoutRecord6 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureWgpuShadowSampleLayoutRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuShadowSampleLayoutRecord7 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for EnsureWgpuShadowSampleLayoutRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_shadow_sample_layout(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.shadow_sample_layout).clone()).is_none() {
        scene.shadow_sample_layout = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroupLayout",
        ));
    }
    return ((scene.shadow_sample_layout).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:561 (sha256:f8ae733515096ecf553034e44b2fd5f2fc75d8847492f851db5b378d31a1a872)
pub fn get_wgpu_material_sampler(
    state: &WgpuRenderState,
    texture: Option<Texture>,
) -> crate::OpaqueHostValue {
    if (texture).is_none() {
        return (get_wgpu_render_state_runtime(state)
            .inner
            .lock()
            .unwrap()
            .linear_sampler)
            .clone();
    }
    let filter = if (texture.as_ref().unwrap().sampler.mag_filter.starts_with)("nearest") {
        "nearest".to_owned()
    } else {
        "linear".to_owned()
    };
    let use_mips = ((texture.as_ref().unwrap().sampler.mipmaps)
        && ((texture.as_ref().unwrap().sampler.min_filter).clone() != "linear"))
        && ((texture.as_ref().unwrap().sampler.min_filter).clone() != "nearest");
    let mipmap_filter: Option<crate::OpaqueHostValue> = if use_mips {
        Some(
            if (texture.as_ref().unwrap().sampler.min_filter.ends_with)("nearest") {
                crate::OpaqueHostValue::String("nearest".to_owned())
            } else {
                crate::OpaqueHostValue::String("linear".to_owned())
            },
        )
    } else {
        None
    };
    return get_wgpu_sampler(
        state,
        (filter).clone(),
        (texture.as_ref().unwrap().sampler.wrap_u).clone(),
        (texture.as_ref().unwrap().sampler.wrap_v).clone(),
        Some(((mipmap_filter).clone().unwrap()).clone()),
        Some(texture.as_ref().unwrap().sampler.anisotropy),
    );
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:578 (sha256:5e63fdf3514c6cb9d7f505060be7f93111d6c4d25a282768f5bbec1b72d9e0e9)
pub fn is_wgpu_texture_ready(texture: Option<Texture>) -> bool {
    return (((texture).is_some()) && (((texture.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(texture.as_ref().unwrap().image.as_ref().unwrap()));
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:587 (sha256:bb538b97839a53ab40af104856bfb0550ce3cec811ac4c26174fbb094ac9112d)
pub fn resolve_wgpu_material_texture_view(
    state: &mut WgpuRenderState,
    texture: Option<Texture>,
) -> crate::OpaqueHostValue {
    if (((texture).is_some()) && (((texture.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(texture.as_ref().unwrap().image.as_ref().unwrap()))
    {
        return (bind_wgpu_image_resource_texture(
            state,
            texture.as_ref().unwrap().image.as_ref().unwrap(),
            Some(texture.as_ref().unwrap().sampler.mipmaps),
        )
        .view)
            .clone();
    }
    return ensure_wgpu_placeholder_texture_view(state);
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:605 (sha256:76704ac05066dac2b46a805d4eff94c0d135a58890c1588e9e1122ef1b243f90)
pub fn stash_wgpu_uv_transform(state: &mut WgpuRenderState, texture: Option<TextureLike>) -> () {
    if (((texture).is_none()) || (((texture.as_ref().unwrap().image).clone()).is_none()))
        || (!has_texture_uv_transform(&texture))
    {
        reset_wgpu_uv_transform_stash(&mut get_wgpu_scene_runtime(state).pending_uv_transform);
        return;
    }
    get_texture_uv_matrix(&mut (*SCRATCH_UV_MATRIX.lock().unwrap()), &texture);
    {
        let mut i = 0.0_f64;
        while (i < 9.0_f64) {
            get_wgpu_scene_runtime(state).pending_uv_transform[i as usize] =
                ((*SCRATCH_UV_MATRIX.lock().unwrap()).m[i as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:622 (sha256:1b35b89c84c28ff1534b012f7db3c2b29e9fc8c2bc22a4cc8fbc8c39f9463325)
pub fn write_wgpu_draw_uniform(
    state: &mut WgpuRenderState,
    proxy: &SceneRenderProxy,
) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    let mut state_runtime = get_wgpu_render_state_runtime(state);
    if ((scene.draw_bind_group).clone()).is_none() {
        scene.draw_bind_group = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroup",
        ));
    }
    let offset = state_runtime.inner.lock().unwrap().uniform_offset;
    let float_offset = (offset / 4.0_f64);
    {
        let mut i = 0.0_f64;
        while (i < 16.0_f64) {
            state_runtime.inner.lock().unwrap().uniform_data[(float_offset + i) as usize] =
                (proxy.world_matrix.m[i as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 16.0_f64) as usize] =
        (proxy.normal_matrix.m[0.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 17.0_f64) as usize] =
        (proxy.normal_matrix.m[1.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 18.0_f64) as usize] =
        (proxy.normal_matrix.m[2.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 19.0_f64) as usize] =
        (0.0_f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 20.0_f64) as usize] =
        (proxy.normal_matrix.m[3.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 21.0_f64) as usize] =
        (proxy.normal_matrix.m[4.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 22.0_f64) as usize] =
        (proxy.normal_matrix.m[5.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 23.0_f64) as usize] =
        (0.0_f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 24.0_f64) as usize] =
        (proxy.normal_matrix.m[6.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 25.0_f64) as usize] =
        (proxy.normal_matrix.m[7.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 26.0_f64) as usize] =
        (proxy.normal_matrix.m[8.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 27.0_f64) as usize] =
        (0.0_f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 28.0_f64) as usize] =
        (scene.pending_uv_transform[0.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 29.0_f64) as usize] =
        (scene.pending_uv_transform[1.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 30.0_f64) as usize] =
        (scene.pending_uv_transform[2.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 31.0_f64) as usize] =
        (0.0_f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 32.0_f64) as usize] =
        (scene.pending_uv_transform[3.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 33.0_f64) as usize] =
        (scene.pending_uv_transform[4.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 34.0_f64) as usize] =
        (scene.pending_uv_transform[5.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 35.0_f64) as usize] =
        (0.0_f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 36.0_f64) as usize] =
        (scene.pending_uv_transform[6.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 37.0_f64) as usize] =
        (scene.pending_uv_transform[7.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 38.0_f64) as usize] =
        (scene.pending_uv_transform[8.0_f64 as usize] as f64) as f32;
    state_runtime.inner.lock().unwrap().uniform_data[(float_offset + 39.0_f64) as usize] =
        (0.0_f64) as f32;
    scene.pending_draw_offset = offset;
    {
        let __flight_runtime = state_runtime;
        let __flight_value = state_runtime.inner.lock().unwrap().uniform_stride;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.uniform_offset += __flight_value;
    };
    return ((scene.draw_bind_group).clone()).unwrap();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:687 (sha256:fc7963483f638b4b558eaf12735528b1a89d3e698847f1f1d580cacba5b6ce9d)
pub fn write_wgpu_frame_uniform(
    state: &mut WgpuRenderState,
    camera: &Camera,
    lights: &SceneLightBlock,
) -> () {
    ensure_wgpu_frame_bind_group(state);
    let scene = get_wgpu_scene_runtime(state);
    let aspect = if (camera.projection.kind == "perspective") {
        camera.projection.aspect
    } else {
        1.0_f64
    };
    get_camera_view_projection_matrix4(
        &mut (*SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        if (aspect != 0.0_f64) { aspect } else { 1.0_f64 },
    );
    {
        let mut i = 0.0_f64;
        while (i < 16.0_f64) {
            (*_FRAME_SCRATCH.lock().unwrap())[i as usize] =
                ((*SCRATCH_VIEW_PROJECTION.lock().unwrap()).m[i as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    inverse_matrix4(&mut (*SCRATCH_INVERSE_VIEW.lock().unwrap()), &{
        let __flight_source = &(camera.view);
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    });
    get_matrix4_position(&mut (*SCRATCH_CAMERA_POSITION.lock().unwrap()), &{
        let __flight_source = &(*SCRATCH_INVERSE_VIEW.lock().unwrap());
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    });
    (*_FRAME_SCRATCH.lock().unwrap())[16.0_f64 as usize] = (scratchCameraPosition::x) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[17.0_f64 as usize] = (scratchCameraPosition::y) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[18.0_f64 as usize] = (scratchCameraPosition::z) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[19.0_f64 as usize] = (0.0_f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[20.0_f64 as usize] =
        (lights.data[0.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[21.0_f64 as usize] =
        (lights.data[1.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[22.0_f64 as usize] =
        (lights.data[2.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[23.0_f64 as usize] = (lights.directional_count) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[24.0_f64 as usize] =
        (lights.data[4.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[25.0_f64 as usize] =
        (lights.data[5.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[26.0_f64 as usize] =
        (lights.data[6.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[27.0_f64 as usize] = (0.0_f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[28.0_f64 as usize] =
        (lights.data[8.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[29.0_f64 as usize] =
        (lights.data[9.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[30.0_f64 as usize] =
        (lights.data[10.0_f64 as usize] as f64) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[31.0_f64 as usize] = (lights.ambient_count) as f32;
    {
        let mut i = 0.0_f64;
        while (i < 16.0_f64) {
            (*_FRAME_SCRATCH.lock().unwrap())[(32.0_f64 + i) as usize] =
                (camera.view.m[i as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    let point_floats = (scene_light_point_stride_constant * max_forward_lights_constant);
    {
        let mut i = 0.0_f64;
        while (i < point_floats) {
            (*_FRAME_SCRATCH.lock().unwrap())[(FRAME_POINT_OFFSET + i) as usize] =
                (lights.data[(scene_light_point_offset_constant + i) as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    let spot_floats = (scene_light_spot_stride_constant * max_forward_lights_constant);
    {
        let mut i = 0.0_f64;
        while (i < spot_floats) {
            (*_FRAME_SCRATCH.lock().unwrap())[(FRAME_SPOT_OFFSET + i) as usize] =
                (lights.data[(scene_light_spot_offset_constant + i) as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    let hemisphere_floats = (scene_light_hemisphere_stride_constant * max_forward_lights_constant);
    {
        let mut i = 0.0_f64;
        while (i < hemisphere_floats) {
            (*_FRAME_SCRATCH.lock().unwrap())[(FRAME_HEMISPHERE_OFFSET + i) as usize] =
                (lights.data[(scene_light_hemisphere_offset_constant + i) as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    (*_FRAME_SCRATCH.lock().unwrap())[FRAME_COUNTS_OFFSET as usize] = (lights.point_count) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[(FRAME_COUNTS_OFFSET + 1.0_f64) as usize] =
        (lights.spot_count) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[(FRAME_COUNTS_OFFSET + 2.0_f64) as usize] =
        (lights.hemisphere_count) as f32;
    (*_FRAME_SCRATCH.lock().unwrap())[(FRAME_COUNTS_OFFSET + 3.0_f64) as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:752 (sha256:55fea7d13e0b2f83f8f30e6f5361aec965e99493c70d57de94d93204353aab7e)
pub const WGPU_MESH_PRELUDE_WGSL: &'static str = "\nconst PI : f32 = 3.14159265359;\nconst MAX_FORWARD_LIGHTS : u32 = 4u;\n\nstruct Frame {\n  viewProjection : mat4x4f,\n  cameraPosition : vec4f,\n  lightDirection : vec4f,       // xyz = directional light travel direction; w = directionalCount\n  directionalRadiance : vec4f,  // rgb = linear premultiplied radiance\n  ambientRadiance : vec4f,      // rgb = linear premultiplied radiance; w = ambientCount\n  view : mat4x4f,               // camera view matrix; rotates world normals into view space (matcap)\n  // Punctual light arrays — layout mirrors SceneLightBlock.data (packSceneLightBlock).\n  //   point[i]      = pointLights[i*2+0]={pos.xyz,range}, [i*2+1]={radiance.rgb,invSqrRange}\n  //   spot[i]       = spotLights[i*4+0..1] as point, [i*4+2]={dir.xyz,_}, [i*4+3]={cosInner,cosOuter,_,_}\n  //   hemisphere[i] = hemisphereLights[i*3+0]={sky.rgb,_}, [i*3+1]={ground.rgb,_}, [i*3+2]={up.xyz,_}\n  pointLights : array<vec4f, 8>,       // MAX_FORWARD_LIGHTS * 2\n  spotLights : array<vec4f, 16>,       // MAX_FORWARD_LIGHTS * 4\n  hemisphereLights : array<vec4f, 12>, // MAX_FORWARD_LIGHTS * 3\n  punctualCounts : vec4f,              // x = pointCount, y = spotCount, z = hemisphereCount\n};\n\nstruct Draw {\n  world : mat4x4f,\n  normalMatrix : mat3x3f,\n  uvTransform : mat3x3f,   // KHR_texture_transform of the material's primary map (identity when unused)\n};\n\n@group(0) @binding(0) var<uniform> frame : Frame;\n@group(1) @binding(0) var<uniform> draw : Draw;\n\nstruct VertexOutput {\n  @builtin(position) clipPosition : vec4f,\n  @location(0) worldPosition : vec3f,\n  @location(1) worldNormal : vec3f,\n  @location(2) worldTangent : vec4f,\n  @location(3) uv : vec2f,\n};\n\n@vertex fn vs_main(\n  @location(0) position : vec3f,\n  @location(1) normal : vec3f,\n  @location(2) tangent : vec4f,\n  @location(3) uv : vec2f,\n) -> VertexOutput {\n  var out : VertexOutput;\n  let world = draw.world * vec4f(position, 1.0);\n  out.worldPosition = world.xyz;\n  out.clipPosition = frame.viewProjection * world;\n  out.worldNormal = draw.normalMatrix * normal;\n  out.worldTangent = vec4f(draw.normalMatrix * tangent.xyz, tangent.w);\n  // Apply the material's KHR_texture_transform to the uv. draw.uvTransform is identity for an untiled\n  // material (writeWgpuDrawUniform's default), so this is a no-op there — applied unconditionally rather\n  // than behind a pipeline const because this vs_main is shared by every family (classic/unlit/toon/\n  // matcap/debug/wireframe) and a const would have to thread through all of them; a per-vertex mat3\n  // multiply is negligible. The scene-gl mirror gates the equivalent branch via its #ifdef variant.\n  out.uv = (draw.uvTransform * vec3f(uv, 1.0)).xy;\n  return out;\n}\n\n// sRgb albedo texels are gamma-encoded; decode to linear before lighting.\nfn srgbToLinear(c : vec3f) -> vec3f {\n  let lo = c / 12.92;\n  let hi = pow((c + vec3f(0.055)) / 1.055, vec3f(2.4));\n  return select(lo, hi, c > vec3f(0.04045));\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:824 (sha256:f63702d650ac241896f864ad8bdbbaf95d99d051b4024c471a5f71c39b4cd2d9)
const FRAME_POINT_OFFSET: f64 = 48.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:825 (sha256:e4faedeefcdd4b83c6f525163ff7e4150d2ac458affea0525109abba6a7c9b39)
const FRAME_SPOT_OFFSET: f64 =
    (FRAME_POINT_OFFSET + (scene_light_point_stride_constant * max_forward_lights_constant));

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:826 (sha256:5c02d20955cfe74eb70fd98c331ed8f4688f3987d6730c889f7b61ab2cba0c56)
const FRAME_HEMISPHERE_OFFSET: f64 =
    (FRAME_SPOT_OFFSET + (scene_light_spot_stride_constant * max_forward_lights_constant));

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:827 (sha256:02584f2d1a29f72035797dea6bbaf27e6f28ffa8e83e96e97f15f998d58149dd)
const FRAME_COUNTS_OFFSET: f64 = (FRAME_HEMISPHERE_OFFSET
    + (scene_light_hemisphere_stride_constant * max_forward_lights_constant));

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:833 (sha256:4075d0be9b6da3da5132a3fa928fc8121d04b64b6cf20eace976039f7856aff0)
const FRAME_UNIFORM_BYTES: f64 = ((FRAME_COUNTS_OFFSET + 4.0_f64) * 4.0_f64);

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:838 (sha256:643b1b1764929d9085f38880886dba803aeaa7a85b08fb738f575ee5f86bb02c)
const DRAW_UNIFORM_BYTES: f64 = 160.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:841 (sha256:ff4c7d541fefccfa641f5378ee994277c49ed4024b3b912efbe1f1acf79b06d6)
fn reset_wgpu_uv_transform_stash(out: &mut Vec<f32>) -> () {
    out[0.0_f64 as usize] = (1.0_f64) as f32;
    out[1.0_f64 as usize] = (0.0_f64) as f32;
    out[2.0_f64 as usize] = (0.0_f64) as f32;
    out[3.0_f64 as usize] = (0.0_f64) as f32;
    out[4.0_f64 as usize] = (1.0_f64) as f32;
    out[5.0_f64 as usize] = (0.0_f64) as f32;
    out[6.0_f64 as usize] = (0.0_f64) as f32;
    out[7.0_f64 as usize] = (0.0_f64) as f32;
    out[8.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:854 (sha256:591628d13476cd1379f3e3321adebd8dde23da6c434a9b73ccb6259319613fd2)
static SCRATCH_UV_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ))
    });

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:858 (sha256:343dac28670ced6c1f8cde8e29749cac7980c6e130e5cd10a216d956f2e85dde)
const DEPTH_STENCIL_FORMAT: &'static str = "depth24plus-stencil8";

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:862 (sha256:69c5652f93ace728fb55f3b0a394a5188ed99a54facd02b49d8d4d1cd9ff3037)
pub const SHADOW_DEPTH_FORMAT: &'static str = "depth32float";

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:865 (sha256:8255bc2b28be009dab9e0567cab3eadc0b32a3fc388424a46c4753b71da94805)
const SHADOW_SAMPLE_UNIFORM_BYTES: f64 = 80.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:868 (sha256:a183496ad1d756e4227b5ef752b21821844b16b7a24f93f6c243b2b2b983a393)
const IBL_SAMPLE_UNIFORM_BYTES: f64 = 16.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:872 (sha256:3ad4d7017496ab620c38b334e490a5be358395dd00bd2b048524a57fe6cb1867)
const IBL_DUMMY_FORMAT: &'static str = "rgba8unorm";

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:875 (sha256:e3b53a5f8871c52fa8f6288404cd13cce354ed7f90dd9905187ba8fa3183b073)
static WHITE_PIXEL: std::sync::LazyLock<Vec<u8>> = std::sync::LazyLock::new(|| {
    (vec![255.0_f64, 255.0_f64, 255.0_f64, 255.0_f64])
        .iter()
        .map(|value| (*value) as u8)
        .collect()
});

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:879 (sha256:1ff13794d056ee50aec9e6f6234a34962117143039d3a802ed70f9fc0980b86a)
static VERTEX_BUFFER_LAYOUTS: std::sync::LazyLock<Vec<crate::OpaqueHostValue>> =
    std::sync::LazyLock::new(|| {
        vec![ModuleSynthesizedRecord2902218824 {
            __flight_identity: std::sync::Arc::new(()),
            array_stride: 48.0_f64,
            attributes: vec![
                ModuleSynthesizedRecord928826179 {
                    __flight_identity: std::sync::Arc::new(()),
                    shader_location: 0.0_f64,
                    offset: 0.0_f64,
                    format: "float32x3".to_owned(),
                },
                ModuleSynthesizedRecord928826179 {
                    __flight_identity: std::sync::Arc::new(()),
                    shader_location: 1.0_f64,
                    offset: 12.0_f64,
                    format: "float32x3".to_owned(),
                },
                ModuleSynthesizedRecord928826179 {
                    __flight_identity: std::sync::Arc::new(()),
                    shader_location: 2.0_f64,
                    offset: 24.0_f64,
                    format: "float32x4".to_owned(),
                },
                ModuleSynthesizedRecord928826179 {
                    __flight_identity: std::sync::Arc::new(()),
                    shader_location: 3.0_f64,
                    offset: 40.0_f64,
                    format: "float32x2".to_owned(),
                },
            ],
        }]
    });

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:891 (sha256:140acb0d499b3786d700284ab9e3540997031e7613e61aec3eb90cf6d2ab88c6)
static SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:892 (sha256:cac89509b1c8459b541129553f50270085b134abaef34130e7309e6463eaf999)
static SCRATCH_INVERSE_VIEW: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:893 (sha256:87ef2e4557c2d0eb9483fdfe75d849407b3f25f82a43c851b3a0ff34fb85a5b9)
static SCRATCH_CAMERA_POSITION: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(Vector3 {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_runtime: Default::default(),
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f64,
        })
    });

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:894 (sha256:0fbe4a48ac974bf60810ff2253836c167bf63ca58fcd6d91bc3f0d4a446376b7)
static _FRAME_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (FRAME_UNIFORM_BYTES / 4.0_f64) as usize])
    });

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:895 (sha256:58f9b06eb92a298e62fc3daaeeb389221c18a43a05437c9efca88f9e2187010d)
static _DYNAMIC_OFFSETS: std::sync::LazyLock<std::sync::Mutex<Vec<u32>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0_u32; (1.0_f64) as usize]));

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:896 (sha256:a63935305ea1aaa902afcdff9ba9583dc87afb865d96d378d450a4f9c7c05b04)
static _SHADOW_SAMPLE_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            0.0_f32;
            (SHADOW_SAMPLE_UNIFORM_BYTES / 4.0_f64) as usize
        ])
    });

// Source: upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts:897 (sha256:a8f08dc10f366f65031928b12369e83a9a0c77c28aa9dcb0773eae81a9fea145)
static _IBL_SAMPLE_SCRATCH: std::sync::LazyLock<Vec<f32>> =
    std::sync::LazyLock::new(|| vec![0.0_f32; (IBL_SAMPLE_UNIFORM_BYTES / 4.0_f64) as usize]);
