// @generated from upstream/packages/scene-wgpu/src/wgpuMeshUpload.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{WgpuMeshUpload, get_wgpu_scene_runtime};
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

// Source: upstream/packages/scene-wgpu/src/wgpuMeshUpload.ts:14 (sha256:11b376f68db5b7b1b573bf2ec72d92f5d1430253ca58d205ac2993440c6c27dd)
pub fn ensure_wgpu_mesh_upload(
    state: &mut WgpuRenderState,
    geometry: &mut MeshGeometry,
) -> Option<WgpuMeshUpload> {
    let indices = (geometry.indices).clone();
    if (indices).is_none() {
        return None;
    }
    let mut upload = get_wgpu_scene_runtime(state)
        .upload_cache
        .iter()
        .find(|(key, _)| key == &geometry)
        .map(|(_, value)| value.clone());
    if ((upload).is_some()) && (upload.as_mut().unwrap().version == geometry.version) {
        return (upload).clone();
    }
    let device = (state.device).clone();
    if (upload).is_some() {
        crate::host_value::<()>("host.destroy");
        crate::host_value::<()>("host.destroy");
    }
    let vertex_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.writeBuffer");
    let index_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.writeBuffer");
    upload = Some(WgpuMeshUpload {
        __flight_identity: std::sync::Arc::new(()),
        index_buffer: index_buffer,
        index_count: (indices.as_ref().unwrap().len() as f64),
        index_format: if (indices.as_ref().unwrap().bytes_per_element == 4.0_f64) {
            crate::OpaqueHostValue::String("uint32".to_owned())
        } else {
            crate::OpaqueHostValue::String("uint16".to_owned())
        },
        version: geometry.version,
        vertex_buffer: (vertex_buffer).clone(),
    });
    {
        let __flight_key = geometry;
        let __flight_value = (upload).clone().unwrap();
        if let Some((_, value)) = get_wgpu_scene_runtime(state)
            .upload_cache
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            get_wgpu_scene_runtime(state)
                .upload_cache
                .push((__flight_key, __flight_value));
        }
    };
    let mut mesh_runtime = Some(
        ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(geometry)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        })
        .clone(),
    );
    if (mesh_runtime).is_some() {
        {
            let __flight_runtime = mesh_runtime.as_mut().unwrap();
            let __flight_value = Some((upload).clone().unwrap());
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.webgpu_data = __flight_value;
        };
    }
    return (upload).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshUpload.ts:62 (sha256:7373b4bd7ad102ceeb0775c768ea5777c9d0508717084ac04d5b05f8f58d58c3)
fn align_to4(byte_length: f64) -> f64 {
    return (__flight_js_to_i32((byte_length + 3.0_f64)) & __flight_js_to_i32((!3.0_f64))) as f64;
}
