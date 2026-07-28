// @generated from upstream/packages/skeleton3d/src/skinMeshGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::skin_vertices;
use flighthq_types::{MeshGeometry, MeshSkinBindPose, Skeleton3D, VertexAttributeLayout};

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

// Source: upstream/packages/skeleton3d/src/skinMeshGeometry.ts:12 (sha256:fa6035626407accf0c54efd205b5b169e4a0e8eb88879c4370d247e02db1dc8f)
pub fn capture_mesh_skin_bind_pose(geometry: &MeshGeometry) -> MeshSkinBindPose {
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let vertex_count = if (floats_per_vertex > 0.0_f64) {
        (__flight_js_to_i32(((geometry.vertices.len() as f64) / floats_per_vertex))
            | __flight_js_to_i32(0.0_f64)) as f64
    } else {
        0.0_f64
    };
    let position_offset = float_offset_for_semantic(&geometry.layout, "position".to_owned());
    let normal_offset = float_offset_for_semantic(&geometry.layout, "normal".to_owned());
    let joints_offset = float_offset_for_semantic(&geometry.layout, "joints0".to_owned());
    let weights_offset = float_offset_for_semantic(&geometry.layout, "weights0".to_owned());
    let mut positions = vec![0.0_f32; (vertex_count * 3.0_f64) as usize];
    let mut normals = vec![0.0_f32; (vertex_count * 3.0_f64) as usize];
    let mut joints = vec![0.0_f32; (vertex_count * 4.0_f64) as usize];
    let mut weights = vec![0.0_f32; (vertex_count * 4.0_f64) as usize];
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let base = (v * floats_per_vertex);
            let p = (v * 3.0_f64);
            let w = (v * 4.0_f64);
            if (position_offset >= 0.0_f64) {
                positions[p as usize] =
                    (geometry.vertices[(base + position_offset) as usize] as f64) as f32;
                positions[(p + 1.0_f64) as usize] = (geometry.vertices
                    [((base + position_offset) + 1.0_f64) as usize]
                    as f64) as f32;
                positions[(p + 2.0_f64) as usize] = (geometry.vertices
                    [((base + position_offset) + 2.0_f64) as usize]
                    as f64) as f32;
            }
            if (normal_offset >= 0.0_f64) {
                normals[p as usize] =
                    (geometry.vertices[(base + normal_offset) as usize] as f64) as f32;
                normals[(p + 1.0_f64) as usize] =
                    (geometry.vertices[((base + normal_offset) + 1.0_f64) as usize] as f64) as f32;
                normals[(p + 2.0_f64) as usize] =
                    (geometry.vertices[((base + normal_offset) + 2.0_f64) as usize] as f64) as f32;
            }
            if (joints_offset >= 0.0_f64) {
                joints[w as usize] =
                    (geometry.vertices[(base + joints_offset) as usize] as f64) as f32;
                joints[(w + 1.0_f64) as usize] =
                    (geometry.vertices[((base + joints_offset) + 1.0_f64) as usize] as f64) as f32;
                joints[(w + 2.0_f64) as usize] =
                    (geometry.vertices[((base + joints_offset) + 2.0_f64) as usize] as f64) as f32;
                joints[(w + 3.0_f64) as usize] =
                    (geometry.vertices[((base + joints_offset) + 3.0_f64) as usize] as f64) as f32;
            }
            if (weights_offset >= 0.0_f64) {
                weights[w as usize] =
                    (geometry.vertices[(base + weights_offset) as usize] as f64) as f32;
                weights[(w + 1.0_f64) as usize] =
                    (geometry.vertices[((base + weights_offset) + 1.0_f64) as usize] as f64) as f32;
                weights[(w + 2.0_f64) as usize] =
                    (geometry.vertices[((base + weights_offset) + 2.0_f64) as usize] as f64) as f32;
                weights[(w + 3.0_f64) as usize] =
                    (geometry.vertices[((base + weights_offset) + 3.0_f64) as usize] as f64) as f32;
            }
            {
                v += 1.0;
                v
            };
        }
    }
    return MeshSkinBindPose {
        __flight_identity: std::sync::Arc::new(()),
        joints: (joints).clone(),
        normals: (normals).clone(),
        positions: (positions).clone(),
        skinned_normals: vec![0.0_f32; (vertex_count * 3.0_f64) as usize],
        skinned_positions: vec![0.0_f32; (vertex_count * 3.0_f64) as usize],
        weights: (weights).clone(),
    };
}

// Source: upstream/packages/skeleton3d/src/skinMeshGeometry.ts:72 (sha256:f96e8959c09cb592cad2ecd54862aa8a5e5d1c8a929499f7a1d2893715732ef7)
pub fn skin_mesh_geometry(
    geometry: &mut MeshGeometry,
    skeleton: &Skeleton3D,
    bind_pose: &mut MeshSkinBindPose,
) -> () {
    {
        let __flight_argument_2 = (bind_pose.positions).clone();
        let __flight_argument_3 = (bind_pose.normals).clone();
        let __flight_argument_4 = (bind_pose.joints).clone();
        let __flight_argument_5 = (bind_pose.weights).clone();
        skin_vertices(
            &mut bind_pose.skinned_positions,
            &mut bind_pose.skinned_normals,
            &__flight_argument_2,
            &__flight_argument_3,
            &__flight_argument_4,
            &__flight_argument_5,
            &skeleton.joint_matrices,
        )
    };
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let position_offset = float_offset_for_semantic(&geometry.layout, "position".to_owned());
    let normal_offset = float_offset_for_semantic(&geometry.layout, "normal".to_owned());
    let vertex_count = (__flight_js_to_i32(((bind_pose.skinned_positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let base = (v * floats_per_vertex);
            let s = (v * 3.0_f64);
            if (position_offset >= 0.0_f64) {
                geometry.vertices[(base + position_offset) as usize] =
                    (bind_pose.skinned_positions[s as usize] as f64) as f32;
                geometry.vertices[((base + position_offset) + 1.0_f64) as usize] =
                    (bind_pose.skinned_positions[(s + 1.0_f64) as usize] as f64) as f32;
                geometry.vertices[((base + position_offset) + 2.0_f64) as usize] =
                    (bind_pose.skinned_positions[(s + 2.0_f64) as usize] as f64) as f32;
            }
            if (normal_offset >= 0.0_f64) {
                geometry.vertices[(base + normal_offset) as usize] =
                    (bind_pose.skinned_normals[s as usize] as f64) as f32;
                geometry.vertices[((base + normal_offset) + 1.0_f64) as usize] =
                    (bind_pose.skinned_normals[(s + 1.0_f64) as usize] as f64) as f32;
                geometry.vertices[((base + normal_offset) + 2.0_f64) as usize] =
                    (bind_pose.skinned_normals[(s + 2.0_f64) as usize] as f64) as f32;
            }
            {
                v += 1.0;
                v
            };
        }
    }
    {
        geometry.version += 1.0;
        geometry.version
    };
}

// Source: upstream/packages/skeleton3d/src/skinMeshGeometry.ts:114 (sha256:4784a400303ed042d4f353458dcc4cb54909ac422e5545c1b3490f908aff4fa4)
fn float_offset_for_semantic(layout: &VertexAttributeLayout, semantic: String) -> f64 {
    {
        let mut i = 0.0_f64;
        while (i < (layout.attributes.len() as f64)) {
            if ((layout.attributes[i as usize].semantic).clone() == semantic) {
                return (layout.attributes[i as usize].byte_offset / 4.0_f64);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return (-1.0_f64);
}
