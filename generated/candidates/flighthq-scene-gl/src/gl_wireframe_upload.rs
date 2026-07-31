// @generated from upstream/packages/scene-gl/src/glWireframeUpload.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ensure_gl_mesh_upload;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, MeshGeometry, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
    VertexAttribute,
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

// Source: upstream/packages/scene-gl/src/glWireframeUpload.ts:12 (sha256:6aaa4136bcd431697355c53644dbe3b6636296775ee6f35ff740addf61f844ce)
#[derive(Clone, Default)]
pub struct GlWireframeUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_type: f64,
    pub line_index_buffer: crate::OpaqueHostValue,
    pub vao: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for GlWireframeUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glWireframeUpload.ts:30 (sha256:2722692c06a02ad080a04f5bbb764133d119e707e60870ee500aced50ccefd4e)
pub fn destroy_gl_wireframe_upload(state: &GlRenderState, upload: &GlWireframeUpload) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.deleteVertexArray");
    crate::host_value::<()>("host.deleteBuffer");
}

// Source: upstream/packages/scene-gl/src/glWireframeUpload.ts:36 (sha256:8983eb93142ceb6d43234d10e6fab543bdffee1a21529bc60f11f7397453cc0b)
pub fn ensure_gl_wireframe_upload(
    state: &mut GlRenderState,
    geometry: &mut MeshGeometry,
) -> GlWireframeUpload {
    let gl = (state.gl).clone();
    let mesh_upload = ensure_gl_mesh_upload(state, geometry, None);
    let mut per_state = (*WIREFRAME_UPLOADS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (per_state).is_none() {
        per_state = Some(Vec::new());
        {
            let __flight_key = (*state).clone();
            let __flight_value = (per_state).clone().unwrap();
            if let Some((_, value)) = (*WIREFRAME_UPLOADS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*WIREFRAME_UPLOADS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let mut upload = per_state
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(*geometry).clone())
        .map(|(_, value)| value.clone());
    if ((upload).is_some()) && (upload.as_mut().unwrap().version == geometry.version) {
        crate::host_value::<()>("host.bindVertexArray");
        return ((upload).clone().unwrap()).clone();
    }
    let line_indices = build_line_indices(geometry);
    let index_type = if true {
        crate::host_value::<crate::OpaqueHostValue>("host.UNSIGNED_INT")
    } else {
        crate::host_value::<crate::OpaqueHostValue>("host.UNSIGNED_SHORT")
    };
    if (upload).is_none() {
        upload = Some(GlWireframeUpload {
            __flight_identity: std::sync::Arc::new(()),
            index_type: index_type,
            line_index_buffer: crate::host_value::<crate::OpaqueHostValue>("host.createBuffer"),
            vao: crate::host_value::<crate::OpaqueHostValue>("host.createVertexArray"),
            version: (-1.0_f64),
        });
        {
            let __flight_key = (*geometry).clone();
            let __flight_value = (upload).clone().unwrap();
            if let Some((_, value)) = per_state
                .as_mut()
                .unwrap()
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                per_state
                    .as_mut()
                    .unwrap()
                    .push((__flight_key, __flight_value));
            }
        };
    }
    upload.as_mut().unwrap().index_type = index_type;
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.bindBuffer");
    let stride = geometry.layout.stride;
    let position = ((geometry.layout.attributes).clone())
        .iter()
        .find(|value| {
            (|a: VertexAttribute| -> bool { ((a.semantic).clone() == "position") })(
                (*value).clone(),
            )
        })
        .cloned();
    let byte_offset = if (position).is_some() {
        position.as_ref().unwrap().byte_offset
    } else {
        0.0_f64
    };
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    upload.as_mut().unwrap().version = geometry.version;
    return ((upload).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-gl/src/glWireframeUpload.ts:86 (sha256:209cd37ce18bdfc0cebbd58b3ad7a663bc748f852db73608c929d9bd32a4db14)
fn build_line_indices(geometry: &MeshGeometry) -> Vec<u32> {
    let triangle_indices = (geometry.indices).clone();
    let triangle_count = if (triangle_indices).is_some() {
        ((triangle_indices.as_ref().unwrap().len() as f64) / 3.0_f64).floor()
    } else {
        ((((geometry.vertices.len() as f64) * 4.0_f64) / geometry.layout.stride) / 3.0_f64).floor()
    };
    let line_count = (triangle_count * 6.0_f64);
    let use_uint32 = ((triangle_indices).is_some()) || (line_count > 65535.0_f64);
    let mut lines = if use_uint32 {
        vec![0_u32; (line_count) as usize]
    } else {
        vec![0_u32; (line_count) as usize]
    };
    {
        let mut t = 0.0_f64;
        while (t < triangle_count) {
            let base = (t * 3.0_f64);
            let i0 = if (triangle_indices).is_some() {
                (triangle_indices.as_ref().unwrap()[base as usize] as f64) as u32
            } else {
                (base) as u32
            };
            let i1 = if (triangle_indices).is_some() {
                (triangle_indices.as_ref().unwrap()[(base + 1.0_f64) as usize] as f64) as u32
            } else {
                (base + 1.0_f64) as u32
            };
            let i2 = if (triangle_indices).is_some() {
                (triangle_indices.as_ref().unwrap()[(base + 2.0_f64) as usize] as f64) as u32
            } else {
                (base + 2.0_f64) as u32
            };
            let out = (t * 6.0_f64);
            lines[out as usize] = ((i0).clone()) as u32;
            lines[(out + 1.0_f64) as usize] = ((i1).clone()) as u32;
            lines[(out + 2.0_f64) as usize] = ((i1).clone()) as u32;
            lines[(out + 3.0_f64) as usize] = ((i2).clone()) as u32;
            lines[(out + 4.0_f64) as usize] = ((i2).clone()) as u32;
            lines[(out + 5.0_f64) as usize] = ((i0).clone()) as u32;
            {
                t += 1.0;
                t
            };
        }
    }
    return lines;
}

// Source: upstream/packages/scene-gl/src/glWireframeUpload.ts:115 (sha256:d0fd91cc11cfecef0f3cb55b7ea2c9f98c08df683a5fefc5f58dbbe889c19f4d)
static WIREFRAME_UPLOADS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, Vec<(MeshGeometry, GlWireframeUpload)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
