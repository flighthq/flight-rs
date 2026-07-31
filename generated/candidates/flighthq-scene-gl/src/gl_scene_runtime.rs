// @generated from upstream/packages/scene-gl/src/glSceneRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlMeshProgram, destroy_gl_bake_programs};
use flighthq_render_gl::{
    create_gl_skin_palette_texture, destroy_gl_render_target, destroy_gl_skin_palette_texture,
};
use flighthq_shading::ModifierRegistry;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlMeshMaterialRenderer, GlRenderState, GlRenderTarget,
    GlSkinPaletteTexture, ImageResource, Kind, Matrix, Matrix4, MeshGeometry, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:20 (sha256:a3e0e95fd5a16eb6979c41b4e0a96ba37fb0a9b4fa99721eff788393b5e8783b)
#[derive(Clone, Default)]
pub struct GlSceneShadow {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub matrix: Matrix4,
    pub texture: crate::OpaqueHostValue,
}
impl PartialEq for GlSceneShadow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:30 (sha256:4ff646e7b7211f9bd9129dbc7f4e6189aa652c989d436fdef16f2d87003cb443)
#[derive(Clone, Default)]
pub struct GlSceneIbl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brdf_lut: crate::OpaqueHostValue,
    pub intensity: f64,
    pub irradiance_cube: crate::OpaqueHostValue,
    pub prefiltered_cube: crate::OpaqueHostValue,
    pub prefiltered_mip_count: f64,
}
impl PartialEq for GlSceneIbl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:41 (sha256:ad777585e24c1477873b89048b263224a871ab3ecbc91cd4c52cbad2420b41b6)
#[derive(Clone, Default)]
pub struct GlSceneDrawEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub clip_w: f64,
    pub material: crate::OpaqueHostValue,
    pub mesh: crate::OpaqueHostValue,
    pub normal_matrix: crate::OpaqueHostValue,
    pub renderer: crate::OpaqueHostValue,
    pub subset: crate::OpaqueHostValue,
    pub world_matrix: crate::OpaqueHostValue,
}
impl PartialEq for GlSceneDrawEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:67 (sha256:a606d45f5010f9664e525cd39596065512d1ba41ed4058c18a256debc8e04119)
#[derive(Clone, Default)]
pub struct GlSceneRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub active_mesh_program: Option<GlMeshProgram>,
    pub active_skinned_run: bool,
    pub blended_draw_list: Vec<GlSceneDrawEntry>,
    pub blended_pool: Vec<GlSceneDrawEntry>,
    pub color_space_guard:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub custom_shader_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(GlRenderState, crate::OpaqueHostValue, String) -> () + Send + 'static,
                >,
            >,
        >,
    >,
    pub environment_source_cube: Option<crate::OpaqueHostValue>,
    pub ibl: Option<GlSceneIbl>,
    pub ibl_bake_framebuffer: Option<crate::OpaqueHostValue>,
    pub material_registry: Vec<(Kind, GlMeshMaterialRenderer)>,
    pub modifier_snippet_registry: Option<ModifierRegistry>,
    pub opaque_draw_list: Vec<GlSceneDrawEntry>,
    pub opaque_pool: Vec<GlSceneDrawEntry>,
    pub program_cache: Vec<(String, GlMeshProgram)>,
    pub shadow: Option<GlSceneShadow>,
    pub shadow_target: Option<GlRenderTarget>,
    pub skin_palette: Option<GlSkinPaletteTexture>,
    pub time: f64,
    pub upload_cache: Vec<(MeshGeometry, GlMeshUpload)>,
}
impl PartialEq for GlSceneRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:108 (sha256:9a4ac05fe5c79a23a8b02105a79ad7198141e77014e5cf6e2db22a8080291234)
#[derive(Clone, Default)]
pub struct GlMeshUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_buffer: Option<crate::OpaqueHostValue>,
    pub index_count: f64,
    pub index_type: f64,
    pub skin_bind_uploaded: Option<bool>,
    pub vao: crate::OpaqueHostValue,
    pub version: f64,
    pub vertex_buffer: crate::OpaqueHostValue,
}
impl PartialEq for GlMeshUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:132 (sha256:2383b7fcf3356b0c99a0a370fe491e30b29f6d101d2308fb947320082341db2a)
pub fn destroy_gl_scene_runtime(state: &GlRenderState) -> () {
    let mut scene = (*SCENE_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (scene).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    for program in (scene
        .as_mut()
        .unwrap()
        .program_cache
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        crate::host_value::<()>("host.deleteProgram");
    }
    scene.as_mut().unwrap().program_cache.clear();
    scene.as_mut().unwrap().active_mesh_program = None;
    if ((scene.as_mut().unwrap().ibl).clone()).is_some() {
        crate::host_value::<()>("host.deleteTexture");
        crate::host_value::<()>("host.deleteTexture");
        crate::host_value::<()>("host.deleteTexture");
        scene.as_mut().unwrap().ibl = None;
    }
    if ((scene.as_mut().unwrap().ibl_bake_framebuffer).clone()).is_some() {
        crate::host_value::<()>("host.deleteFramebuffer");
        scene.as_mut().unwrap().ibl_bake_framebuffer = None;
    }
    if ((scene.as_mut().unwrap().environment_source_cube).clone()).is_some() {
        crate::host_value::<()>("host.deleteTexture");
        scene.as_mut().unwrap().environment_source_cube = None;
    }
    destroy_gl_bake_programs(state);
    if ((scene.as_mut().unwrap().shadow_target).clone()).is_some() {
        destroy_gl_render_target(
            state,
            scene.as_mut().unwrap().shadow_target.as_ref().unwrap(),
        );
        scene.as_mut().unwrap().shadow_target = None;
    }
    scene.as_mut().unwrap().shadow = None;
    if ((scene.as_mut().unwrap().skin_palette).clone()).is_some() {
        destroy_gl_skin_palette_texture(
            (gl).clone(),
            scene.as_mut().unwrap().skin_palette.as_ref().unwrap(),
        );
        scene.as_mut().unwrap().skin_palette = None;
    }
    scene.as_mut().unwrap().blended_draw_list.clear();
    scene.as_mut().unwrap().opaque_draw_list.clear();
    scene.as_mut().unwrap().blended_pool.clear();
    scene.as_mut().unwrap().opaque_pool.clear();
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:177 (sha256:668cca329924b49d3148f0c9c645d606f2d82295e8637130cea9924ed667126d)
pub fn ensure_gl_skin_palette(state: &mut GlRenderState) -> GlSkinPaletteTexture {
    let mut scene = get_gl_scene_runtime(state);
    let mut palette = (scene.skin_palette).clone();
    if (palette).is_none() {
        palette = Some(create_gl_skin_palette_texture((state.gl).clone()));
        scene.skin_palette = (palette).clone();
    }
    return ((palette).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:190 (sha256:c8827ecb2678e6fa1e2c850282684ae95951f53987b90e43d1a3f1c29218b981)
pub fn get_gl_scene_runtime(state: &mut GlRenderState) -> GlSceneRuntime {
    let mut state_runtime = ({
        let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(state)
            .lock()
            .unwrap()
            .clone()
            .expect("entity runtime was read before initialization");
        __flight_runtime
    })
    .clone();
    let mut scene = (*SCENE_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (scene).is_none() {
        scene = Some(GlSceneRuntime {
            __flight_identity: std::sync::Arc::new(()),
            active_mesh_program: None,
            active_skinned_run: false,
            blended_draw_list: vec![],
            blended_pool: vec![],
            environment_source_cube: None,
            ibl: None,
            ibl_bake_framebuffer: None,
            material_registry: Vec::new(),
            modifier_snippet_registry: None,
            opaque_draw_list: vec![],
            opaque_pool: vec![],
            program_cache: Vec::new(),
            shadow: None,
            shadow_target: None,
            skin_palette: None,
            time: 0.0_f64,
            upload_cache: Vec::new(),
            color_space_guard: None,
            custom_shader_guard: None,
        });
        {
            let __flight_key = (*state).clone();
            let __flight_value = (scene).clone().unwrap();
            if let Some((_, value)) = (*SCENE_RUNTIMES.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*SCENE_RUNTIMES.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
        {
            let __flight_runtime = state_runtime;
            let __flight_value = Some((scene.as_mut().unwrap().material_registry).clone());
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage
                .gl_render_state_runtime
                .scene_mesh_material_registry = __flight_value;
        };
        {
            let __flight_runtime = state_runtime;
            let __flight_value = Some(crate::host_value::<
                Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
            >("host.cast"));
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.scene_mesh_upload_cache = __flight_value;
        };
    }
    return ((scene).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-gl/src/glSceneRuntime.ts:222 (sha256:47c502a26288b251f194fcff26d76473ad10f15efd6c4e66a4247dccf590b19e)
static SCENE_RUNTIMES: std::sync::LazyLock<std::sync::Mutex<Vec<(GlRenderState, GlSceneRuntime)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
