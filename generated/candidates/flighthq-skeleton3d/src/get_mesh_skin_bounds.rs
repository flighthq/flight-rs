// @generated from upstream/packages/skeleton3d/src/getMeshSkinBounds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

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
    let positions = &bind_pose.positions;
    let rest_vertex_count = (__flight_js_to_i32(((positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    let mut rest_min_x = number.positive_infinity;
    let mut rest_min_y = number.positive_infinity;
    let mut rest_min_z = number.positive_infinity;
    let mut rest_max_x = number.negative_infinity;
    let mut rest_max_y = number.negative_infinity;
    let mut rest_max_z = number.negative_infinity;
    {
        let mut v = 0.0_f64;
        while (v < rest_vertex_count) {
            let p = (v * 3.0_f64);
            let px = (positions[p as usize] as f64);
            let py = (positions[(p + 1.0_f64) as usize] as f64);
            let pz = (positions[(p + 2.0_f64) as usize] as f64);
            if (px < rest_min_x) {
                rest_min_x = px;
            }
            if (py < rest_min_y) {
                rest_min_y = py;
            }
            if (pz < rest_min_z) {
                rest_min_z = pz;
            }
            if (px > rest_max_x) {
                rest_max_x = px;
            }
            if (py > rest_max_y) {
                rest_max_y = py;
            }
            if (pz > rest_max_z) {
                rest_max_z = pz;
            }
            {
                v += 1.0;
                v
            };
        }
    }
    if (rest_vertex_count == 0.0_f64) {
        out.min.x = number.positive_infinity;
        out.min.y = number.positive_infinity;
        out.min.z = number.positive_infinity;
        out.max.x = number.negative_infinity;
        out.max.y = number.negative_infinity;
        out.max.z = number.negative_infinity;
        return;
    }
    let cx = ((rest_min_x + rest_max_x) * 0.5_f64);
    let cy = ((rest_min_y + rest_max_y) * 0.5_f64);
    let cz = ((rest_min_z + rest_max_z) * 0.5_f64);
    let ex = ((rest_max_x - rest_min_x) * 0.5_f64);
    let ey = ((rest_max_y - rest_min_y) * 0.5_f64);
    let ez = ((rest_max_z - rest_min_z) * 0.5_f64);
    let palette = &skeleton.joint_matrices;
    let joint_count = (__flight_js_to_i32(((palette.len() as f64) / 16.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    let referenced = get_referenced_joints(
        (bind_pose.joints).clone(),
        (bind_pose.weights).clone(),
        joint_count,
    );
    let mut min_x = number.positive_infinity;
    let mut min_y = number.positive_infinity;
    let mut min_z = number.positive_infinity;
    let mut max_x = number.negative_infinity;
    let mut max_y = number.negative_infinity;
    let mut max_z = number.negative_infinity;
    {
        let mut j = 0.0_f64;
        while (j < joint_count) {
            if (!referenced[j as usize].clone()) {
                {
                    j += 1.0;
                    j
                };
                continue;
            }
            let m = (j * 16.0_f64);
            let tcx = (((((palette[m as usize] as f64) * cx)
                + ((palette[(m + 4.0_f64) as usize] as f64) * cy))
                + ((palette[(m + 8.0_f64) as usize] as f64) * cz))
                + (palette[(m + 12.0_f64) as usize] as f64));
            let tcy = (((((palette[(m + 1.0_f64) as usize] as f64) * cx)
                + ((palette[(m + 5.0_f64) as usize] as f64) * cy))
                + ((palette[(m + 9.0_f64) as usize] as f64) * cz))
                + (palette[(m + 13.0_f64) as usize] as f64));
            let tcz = (((((palette[(m + 2.0_f64) as usize] as f64) * cx)
                + ((palette[(m + 6.0_f64) as usize] as f64) * cy))
                + ((palette[(m + 10.0_f64) as usize] as f64) * cz))
                + (palette[(m + 14.0_f64) as usize] as f64));
            let tex = ((((palette[m as usize] as f64).abs() * ex)
                + ((palette[(m + 4.0_f64) as usize] as f64).abs() * ey))
                + ((palette[(m + 8.0_f64) as usize] as f64).abs() * ez));
            let tey = ((((palette[(m + 1.0_f64) as usize] as f64).abs() * ex)
                + ((palette[(m + 5.0_f64) as usize] as f64).abs() * ey))
                + ((palette[(m + 9.0_f64) as usize] as f64).abs() * ez));
            let tez = ((((palette[(m + 2.0_f64) as usize] as f64).abs() * ex)
                + ((palette[(m + 6.0_f64) as usize] as f64).abs() * ey))
                + ((palette[(m + 10.0_f64) as usize] as f64).abs() * ez));
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
    bind_pose: &MeshSkinBindPose,
    skeleton: &Skeleton3D,
) -> () {
    skin_vertices(
        (bind_pose.skinned_positions).clone(),
        (bind_pose.skinned_normals).clone(),
        (bind_pose.positions).clone(),
        (bind_pose.normals).clone(),
        (bind_pose.joints).clone(),
        (bind_pose.weights).clone(),
        (skeleton.joint_matrices).clone(),
    );
    let skinned = &bind_pose.skinned_positions;
    let vertex_count = (__flight_js_to_i32(((skinned.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    let mut min_x = number.positive_infinity;
    let mut min_y = number.positive_infinity;
    let mut min_z = number.positive_infinity;
    let mut max_x = number.negative_infinity;
    let mut max_y = number.negative_infinity;
    let mut max_z = number.negative_infinity;
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let p = (v * 3.0_f64);
            let px = (skinned[p as usize] as f64);
            let py = (skinned[(p + 1.0_f64) as usize] as f64);
            let pz = (skinned[(p + 2.0_f64) as usize] as f64);
            if (px < min_x) {
                min_x = px;
            }
            if (py < min_y) {
                min_y = py;
            }
            if (pz < min_z) {
                min_z = pz;
            }
            if (px > max_x) {
                max_x = px;
            }
            if (py > max_y) {
                max_y = py;
            }
            if (pz > max_z) {
                max_z = pz;
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
    let mut referenced = vec![0_u8; (joint_count) as usize];
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
            if ((j >= 0.0_f64) && (j < joint_count)) {
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
