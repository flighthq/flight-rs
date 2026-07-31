// @generated from upstream/packages/scene-gl/src/enableGlSceneCustomShaderGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene_runtime;
use flighthq_log::log_once;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, LogData, LogDataProvider,
    LogLevel, Matrix, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace,
    TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/enableGlSceneCustomShaderGuards.ts:8 (sha256:f08cc56ea918dbd1bc5629d24a5f656d529d49f7c5f29485707585a1cd6d8809)
pub fn are_gl_scene_custom_shader_guards_enabled(state: &mut GlRenderState) -> bool {
    return ((get_gl_scene_runtime(state).custom_shader_guard).clone()).is_some();
}

// Source: upstream/packages/scene-gl/src/enableGlSceneCustomShaderGuards.ts:19 (sha256:cdefdc921735a54de977bb86fe4ae4d40176e923991e8e56d3acface5498511a)
pub fn enable_gl_scene_custom_shader_guards(state: &mut GlRenderState) -> () {
    get_gl_scene_runtime(state).custom_shader_guard =
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: GlRenderState,
                  __flight_argument_1: crate::OpaqueHostValue,
                  __flight_argument_2: String|
                  -> () {
                warn_gl_custom_shader_uniform_types(
                    &__flight_argument_0,
                    (__flight_argument_1).clone(),
                    (__flight_argument_2).clone(),
                )
            },
        )
            as Box<
                dyn FnMut(GlRenderState, crate::OpaqueHostValue, String) -> () + Send + 'static,
            >)));
}

// Source: upstream/packages/scene-gl/src/enableGlSceneCustomShaderGuards.ts:26 (sha256:808160dc7f974fbaf3b6981530a99dbab07ef7020b201880639ce732a3d40e9c)
fn gl_uniform_type_name(gl: crate::OpaqueHostValue, type_: f64) -> String {
    {
        let __switch_value = type_;
        let __flight_case = if __switch_value
            == crate::host_value::<crate::OpaqueHostValue>("host.FLOAT_MAT4")
        {
            0_usize
        } else if __switch_value == crate::host_value::<crate::OpaqueHostValue>("host.FLOAT_MAT3") {
            1_usize
        } else if __switch_value == crate::host_value::<crate::OpaqueHostValue>("host.FLOAT_MAT2") {
            2_usize
        } else if __switch_value == crate::host_value::<crate::OpaqueHostValue>("host.FLOAT_VEC4") {
            3_usize
        } else if __switch_value == crate::host_value::<crate::OpaqueHostValue>("host.FLOAT_VEC3") {
            4_usize
        } else if __switch_value == crate::host_value::<crate::OpaqueHostValue>("host.FLOAT_VEC2") {
            5_usize
        } else if __switch_value == crate::host_value::<crate::OpaqueHostValue>("host.FLOAT") {
            6_usize
        } else {
            7_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return "mat4".to_owned();
            }
            if __flight_case <= 1_usize {
                return "mat3".to_owned();
            }
            if __flight_case <= 2_usize {
                return "mat2".to_owned();
            }
            if __flight_case <= 3_usize {
                return "vec4".to_owned();
            }
            if __flight_case <= 4_usize {
                return "vec3".to_owned();
            }
            if __flight_case <= 5_usize {
                return "vec2".to_owned();
            }
            if __flight_case <= 6_usize {
                return "float".to_owned();
            }
            if __flight_case <= 7_usize {
                return format!("gl-type-{}", type_);
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}

// Source: upstream/packages/scene-gl/src/enableGlSceneCustomShaderGuards.ts:52 (sha256:34b498ad429e18c0d9d598b7cec608419a0ce74432b67ab8df66f73ac33d06ee)
fn warn_gl_custom_shader_uniform_types(
    state: &GlRenderState,
    program: crate::OpaqueHostValue,
    shader_key: String,
) -> () {
    if (*_CHECKED_PROGRAMS.lock().unwrap())
        .iter()
        .any(|item| item == &(program).clone())
    {
        return;
    }
    {
        let __flight_value = (program).clone();
        if !(*_CHECKED_PROGRAMS.lock().unwrap()).contains(&__flight_value) {
            (*_CHECKED_PROGRAMS.lock().unwrap()).push(__flight_value);
        }
    };
    let gl = (state.gl).clone();
    let expected: Vec<(String, f64)> = Vec::new();
    let count = crate::host_value::<f64>("host.getProgramParameter");
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let info = crate::host_value::<()>("host.getActiveUniform");
            if (info).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let want = expected
                .iter()
                .find(|(key, _)| key == &crate::host_value::<String>("host.name"))
                .map(|(_, value)| value.clone());
            if ((want).is_none()) || (crate::host_value::<Option<f64>>("host.type") == want) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            log_once(
                format!(
                    "scene-gl:custom-shader-uniform-type:{}:{}",
                    shader_key,
                    crate::host_value::<crate::OpaqueHostValue>("host.name")
                ),
                LogLevel::Warn,
                &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                    String,
                    Vec<(String, crate::OpaqueHostValue)>,
                >::B({
                    let mut __flight_record = Vec::new();
                    __flight_record.push(("message".to_owned(), format!("customShaderGlMeshMaterialRenderer: shader \"{}\" declares {} as {} but the renderer uploads it as {} — the mismatched upload raises a silent GL_INVALID_OPERATION and the draw is dropped. Declare '{} {}' in the shader.", shader_key, crate::host_value::<crate::OpaqueHostValue>("host.name"), gl_uniform_type_name((gl).clone(), crate::host_value::<f64>("host.type")), gl_uniform_type_name((gl).clone(), (want).clone().unwrap()), gl_uniform_type_name((gl).clone(), (want).clone().unwrap()), crate::host_value::<crate::OpaqueHostValue>("host.name"))));
                    __flight_record
                }))),
                Some(("scene-gl".to_owned()).clone()),
            );
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/scene-gl/src/enableGlSceneCustomShaderGuards.ts:81 (sha256:47bc38d186a1cdc7d2b45cf90ffe094fd6032eb8e9128ccd6573cda2c54fd670)
static _CHECKED_PROGRAMS: std::sync::LazyLock<std::sync::Mutex<Vec<crate::OpaqueHostValue>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
