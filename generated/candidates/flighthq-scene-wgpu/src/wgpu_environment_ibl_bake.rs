// @generated from upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{WgpuSceneIbl, ensure_wgpu_environment_source_cube, get_wgpu_scene_runtime};
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, Environment, ImageResource, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
    WgpuRenderState,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

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

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:20 (sha256:98da811f628a5ea03c4cd9d8fbd63b4a2ff7fa6a07727403768d7530860062c4)
pub fn bake_wgpu_environment_ibl(state: &mut WgpuRenderState, environment: &Environment) -> () {
    let source_cube_view = ensure_wgpu_environment_source_cube(state, environment);
    if (source_cube_view).is_none() {
        return;
    }
    let mut scene = get_wgpu_scene_runtime(state);
    let programs = ensure_wgpu_bake_programs(state);
    let source_bind_group = crate::host_value::<()>("host.createBindGroup");
    let irradiance = bake_wgpu_irradiance(state, &programs, source_bind_group);
    let prefiltered = bake_wgpu_prefiltered(state, &programs, source_bind_group);
    let brdf = (scene.ibl.as_ref().map(|value| (value.brdf_lut).clone()))
        .unwrap_or(bake_wgpu_brdf_lut(state, &programs));
    let brdf_view = (scene
        .ibl
        .as_ref()
        .map(|value| (value.brdf_lut_view).clone()))
    .unwrap_or(crate::host_value::<crate::OpaqueHostValue>(
        "host.createView",
    ));
    let ibl: WgpuSceneIbl = WgpuSceneIbl {
        __flight_identity: std::sync::Arc::new(()),
        brdf_lut: (brdf).clone(),
        brdf_lut_view: (brdf_view).clone(),
        intensity: environment.intensity,
        irradiance_cube: (irradiance.texture).clone(),
        irradiance_cube_view: (irradiance.view).clone(),
        prefiltered_cube: (prefiltered.texture).clone(),
        prefiltered_cube_view: (prefiltered.view).clone(),
        prefiltered_mip_count: PREFILTERED_MIPS,
    };
    scene.ibl = Some((ibl).clone());
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:59 (sha256:552d88f45f17bafdd3933207591dab5d42d0bdc71f91f9aff25a27b481c2d1bb)
pub fn destroy_wgpu_scene_ibl(state: &mut WgpuRenderState) -> () {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.ibl).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        crate::host_value::<()>("host.destroy");
        crate::host_value::<()>("host.destroy");
        scene.ibl = None;
    }
    if ((scene.environment_source_cube).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        scene.environment_source_cube = None;
        scene.environment_source_cube_view = None;
    }
    if ((scene.ibl_uniform_buffer).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        scene.ibl_uniform_buffer = None;
    }
    if ((scene.ibl_dummy_cube_texture).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        scene.ibl_dummy_cube_texture = None;
        scene.ibl_dummy_cube_view = None;
    }
    if ((scene.ibl_dummy_lut_texture).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        scene.ibl_dummy_lut_texture = None;
        scene.ibl_dummy_lut_view = None;
    }
    scene.ibl_sampler = None;
    scene.ibl_sample_layout = None;
    scene.ibl_sample_bind_group = None;
    scene.ibl_sample_cube_view = None;
    scene.pbr_sample_bind_group = None;
    scene.pbr_sample_ibl_cube_view = None;
    {
        let __flight_key = (*state).clone();
        if let Some(__flight_index) = (*_BAKE_PROGRAMS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_BAKE_PROGRAMS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:96 (sha256:ec3f88971ce93f3442e77e1a49bbd45397522cefe8a3ca639e6fd0734824efe3)
#[derive(Clone, Default)]
struct WgpuBakedCube {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for WgpuBakedCube {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:101 (sha256:0950bd4cd6393e989ec2672590b5f2b69e2824f91bb4c7ef5529516d10a3d5c3)
#[derive(Clone, Default)]
struct BakeWgpuIrradianceRecord5 {
    __flight_identity: std::sync::Arc<()>,
    dimension: String,
}
impl PartialEq for BakeWgpuIrradianceRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn bake_wgpu_irradiance(
    state: &WgpuRenderState,
    programs: &WgpuBakePrograms,
    source_bind_group: crate::OpaqueHostValue,
) -> WgpuBakedCube {
    let texture = create_wgpu_bake_cube(state, IRRADIANCE_SIZE, 1.0_f64);
    render_wgpu_bake_cube_faces(
        state,
        (programs.irradiance_pipeline).clone(),
        programs,
        (texture).clone(),
        IRRADIANCE_SIZE,
        0.0_f64,
        0.0_f64,
        (source_bind_group).clone(),
    );
    return WgpuBakedCube {
        __flight_identity: std::sync::Arc::new(()),
        texture: (texture).clone(),
        view: crate::host_value::<crate::OpaqueHostValue>("host.createView"),
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:120 (sha256:bbe1bde6bb6d3b3165193e523c63ec093f5a07c2a96da590298ed3a477f053f2)
#[derive(Clone, Default)]
struct BakeWgpuPrefilteredRecord5 {
    __flight_identity: std::sync::Arc<()>,
    dimension: String,
}
impl PartialEq for BakeWgpuPrefilteredRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn bake_wgpu_prefiltered(
    state: &WgpuRenderState,
    programs: &WgpuBakePrograms,
    source_bind_group: crate::OpaqueHostValue,
) -> WgpuBakedCube {
    let texture = create_wgpu_bake_cube(state, PREFILTERED_SIZE, PREFILTERED_MIPS);
    {
        let mut mip = 0.0_f64;
        while (mip < PREFILTERED_MIPS) {
            let mip_size = (1.0_f64).max(
                (__flight_js_to_i32(PREFILTERED_SIZE) >> (__flight_js_to_u32(mip) & 31)) as f64,
            );
            let roughness = if (PREFILTERED_MIPS > 1.0_f64) {
                (mip / (PREFILTERED_MIPS - 1.0_f64))
            } else {
                0.0_f64
            };
            render_wgpu_bake_cube_faces(
                state,
                (programs.prefiltered_pipeline).clone(),
                programs,
                (texture).clone(),
                mip_size,
                mip,
                roughness,
                (source_bind_group).clone(),
            );
            {
                mip += 1.0;
                mip
            };
        }
    }
    return WgpuBakedCube {
        __flight_identity: std::sync::Arc::new(()),
        texture: (texture).clone(),
        view: crate::host_value::<crate::OpaqueHostValue>("host.createView"),
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:143 (sha256:0d5604ff3a54f79656dd3fe3b72f71948998fadad1f954b2727217491fc5cbf9)
fn bake_wgpu_brdf_lut(
    state: &WgpuRenderState,
    programs: &WgpuBakePrograms,
) -> crate::OpaqueHostValue {
    let device = (state.device).clone();
    let texture = crate::host_value::<()>("host.createTexture");
    let encoder = crate::host_value::<()>("host.createCommandEncoder");
    let pass = crate::host_value::<()>("host.beginRenderPass");
    crate::host_value::<()>("host.setPipeline");
    crate::host_value::<()>("host.draw");
    crate::host_value::<()>("host.end");
    crate::host_value::<()>("host.submit");
    return texture;
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:164 (sha256:7d2837d3d2d74b3565f7e873fca363a39e7d11ee7abc264580fd09b0e66aa433)
fn render_wgpu_bake_cube_faces(
    state: &WgpuRenderState,
    pipeline: crate::OpaqueHostValue,
    programs: &WgpuBakePrograms,
    cube: crate::OpaqueHostValue,
    size: f64,
    mip_level: f64,
    roughness: f64,
    source_bind_group: crate::OpaqueHostValue,
) -> () {
    let device = (state.device).clone();
    {
        let mut face = 0.0_f64;
        while (face < 6.0_f64) {
            let b = CUBE_FACE_BASIS[face as usize].clone();
            (*_BAKE_SCRATCH.lock().unwrap())[0.0_f64 as usize] =
                (b[0.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[1.0_f64 as usize] =
                (b[1.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[2.0_f64 as usize] =
                (b[2.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[3.0_f64 as usize] = (0.0_f64) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[4.0_f64 as usize] =
                (b[3.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[5.0_f64 as usize] =
                (b[4.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[6.0_f64 as usize] =
                (b[5.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[7.0_f64 as usize] = (0.0_f64) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[8.0_f64 as usize] =
                (b[6.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[9.0_f64 as usize] =
                (b[7.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[10.0_f64 as usize] =
                (b[8.0_f64 as usize].clone()) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[11.0_f64 as usize] = (0.0_f64) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[12.0_f64 as usize] = (roughness) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[13.0_f64 as usize] = (0.0_f64) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[14.0_f64 as usize] = (0.0_f64) as f32;
            (*_BAKE_SCRATCH.lock().unwrap())[15.0_f64 as usize] = (0.0_f64) as f32;
            crate::host_value::<()>("host.writeBuffer");
            let mut view = crate::host_value::<()>("host.createView");
            let encoder = crate::host_value::<()>("host.createCommandEncoder");
            let pass = crate::host_value::<()>("host.beginRenderPass");
            crate::host_value::<()>("host.setViewport");
            crate::host_value::<()>("host.setPipeline");
            crate::host_value::<()>("host.setBindGroup");
            crate::host_value::<()>("host.setBindGroup");
            crate::host_value::<()>("host.draw");
            crate::host_value::<()>("host.end");
            crate::host_value::<()>("host.submit");
            {
                view.base_array_layer += 1.0;
                view.base_array_layer
            };
        }
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:220 (sha256:d016feb23c7a03eae2c98db67eac3ba82faf51955398b7cb5ee237139d14b4d5)
fn create_wgpu_bake_cube(state: &WgpuRenderState, size: f64, mips: f64) -> crate::OpaqueHostValue {
    return crate::host_value::<crate::OpaqueHostValue>("host.createTexture");
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:229 (sha256:3f78fb6a42021e5ff75165e71ae799c7102b4ad094161a14b8aa4ae399e6bad4)
#[derive(Clone, Default)]
struct WgpuBakePrograms {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brdf_pipeline: crate::OpaqueHostValue,
    pub irradiance_pipeline: crate::OpaqueHostValue,
    pub prefiltered_pipeline: crate::OpaqueHostValue,
    pub sampler: crate::OpaqueHostValue,
    pub source_bind_group_layout: crate::OpaqueHostValue,
    pub uniform_bind_group: crate::OpaqueHostValue,
    pub uniform_buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuBakePrograms {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:242 (sha256:8215bc7bc6dc244ac7e36ac6f6e41a1c189e8f88529245b986c671d23368b866)
#[derive(Clone, Default)]
struct EnsureWgpuBakeProgramsRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureWgpuBakeProgramsRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuBakeProgramsRecord6 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
    view_dimension: String,
}
impl PartialEq for EnsureWgpuBakeProgramsRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuBakeProgramsRecord7 {
    __flight_identity: std::sync::Arc<()>,
    topology: String,
}
impl PartialEq for EnsureWgpuBakeProgramsRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuBakeProgramsRecord8 {
    __flight_identity: std::sync::Arc<()>,
    mag_filter: String,
    min_filter: String,
}
impl PartialEq for EnsureWgpuBakeProgramsRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_wgpu_bake_programs(state: &WgpuRenderState) -> WgpuBakePrograms {
    let mut programs = (*_BAKE_PROGRAMS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (programs).is_some() {
        return ((programs.as_mut().unwrap()).clone()).clone();
    }
    let device = (state.device).clone();
    let uniform_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let source_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let cube_layout = crate::host_value::<()>("host.createPipelineLayout");
    let brdf_layout = crate::host_value::<()>("host.createPipelineLayout");
    let irradiance_module = crate::host_value::<()>("host.createShaderModule");
    let prefiltered_module = crate::host_value::<()>("host.createShaderModule");
    let brdf_module = crate::host_value::<()>("host.createShaderModule");
    let uniform_buffer = crate::host_value::<()>("host.createBuffer");
    programs = Some(WgpuBakePrograms {
        __flight_identity: std::sync::Arc::new(()),
        brdf_pipeline: crate::host_value::<crate::OpaqueHostValue>("host.createRenderPipeline"),
        irradiance_pipeline: crate::host_value::<crate::OpaqueHostValue>(
            "host.createRenderPipeline",
        ),
        prefiltered_pipeline: crate::host_value::<crate::OpaqueHostValue>(
            "host.createRenderPipeline",
        ),
        sampler: crate::host_value::<crate::OpaqueHostValue>("host.createSampler"),
        source_bind_group_layout: (source_bind_group_layout).clone(),
        uniform_bind_group: crate::host_value::<crate::OpaqueHostValue>("host.createBindGroup"),
        uniform_buffer: (uniform_buffer).clone(),
    });
    {
        let __flight_key = (*state).clone();
        let __flight_value = (programs).clone().unwrap();
        if let Some((_, value)) = (*_BAKE_PROGRAMS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_BAKE_PROGRAMS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((programs).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:300 (sha256:5e89e94a1bc7ae843942497522c48ebed2208dd7344e31082823e3fdf8ea423e)
const IRRADIANCE_SIZE: f64 = 16.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:301 (sha256:4ebb079b70adb55fb4d0920d186d68cb1b8cfb745de25a5811540369b2a4150b)
const PREFILTERED_SIZE: f64 = 64.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:302 (sha256:7c822a5cabc4a4fb565f49e0b2392dce59bffdde9b41c00645869bd4e7417a17)
const PREFILTERED_MIPS: f64 = 5.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:303 (sha256:6d44ae039a3168e1b77d4f527c615f1a45e614de6ffb746d3fd19a35a646634b)
const BRDF_LUT_SIZE: f64 = 128.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:307 (sha256:10127bad98d5f2ac3a1c3884378f629f00b600fd9ca3bf7cd13b50404248ec75)
const IBL_BAKE_FORMAT: &'static str = "rgba16float";

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:310 (sha256:d4ac9e855ab7d61f6e2e589a9f3742aaced52ff9496a005fd5863f87e559933a)
const BAKE_UNIFORM_BYTES: f64 = 64.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:312 (sha256:59bd8664078b026066249658e33b02aea0915432be821c81eaf2901d22128a72)
#[derive(Clone, Default)]
struct BakeClear {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub a: f64,
    pub b: f64,
    pub g: f64,
    pub r: f64,
}
impl PartialEq for BakeClear {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

static BAKE_CLEAR: std::sync::LazyLock<BakeClear> = std::sync::LazyLock::new(|| BakeClear {
    __flight_identity: std::sync::Arc::new(()),
    r: 0.0_f64,
    g: 0.0_f64,
    b: 0.0_f64,
    a: 1.0_f64,
});

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:317 (sha256:3aafcf7c2b93fc31e9366bd44f7a59804c80950a4aeb40c29c1682a914b92d9c)
static CUBE_FACE_BASIS: std::sync::LazyLock<Vec<Vec<f64>>> = std::sync::LazyLock::new(|| {
    vec![
        vec![
            1.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
        vec![
            (-1.0_f64),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
        vec![
            0.0_f64, 1.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
        ],
        vec![
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
            1.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
        ],
        vec![
            0.0_f64,
            0.0_f64,
            1.0_f64,
            1.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
        vec![
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            (-1.0_f64),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
    ]
});

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:326 (sha256:950a0a90b107e7afb243b6b7ea332d1bfd9761cb41168545d033fd8804fa23af)
static _BAKE_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (BAKE_UNIFORM_BYTES / 4.0_f64) as usize])
    });

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:327 (sha256:51c97a39351e4655974472533d980e9f8434bda67baac7fead463dbff1341dcc)
static _BAKE_PROGRAMS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(WgpuRenderState, WgpuBakePrograms)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:335 (sha256:92cf6e5d075bbb847257c24fbfe153159ecb0895137e7166be688cdef0f203b9)
const BAKE_VERTEX_WGSL: &'static str = "\nstruct FaceUniform {\n  faceForward : vec4f,\n  faceRight : vec4f,\n  faceUp : vec4f,\n  roughness : vec4f,   // x = roughness (prefiltered only)\n};\n@group(0) @binding(0) var<uniform> face : FaceUniform;\n\nstruct VertexOutput {\n  @builtin(position) clipPosition : vec4f,\n  @location(0) uv : vec2f,\n};\n\n@vertex fn vs_main(@builtin(vertex_index) vi : u32) -> VertexOutput {\n  var out : VertexOutput;\n  let x = f32((vi & 1u) << 2u) - 1.0;\n  let y = f32((vi & 2u) << 1u) - 1.0;\n  out.uv = vec2f(x, -y);\n  out.clipPosition = vec4f(x, y, 0.0, 1.0);\n  return out;\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:361 (sha256:75be3771a13726146b407048bb5d573a08287614e98ff1f134e17cb7352ef4cb)
const BAKE_COMMON_WGSL: &'static str = "\nconst PI : f32 = 3.14159265359;\n\n@group(1) @binding(0) var envCube : texture_cube<f32>;\n@group(1) @binding(1) var envSampler : sampler;\n\nfn faceDirection(uv : vec2f) -> vec3f {\n  return normalize(face.faceForward.xyz + uv.x * face.faceRight.xyz + uv.y * face.faceUp.xyz);\n}\n\nfn srgbToLinear(c : vec3f) -> vec3f {\n  let lo = c / 12.92;\n  let hi = pow((c + vec3f(0.055)) / 1.055, vec3f(2.4));\n  return select(lo, hi, c > vec3f(0.04045));\n}\n\nfn radicalInverse(bitsIn : u32) -> f32 {\n  var bits = bitsIn;\n  bits = (bits << 16u) | (bits >> 16u);\n  bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);\n  bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);\n  bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);\n  bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);\n  return f32(bits) * 2.3283064365386963e-10;\n}\n\nfn hammersley(i : u32, n : u32) -> vec2f {\n  return vec2f(f32(i) / f32(n), radicalInverse(i));\n}\n\nfn importanceSampleGGX(Xi : vec2f, N : vec3f, roughness : f32) -> vec3f {\n  let a = roughness * roughness;\n  let phi = 2.0 * PI * Xi.x;\n  let cosTheta = sqrt((1.0 - Xi.y) / (1.0 + (a * a - 1.0) * Xi.y));\n  let sinTheta = sqrt(1.0 - cosTheta * cosTheta);\n  let H = vec3f(cos(phi) * sinTheta, sin(phi) * sinTheta, cosTheta);\n  var up = vec3f(0.0, 0.0, 1.0);\n  if (abs(N.z) >= 0.999) {\n    up = vec3f(1.0, 0.0, 0.0);\n  }\n  let tangent = normalize(cross(up, N));\n  let bitangent = cross(N, tangent);\n  return normalize(tangent * H.x + bitangent * H.y + N * H.z);\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:408 (sha256:80065959cebdd2fe881614349c327e477daaa6e680608beb0d91f02135a6cdb2)
const IRRADIANCE_FRAGMENT_WGSL: f64 = (BAKE_COMMON_WGSL
    + "\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  let N = faceDirection(in.uv);\n  var up = vec3f(0.0, 0.0, 1.0);\n  if (abs(N.z) >= 0.999) {\n    up = vec3f(1.0, 0.0, 0.0);\n  }\n  let right = normalize(cross(up, N));\n  let realUp = normalize(cross(N, right));\n\n  var irradiance = vec3f(0.0);\n  var samples = 0.0;\n  let delta = 0.15;\n  var phi = 0.0;\n  loop {\n    if (phi >= 2.0 * PI) { break; }\n    var theta = 0.0;\n    loop {\n      if (theta >= 0.5 * PI) { break; }\n      let tangent = vec3f(sin(theta) * cos(phi), sin(theta) * sin(phi), cos(theta));\n      let sampleVec = tangent.x * right + tangent.y * realUp + tangent.z * N;\n      irradiance = irradiance + srgbToLinear(textureSampleLevel(envCube, envSampler, sampleVec, 0.0).rgb) * cos(theta) * sin(theta);\n      samples = samples + 1.0;\n      theta = theta + delta;\n    }\n    phi = phi + delta;\n  }\n  return vec4f(PI * irradiance / samples, 1.0);\n}\n");

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:442 (sha256:ef70f76cdf093a3d49117ec29f3927bd95c925f2c080884c8c04cb5502973788)
const PREFILTERED_FRAGMENT_WGSL: f64 = (BAKE_COMMON_WGSL
    + "\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  let N = faceDirection(in.uv);\n  let V = N;\n  let SAMPLE_COUNT = 48u;\n  var prefiltered = vec3f(0.0);\n  var totalWeight = 0.0;\n  for (var i = 0u; i < SAMPLE_COUNT; i = i + 1u) {\n    let Xi = hammersley(i, SAMPLE_COUNT);\n    let H = importanceSampleGGX(Xi, N, face.roughness.x);\n    let L = normalize(2.0 * dot(V, H) * H - V);\n    let nDotL = max(dot(N, L), 0.0);\n    if (nDotL > 0.0) {\n      prefiltered = prefiltered + srgbToLinear(textureSampleLevel(envCube, envSampler, L, 0.0).rgb) * nDotL;\n      totalWeight = totalWeight + nDotL;\n    }\n  }\n  if (totalWeight > 0.0) {\n    return vec4f(prefiltered / totalWeight, 1.0);\n  }\n  return vec4f(srgbToLinear(textureSampleLevel(envCube, envSampler, N, 0.0).rgb), 1.0);\n}\n");

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts:471 (sha256:f7cb00021dc43f572b93e1c11fc5848e5a39a6a3da2f8cc9e48e0dd7cfdf550e)
const BRDF_LUT_FRAGMENT_WGSL: &'static str = "\nconst PI : f32 = 3.14159265359;\n\nfn radicalInverse(bitsIn : u32) -> f32 {\n  var bits = bitsIn;\n  bits = (bits << 16u) | (bits >> 16u);\n  bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);\n  bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);\n  bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);\n  bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);\n  return f32(bits) * 2.3283064365386963e-10;\n}\n\nfn hammersley(i : u32, n : u32) -> vec2f {\n  return vec2f(f32(i) / f32(n), radicalInverse(i));\n}\n\nfn importanceSampleGGX(Xi : vec2f, N : vec3f, roughness : f32) -> vec3f {\n  let a = roughness * roughness;\n  let phi = 2.0 * PI * Xi.x;\n  let cosTheta = sqrt((1.0 - Xi.y) / (1.0 + (a * a - 1.0) * Xi.y));\n  let sinTheta = sqrt(1.0 - cosTheta * cosTheta);\n  let H = vec3f(cos(phi) * sinTheta, sin(phi) * sinTheta, cosTheta);\n  var up = vec3f(0.0, 0.0, 1.0);\n  if (abs(N.z) >= 0.999) {\n    up = vec3f(1.0, 0.0, 0.0);\n  }\n  let tangent = normalize(cross(up, N));\n  let bitangent = cross(N, tangent);\n  return normalize(tangent * H.x + bitangent * H.y + N * H.z);\n}\n\nfn geometrySchlickGGX(nDotV : f32, roughness : f32) -> f32 {\n  let k = roughness * roughness / 2.0;\n  return nDotV / (nDotV * (1.0 - k) + k);\n}\n\nfn geometrySmith(N : vec3f, V : vec3f, L : vec3f, roughness : f32) -> f32 {\n  return geometrySchlickGGX(max(dot(N, L), 0.0), roughness) * geometrySchlickGGX(max(dot(N, V), 0.0), roughness);\n}\n\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  let uv = in.uv * 0.5 + vec2f(0.5);\n  let nDotV = max(uv.x, 0.001);\n  let roughness = uv.y;\n  let V = vec3f(sqrt(1.0 - nDotV * nDotV), 0.0, nDotV);\n  let N = vec3f(0.0, 0.0, 1.0);\n  var A = 0.0;\n  var B = 0.0;\n  let SAMPLE_COUNT = 256u;\n  for (var i = 0u; i < SAMPLE_COUNT; i = i + 1u) {\n    let Xi = hammersley(i, SAMPLE_COUNT);\n    let H = importanceSampleGGX(Xi, N, roughness);\n    let L = normalize(2.0 * dot(V, H) * H - V);\n    let nDotL = max(L.z, 0.0);\n    let nDotH = max(H.z, 0.0);\n    let vDotH = max(dot(V, H), 0.0);\n    if (nDotL > 0.0) {\n      let G = geometrySmith(N, V, L, roughness);\n      let gVis = (G * vDotH) / (nDotH * nDotV);\n      let Fc = pow(1.0 - vDotH, 5.0);\n      A = A + (1.0 - Fc) * gVis;\n      B = B + Fc * gVis;\n    }\n  }\n  return vec4f(A / f32(SAMPLE_COUNT), B / f32(SAMPLE_COUNT), 0.0, 1.0);\n}\n";
