// @generated from upstream/packages/scene-wgpu/src/wgpuWireframeUpload.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ensure_wgpu_mesh_upload;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, ImageResource, Matrix, MeshGeometry,
    Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap,
    Vector2, WgpuRenderState,
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

// Source: upstream/packages/scene-wgpu/src/wgpuWireframeUpload.ts:10 (sha256:f47b684c999ca62a6410324b6f9fd4c19ecdfd4defb5ab69843eefc33182b1c0)
#[derive(Clone, Default)]
pub struct WgpuWireframeUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_format: crate::OpaqueHostValue,
    pub line_index_buffer: crate::OpaqueHostValue,
    pub version: f64,
    pub vertex_buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuWireframeUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframeUpload.ts:22 (sha256:dd9a9dcf5b2c64a5ba33dd3736b3ce60315492c919289e058db61fa505eefe59)
pub fn ensure_wgpu_wireframe_upload(
    state: &mut WgpuRenderState,
    geometry: &mut MeshGeometry,
) -> Option<WgpuWireframeUpload> {
    let mesh_upload = ensure_wgpu_mesh_upload(state, geometry);
    if (mesh_upload).is_none() {
        return None;
    }
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
        return (upload).clone();
    }
    let device = (state.device).clone();
    if (upload).is_some() {
        crate::host_value::<()>("host.destroy");
    }
    let lines = build_line_indices(geometry);
    let line_index_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.writeBuffer");
    upload = Some(WgpuWireframeUpload {
        __flight_identity: std::sync::Arc::new(()),
        index_format: if true {
            crate::OpaqueHostValue::String("uint32".to_owned())
        } else {
            crate::OpaqueHostValue::String("uint16".to_owned())
        },
        line_index_buffer: (line_index_buffer).clone(),
        version: geometry.version,
        vertex_buffer: (mesh_upload.as_ref().unwrap().vertex_buffer).clone(),
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
    return (upload).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframeUpload.ts:62 (sha256:802b788d5813728787526b6d53ef84e03db07d6dd280d45c3d55aed4c461b04f)
fn build_line_indices(geometry: &MeshGeometry) -> Vec<u32> {
    let triangle_indices = (geometry.indices).clone();
    let triangle_count = ((triangle_indices.as_ref().unwrap().len() as f64) / 3.0_f64).floor();
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
            let i0 = triangle_indices.as_ref().unwrap()[base as usize].clone();
            let i1 = triangle_indices.as_ref().unwrap()[(base + 1.0_f64) as usize].clone();
            let i2 = triangle_indices.as_ref().unwrap()[(base + 2.0_f64) as usize].clone();
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

// Source: upstream/packages/scene-wgpu/src/wgpuWireframeUpload.ts:86 (sha256:7373b4bd7ad102ceeb0775c768ea5777c9d0508717084ac04d5b05f8f58d58c3)
fn align_to4(byte_length: f64) -> f64 {
    return (__flight_js_to_i32((byte_length + 3.0_f64)) & __flight_js_to_i32((!3.0_f64))) as f64;
}

// Source: upstream/packages/scene-wgpu/src/wgpuWireframeUpload.ts:92 (sha256:8b026a2e04a0b742a3af9e4a32da5cefda5118af6715ac8a82ca63e182577346)
static WIREFRAME_UPLOADS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(WgpuRenderState, Vec<(MeshGeometry, WgpuWireframeUpload)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
