// @generated from upstream/packages/skeleton3d/src/getMeshSkinBounds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::skin_vertices;
use flighthq_types::{AabbLike, MeshSkinBindPose, Skeleton3D};

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

// Source: upstream/packages/skeleton3d/src/getMeshSkinBounds.ts:21 (sha256:ad55bd33cfa387b7bceeccf09da04fe09d0b75b3bf9016be8b7b338409dba2a1)
pub fn get_mesh_skin_conservative_bounds(
    out: &mut AabbLike,
    bind_pose: &MeshSkinBindPose,
    skeleton: &Skeleton3D,
) -> () {
    let rest_vertex_count = (__flight_js_to_i32(((bind_pose.positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    let mut rest_min_x = f64::INFINITY;
    let mut rest_min_y = f64::INFINITY;
    let mut rest_min_z = f64::INFINITY;
    let mut rest_max_x = f64::NEG_INFINITY;
    let mut rest_max_y = f64::NEG_INFINITY;
    let mut rest_max_z = f64::NEG_INFINITY;
    {
        let mut v = 0.0_f64;
        while (v < rest_vertex_count) {
            let p = (v * 3.0_f64);
            let px = (bind_pose.positions[p as usize] as f64);
            let py = (bind_pose.positions[(p + 1.0_f64) as usize] as f64);
            let pz = (bind_pose.positions[(p + 2.0_f64) as usize] as f64);
            if (px < rest_min_x) {
                rest_min_x = ((px).clone()) as f32;
            }
            if (py < rest_min_y) {
                rest_min_y = ((py).clone()) as f32;
            }
            if (pz < rest_min_z) {
                rest_min_z = ((pz).clone()) as f32;
            }
            if (px > rest_max_x) {
                rest_max_x = ((px).clone()) as f32;
            }
            if (py > rest_max_y) {
                rest_max_y = ((py).clone()) as f32;
            }
            if (pz > rest_max_z) {
                rest_max_z = ((pz).clone()) as f32;
            }
            {
                v += 1.0;
                v
            };
        }
    }
    if (rest_vertex_count == 0.0_f64) {
        out.min.x = f64::INFINITY;
        out.min.y = f64::INFINITY;
        out.min.z = f64::INFINITY;
        out.max.x = f64::NEG_INFINITY;
        out.max.y = f64::NEG_INFINITY;
        out.max.z = f64::NEG_INFINITY;
        return;
    }
    let cx = ((rest_min_x + rest_max_x) * 0.5_f64);
    let cy = ((rest_min_y + rest_max_y) * 0.5_f64);
    let cz = ((rest_min_z + rest_max_z) * 0.5_f64);
    let ex = ((rest_max_x - rest_min_x) * 0.5_f64);
    let ey = ((rest_max_y - rest_min_y) * 0.5_f64);
    let ez = ((rest_max_z - rest_min_z) * 0.5_f64);
    let joint_count = (__flight_js_to_i32(((skeleton.joint_matrices.len() as f64) / 16.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    let referenced = get_referenced_joints(&bind_pose.joints, &bind_pose.weights, joint_count);
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    {
        let mut j = 0.0_f64;
        while (j < joint_count) {
            if (!(referenced[j as usize] as f64)) {
                {
                    j += 1.0;
                    j
                };
                continue;
            }
            let m = (j * 16.0_f64);
            let tcx = (((((skeleton.joint_matrices[m as usize] as f64) * cx)
                + ((skeleton.joint_matrices[(m + 4.0_f64) as usize] as f64) * cy))
                + ((skeleton.joint_matrices[(m + 8.0_f64) as usize] as f64) * cz))
                + (skeleton.joint_matrices[(m + 12.0_f64) as usize] as f64));
            let tcy = (((((skeleton.joint_matrices[(m + 1.0_f64) as usize] as f64) * cx)
                + ((skeleton.joint_matrices[(m + 5.0_f64) as usize] as f64) * cy))
                + ((skeleton.joint_matrices[(m + 9.0_f64) as usize] as f64) * cz))
                + (skeleton.joint_matrices[(m + 13.0_f64) as usize] as f64));
            let tcz = (((((skeleton.joint_matrices[(m + 2.0_f64) as usize] as f64) * cx)
                + ((skeleton.joint_matrices[(m + 6.0_f64) as usize] as f64) * cy))
                + ((skeleton.joint_matrices[(m + 10.0_f64) as usize] as f64) * cz))
                + (skeleton.joint_matrices[(m + 14.0_f64) as usize] as f64));
            let tex = ((((skeleton.joint_matrices[m as usize] as f64).abs() * ex)
                + ((skeleton.joint_matrices[(m + 4.0_f64) as usize] as f64).abs() * ey))
                + ((skeleton.joint_matrices[(m + 8.0_f64) as usize] as f64).abs() * ez));
            let tey = ((((skeleton.joint_matrices[(m + 1.0_f64) as usize] as f64).abs() * ex)
                + ((skeleton.joint_matrices[(m + 5.0_f64) as usize] as f64).abs() * ey))
                + ((skeleton.joint_matrices[(m + 9.0_f64) as usize] as f64).abs() * ez));
            let tez = ((((skeleton.joint_matrices[(m + 2.0_f64) as usize] as f64).abs() * ex)
                + ((skeleton.joint_matrices[(m + 6.0_f64) as usize] as f64).abs() * ey))
                + ((skeleton.joint_matrices[(m + 10.0_f64) as usize] as f64).abs() * ez));
            if ((tcx - tex) < min_x) {
                min_x = (tcx - tex);
            }
            if ((tcy - tey) < min_y) {
                min_y = (tcy - tey);
            }
            if ((tcz - tez) < min_z) {
                min_z = (tcz - tez);
            }
            if ((tcx + tex) > max_x) {
                max_x = (tcx + tex);
            }
            if ((tcy + tey) > max_y) {
                max_y = (tcy + tey);
            }
            if ((tcz + tez) > max_z) {
                max_z = (tcz + tez);
            }
            {
                j += 1.0;
                j
            };
        }
    }
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/skeleton3d/src/getMeshSkinBounds.ts:114 (sha256:d5f29d5d3f14f718693419aff01ce6b00a6572a1b05f144dd5c8fe1f355cee4d)
pub fn get_mesh_skin_exact_bounds(
    out: &mut AabbLike,
    bind_pose: &mut MeshSkinBindPose,
    skeleton: &Skeleton3D,
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
    let vertex_count = (__flight_js_to_i32(((bind_pose.skinned_positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let p = (v * 3.0_f64);
            let px = (bind_pose.skinned_positions[p as usize] as f64);
            let py = (bind_pose.skinned_positions[(p + 1.0_f64) as usize] as f64);
            let pz = (bind_pose.skinned_positions[(p + 2.0_f64) as usize] as f64);
            if (px < min_x) {
                min_x = ((px).clone()) as f32;
            }
            if (py < min_y) {
                min_y = ((py).clone()) as f32;
            }
            if (pz < min_z) {
                min_z = ((pz).clone()) as f32;
            }
            if (px > max_x) {
                max_x = ((px).clone()) as f32;
            }
            if (py > max_y) {
                max_y = ((py).clone()) as f32;
            }
            if (pz > max_z) {
                max_z = ((pz).clone()) as f32;
            }
            {
                v += 1.0;
                v
            };
        }
    }
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/skeleton3d/src/getMeshSkinBounds.ts:163 (sha256:aebbf376ba970c5b80fea2f774013c890a9a3c5ff59c2490afbc3631f2c882ba)
fn get_referenced_joints(joints: &Vec<f32>, weights: &Vec<f32>, joint_count: f64) -> Vec<u8> {
    let mut referenced: Vec<u8> = vec![0_u8; (joint_count) as usize];
    let influence_count = (joints.len() as f64);
    {
        let mut k = 0.0_f64;
        while (k < influence_count) {
            if ((weights[k as usize] as f64) == 0.0_f64) {
                {
                    k += 1.0;
                    k
                };
                continue;
            }
            let j = (__flight_js_to_i32((joints[k as usize] as f64)) | __flight_js_to_i32(0.0_f64))
                as f64;
            if (j >= 0.0_f64) && (j < joint_count) {
                referenced[j as usize] = (1.0_f64) as u8;
            }
            {
                k += 1.0;
                k
            };
        }
    }
    return referenced;
}
