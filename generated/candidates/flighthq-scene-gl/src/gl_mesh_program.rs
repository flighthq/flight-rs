// @generated from upstream/packages/scene-gl/src/glMeshProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ensure_gl_mesh_upload, ensure_gl_skin_palette, get_gl_scene_runtime};
use flighthq_camera::get_camera_view_projection_matrix4;
use flighthq_geometry::{create_matrix3, create_matrix4, get_matrix4_position, inverse_matrix4};
use flighthq_render_gl::{create_gl_program, upload_gl_skin_palette_texture};
use flighthq_texture::{get_texture_uv_matrix, has_texture_uv_transform};
use flighthq_types::{
    BlendMode, Camera, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, Matrix3,
    Matrix4, Matrix4Like, MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneRenderProxy,
    SceneResourceRef, TextureColorSpace, TextureFilter, TextureLike, TextureWrap, Vector2, Vector3,
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

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:16 (sha256:4028b1b1abd6258f7e1ac181ea05fbd861d49da56a0fe4a1b824566f9034e83e)
#[derive(Clone, Default)]
pub struct GlMeshProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
}
impl PartialEq for GlMeshProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:44 (sha256:1991da931b24d1e34f9c6310cd58bf80122a6d9ebf066d812f397d120751be79)
pub fn begin_gl_mesh_draw(
    state: &mut GlRenderState,
    program: &GlMeshProgram,
    double_sided: bool,
) -> () {
    let gl = (state.gl).clone();
    get_gl_scene_runtime(state).active_mesh_program = Some((*program).clone());
    crate::host_value::<()>("host.useProgram");
    crate::host_value::<()>("host.enable");
    crate::host_value::<()>("host.depthFunc");
    crate::host_value::<()>("host.depthMask");
    if double_sided {
        crate::host_value::<()>("host.disable");
    } else {
        crate::host_value::<()>("host.enable");
        crate::host_value::<()>("host.cullFace");
    }
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:67 (sha256:2ac1acc64f92d278e866fd89a14e9e7d359f4683de4da33da6a2729ee01281cb)
pub fn bind_gl_uv_transform(
    gl: crate::OpaqueHostValue,
    program: &mut GlMeshProgram,
    texture: Option<TextureLike>,
) -> () {
    let mut loc = (program.loc_uv_transform).clone();
    if (loc).is_none() {
        loc = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.getUniformLocation",
        ));
        (*program).clone().loc_uv_transform = (loc).clone();
    }
    if ((loc).is_none()) || ((texture).is_none()) {
        return;
    }
    get_texture_uv_matrix(&mut (*SCRATCH_UV_MATRIX.lock().unwrap()), &texture);
    crate::host_value::<()>("host.uniformMatrix3fv");
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:85 (sha256:814105e9040265ec59347e4749f31bd781b5966862113f4ef792d4635416bc30)
pub fn compile_gl_program(
    gl: crate::OpaqueHostValue,
    vertex_source: String,
    fragment_source: String,
) -> crate::OpaqueHostValue {
    return create_gl_program(
        (gl).clone(),
        (vertex_source).clone(),
        (fragment_source).clone(),
        Some(("Mesh".to_owned()).clone()),
    );
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:97 (sha256:4091e5800bc256caac7be00044c1b9edfc70d8b7e223b22a6acf3ecaded55391)
pub fn destroy_gl_mesh_program(state: &GlRenderState, program: &GlMeshProgram) -> () {
    crate::host_value::<()>("host.deleteProgram");
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:105 (sha256:567e717d5db964990e5a9e3d5b80115c544e3d4e60496e633e935785841fd5f2)
pub fn draw_gl_mesh_subset(
    state: &mut GlRenderState,
    program: &mut GlMeshProgram,
    proxy: &SceneRenderProxy,
    geometry: &mut MeshGeometry,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniformMatrix4fv");
    if ((program.loc_normal_matrix).clone()).is_some() {
        crate::host_value::<()>("host.uniformMatrix3fv");
    }
    let mut loc_object_alpha = (program.loc_object_alpha).clone();
    if (loc_object_alpha).is_none() {
        loc_object_alpha = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.getUniformLocation",
        ));
        (*program).clone().loc_object_alpha = (loc_object_alpha).clone();
    }
    if (loc_object_alpha).is_some() {
        crate::host_value::<()>("host.uniform1f");
    }
    let joint_matrices = (proxy.joint_matrices).clone();
    let gpu_skinned =
        (((program.loc_joint_texture).clone()).is_some()) && ((joint_matrices).is_some());
    if gpu_skinned {
        let mut palette = ensure_gl_skin_palette(state);
        crate::host_value::<()>("host.activeTexture");
        upload_gl_skin_palette_texture(
            (gl).clone(),
            &mut palette,
            joint_matrices.as_ref().unwrap(),
            (__flight_js_to_i32(((joint_matrices.as_ref().unwrap().len() as f64) / 16.0_f64))
                | __flight_js_to_i32(0.0_f64)) as f64,
        );
        crate::host_value::<()>("host.uniform1i");
    }
    let upload = ensure_gl_mesh_upload(state, geometry, Some(gpu_skinned));
    if ((upload.index_buffer).clone()).is_some() {
        let element_size = if (upload.index_type == crate::host_value::<f64>("host.UNSIGNED_INT")) {
            4.0_f64
        } else {
            2.0_f64
        };
        crate::host_value::<()>("host.drawElements");
    } else {
        crate::host_value::<()>("host.drawArrays");
    }
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:157 (sha256:0ea73994c3f371aae5b5254245e5fd1b9d74b7128f478a42d98a05e36e7e8373)
pub fn ensure_gl_scene_program<T: Clone>(
    state: &mut GlRenderState,
    key: String,
    compile: &mut impl FnMut(crate::OpaqueHostValue) -> T,
) -> T {
    let mut runtime = get_gl_scene_runtime(state);
    let mut program = runtime
        .program_cache
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (program).is_none() {
        program = Some(compile((state.gl).clone()));
        {
            let __flight_key = (key).clone();
            let __flight_value = (program).clone().unwrap();
            if let Some((_, value)) = runtime
                .program_cache
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime.program_cache.push((__flight_key, __flight_value));
            }
        };
    }
    return (program).clone().unwrap();
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:175 (sha256:d1d3b2dd0f0cc03c100909c51cc9ad37266b9d1b8760bd6d96fff17dba4102e8)
pub fn has_gl_uv_transform(texture: Option<TextureLike>) -> bool {
    return (((texture).is_some()) && (((texture.as_ref().unwrap().image).clone()).is_some()))
        && (has_texture_uv_transform(&texture));
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:182 (sha256:03b28a3289080bb56ef98380f25d15428f102088ad0d1ff7ce7a22ba858e3459)
pub fn set_gl_mesh_camera_position(
    gl: crate::OpaqueHostValue,
    loc_camera_position: Option<crate::OpaqueHostValue>,
    camera: &Camera,
) -> () {
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
    crate::host_value::<()>("host.uniform3f");
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:195 (sha256:05aed9e3d12b708b326955431aba480623c45a86cd4c216c140e1c76f237a116)
pub fn set_gl_mesh_view_projection(
    gl: crate::OpaqueHostValue,
    loc_view_projection: Option<crate::OpaqueHostValue>,
    camera: &Camera,
) -> () {
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
    crate::host_value::<()>("host.uniformMatrix4fv");
}

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:211 (sha256:a6dc7d15a6fa74a62f77ba73d3aea675675b4ca20400d5c772f6bf771f93a39a)
pub const GL_UV_TRANSFORM_VERTEX_GLSL: &'static str = "\n#ifdef HAS_UV_TRANSFORM\nuniform mat3 u_uvTransform;\nvec2 applyUvTransform(vec2 uv) { return (u_uvTransform * vec3(uv, 1.0)).xy; }\n#else\nvec2 applyUvTransform(vec2 uv) { return uv; }\n#endif\n";

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:223 (sha256:bba7e900335a674af9bb9d38ae7eb9fa2e5560ca9ee7bc9fbd4f6f823ca1c390)
pub const SKIN_PALETTE_TEXTURE_UNIT: f64 = 12.0_f64;

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:233 (sha256:a7d9d67ffd8b62f2dbad61a192fa342e425d59fd51cd9bad9da54de0acb14008)
pub const GL_SKIN_VERTEX_DECLARATIONS_GLSL: &'static str = "\nlayout(location = 6) in vec4 a_joints0;\nlayout(location = 7) in vec4 a_weights0;\nuniform highp sampler2D u_jointTexture;\n\nmat4 fetchJointMatrix(int joint) {\n  int x = joint * 4;\n  return mat4(\n    texelFetch(u_jointTexture, ivec2(x, 0), 0),\n    texelFetch(u_jointTexture, ivec2(x + 1, 0), 0),\n    texelFetch(u_jointTexture, ivec2(x + 2, 0), 0),\n    texelFetch(u_jointTexture, ivec2(x + 3, 0), 0)\n  );\n}\n\nmat4 skinMatrix() {\n  return a_weights0.x * fetchJointMatrix(int(a_joints0.x))\n       + a_weights0.y * fetchJointMatrix(int(a_joints0.y))\n       + a_weights0.z * fetchJointMatrix(int(a_joints0.z))\n       + a_weights0.w * fetchJointMatrix(int(a_joints0.w));\n}\n";

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:256 (sha256:140acb0d499b3786d700284ab9e3540997031e7613e61aec3eb90cf6d2ab88c6)
static SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:257 (sha256:cac89509b1c8459b541129553f50270085b134abaef34130e7309e6463eaf999)
static SCRATCH_INVERSE_VIEW: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:258 (sha256:87ef2e4557c2d0eb9483fdfe75d849407b3f25f82a43c851b3a0ff34fb85a5b9)
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

// Source: upstream/packages/scene-gl/src/glMeshProgram.ts:261 (sha256:591628d13476cd1379f3e3321adebd8dde23da6c434a9b73ccb6259319613fd2)
static SCRATCH_UV_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ))
    });
