// @generated from upstream/packages/scene-gl/src/glMeshUpload.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlMeshUpload, get_gl_scene_runtime};
use flighthq_mesh::get_mesh_geometry_skin_bind_pose;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, MeshGeometry,
    MeshSkinBindPose, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace,
    TextureFilter, TextureWrap, Vector2, VertexAttribute,
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

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:11 (sha256:c9ecc1866cbfa94aea8b79f7100912c81b69948ae077d85b0d888d8bf821c8c3)
pub fn destroy_gl_mesh_upload(state: &GlRenderState, upload: &GlMeshUpload) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.deleteVertexArray");
    crate::host_value::<()>("host.deleteBuffer");
    if ((upload.index_buffer).clone()).is_some() {
        crate::host_value::<()>("host.deleteBuffer");
    }
}

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:33 (sha256:9fbe2cc5540a107ba9e5e5d1e52b66b0d96e8bb73945e4d7020dd63b9c76743c)
pub fn ensure_gl_mesh_upload(
    state: &mut GlRenderState,
    geometry: &mut MeshGeometry,
    gpu_skinned: Option<bool>,
) -> GlMeshUpload {
    let gpu_skinned = gpu_skinned.unwrap_or(false);
    let gl = (state.gl).clone();
    let mut upload = get_gl_scene_runtime(state)
        .upload_cache
        .iter()
        .find(|(key, _)| key == &(*geometry).clone())
        .map(|(_, value)| value.clone());
    let bind_pose = if gpu_skinned {
        get_mesh_geometry_skin_bind_pose(geometry)
    } else {
        None
    };
    if ((upload).is_some())
        && (if (bind_pose).is_some() {
            (upload.as_mut().unwrap().skin_bind_uploaded) == Some(true)
        } else {
            (upload.as_mut().unwrap().version == geometry.version)
        })
    {
        crate::host_value::<()>("host.bindVertexArray");
        return ((upload).clone().unwrap()).clone();
    }
    if (upload).is_none() {
        upload = Some(GlMeshUpload {
            __flight_identity: std::sync::Arc::new(()),
            index_buffer: None,
            index_count: 0.0_f64,
            index_type: crate::host_value::<f64>("host.UNSIGNED_SHORT"),
            vao: crate::host_value::<crate::OpaqueHostValue>("host.createVertexArray"),
            version: (-1.0_f64),
            vertex_buffer: crate::host_value::<crate::OpaqueHostValue>("host.createBuffer"),
            skin_bind_uploaded: None,
        });
        {
            let __flight_key = (*geometry).clone();
            let __flight_value = (upload).clone().unwrap();
            if let Some((_, value)) = get_gl_scene_runtime(state)
                .upload_cache
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                get_gl_scene_runtime(state)
                    .upload_cache
                    .push((__flight_key, __flight_value));
            }
        };
    }
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    upload.as_mut().unwrap().skin_bind_uploaded = Some((bind_pose).is_some());
    let stride = geometry.layout.stride;
    {
        let mut i = 0.0_f64;
        while (i < (geometry.layout.attributes.len() as f64)) {
            bind_gl_vertex_attribute(
                (gl).clone(),
                &geometry.layout.attributes[i as usize],
                stride,
            );
            {
                i += 1.0;
                i
            };
        }
    }
    if ((geometry.indices).clone()).is_some() {
        if ((upload.as_mut().unwrap().index_buffer).clone()).is_none() {
            upload.as_mut().unwrap().index_buffer = Some(
                crate::host_value::<crate::OpaqueHostValue>("host.createBuffer"),
            );
        }
        crate::host_value::<()>("host.bindBuffer");
        crate::host_value::<()>("host.bufferData");
        upload.as_mut().unwrap().index_type = if ((geometry.indices).clone()).is_some() {
            crate::host_value::<f64>("host.UNSIGNED_INT")
        } else {
            crate::host_value::<f64>("host.UNSIGNED_SHORT")
        };
        upload.as_mut().unwrap().index_count = (geometry.indices.as_ref().unwrap().len() as f64);
    } else {
        upload.as_mut().unwrap().index_buffer = None;
        upload.as_mut().unwrap().index_count = 0.0_f64;
    }
    upload.as_mut().unwrap().version = geometry.version;
    return ((upload).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:100 (sha256:20a75e43b9673bd0bd0c2d49f63c854e431e311e6ea056e8c02ac73bba69a6c8)
pub fn has_gl_mesh_geometry_uv1(geometry: &MeshGeometry) -> bool {
    {
        let mut i = 0.0_f64;
        while (i < (geometry.layout.attributes.len() as f64)) {
            if ((geometry.layout.attributes[i as usize].semantic).clone() == "uv1") {
                return true;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return false;
}

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:112 (sha256:104e51e50d249371ebecfff168d8d67ef11bb7720fd522d144e4742219343c8c)
fn build_skin_bind_vertices(geometry: &mut MeshGeometry, bind_pose: &MeshSkinBindPose) -> Vec<f32> {
    let mut out = ((geometry.vertices).clone()).clone();
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let position_offset = float_offset_for_semantic(geometry, "position".to_owned());
    let normal_offset = float_offset_for_semantic(geometry, "normal".to_owned());
    let vertex_count = (__flight_js_to_i32(((bind_pose.positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let base = (v * floats_per_vertex);
            let s = (v * 3.0_f64);
            if (position_offset >= 0.0_f64) {
                out[(base + position_offset) as usize] = (bind_pose.positions[s as usize] as f64);
                out[((base + position_offset) + 1.0_f64) as usize] =
                    (bind_pose.positions[(s + 1.0_f64) as usize] as f64);
                out[((base + position_offset) + 2.0_f64) as usize] =
                    (bind_pose.positions[(s + 2.0_f64) as usize] as f64);
            }
            if (normal_offset >= 0.0_f64) {
                out[(base + normal_offset) as usize] = (bind_pose.normals[s as usize] as f64);
                out[((base + normal_offset) + 1.0_f64) as usize] =
                    (bind_pose.normals[(s + 1.0_f64) as usize] as f64);
                out[((base + normal_offset) + 2.0_f64) as usize] =
                    (bind_pose.normals[(s + 2.0_f64) as usize] as f64);
            }
            {
                v += 1.0;
                v
            };
        }
    }
    return out;
}

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:138 (sha256:e9dcbf0a92065ea35cc1b468448302a94456c845014ed1921205c068c4246a47)
fn float_offset_for_semantic(geometry: &MeshGeometry, semantic: String) -> f64 {
    {
        let mut i = 0.0_f64;
        while (i < (geometry.layout.attributes.len() as f64)) {
            if ((geometry.layout.attributes[i as usize].semantic).clone() == semantic) {
                return (geometry.layout.attributes[i as usize].byte_offset / 4.0_f64);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:152 (sha256:1acb12b0121b279e2d86337798144be860ffbd7b42e1e61c2cfacb92a192519d)
static ATTRIBUTE_LOCATION: std::sync::LazyLock<Vec<(String, f64)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push(("color0".to_owned(), 4.0_f64));
        __flight_record.push(("joints0".to_owned(), 6.0_f64));
        __flight_record.push(("normal".to_owned(), 1.0_f64));
        __flight_record.push(("position".to_owned(), 0.0_f64));
        __flight_record.push(("tangent".to_owned(), 2.0_f64));
        __flight_record.push(("uv0".to_owned(), 3.0_f64));
        __flight_record.push(("uv1".to_owned(), 5.0_f64));
        __flight_record.push(("weights0".to_owned(), 7.0_f64));
        __flight_record
    });

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:163 (sha256:9c80df118abfee8d3e8db98b7fa023c228e5ab795ec86126545eb88f03a38fb4)
fn bind_gl_vertex_attribute(
    gl: crate::OpaqueHostValue,
    attribute: &VertexAttribute,
    stride: f64,
) -> () {
    let location = ATTRIBUTE_LOCATION
        .iter()
        .find(|(key, _)| key == &(attribute.semantic).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
    if (location).is_none() {
        return;
    }
    let __destructure1 = resolve_gl_vertex_format((gl).clone(), (attribute.format).clone());
    let size = __destructure1[0.0_f64 as usize].clone();
    let type_ = __destructure1[1.0_f64 as usize].clone();
    let normalized = __destructure1[2.0_f64 as usize].clone();
    crate::host_value::<()>("host.enableVertexAttribArray");
    if (type_ == crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT")) {
        crate::host_value::<()>("host.vertexAttribPointer");
    } else {
        crate::host_value::<()>("host.vertexAttribIPointer");
    }
}

// Source: upstream/packages/scene-gl/src/glMeshUpload.ts:177 (sha256:87b501c4dd33dd29f0c0e869e5bf4524e412e034ca9834f0b57897622902338b)
fn resolve_gl_vertex_format(
    gl: crate::OpaqueHostValue,
    format: String,
) -> Vec<crate::FlightUnion2<f64, bool>> {
    {
        let __switch_value = format;
        let __flight_case = if __switch_value == "float32x2" {
            0_usize
        } else if __switch_value == "float32x3" {
            1_usize
        } else if __switch_value == "float32x4" {
            2_usize
        } else if __switch_value == "uint8x4" {
            3_usize
        } else if __switch_value == "unorm8x4" {
            4_usize
        } else if __switch_value == "uint16x4" {
            5_usize
        } else {
            6_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(2.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 1_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(3.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 2_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 3_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.UNSIGNED_BYTE"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 4_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.UNSIGNED_BYTE"),
                    crate::FlightUnion2::<f64, bool>::B(true),
                ];
            }
            if __flight_case <= 5_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.UNSIGNED_SHORT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 6_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(3.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}
