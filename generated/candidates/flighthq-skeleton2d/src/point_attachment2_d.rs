// @generated from upstream/packages/skeleton2d/src/pointAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_math::{DEG_TO_RAD as deg_to_rad_constant, RAD_TO_DEG as rad_to_deg_constant};
use flighthq_types::{PointAttachment2D, Skeleton2D, Vector2Like};

// Source: upstream/packages/skeleton2d/src/pointAttachment2D.ts:5 (sha256:43430cb9fcc08b2602c28ac39599e5c4ef0b3c11cc5d023d40daacc2bdcf1160)
const MATRIX_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/skeleton2d/src/pointAttachment2D.ts:9 (sha256:7e74af1cc71d813a2770429b2fb773a892871ea436658ca830587b600bc84ac6)
pub fn compute_skeleton2_d_point_attachment_position(
    out: &mut Vector2Like,
    attachment: &PointAttachment2D,
    skeleton: &Skeleton2D,
    bone_index: f64,
) -> () {
    if (bone_index < 0.0_f64)
        || ((bone_index * MATRIX_STRIDE) >= (skeleton.world_matrices.len() as f64))
    {
        return;
    }
    let b = (bone_index * MATRIX_STRIDE);
    let x = attachment.x;
    let y = attachment.y;
    out.x = ((((skeleton.world_matrices[b as usize] as f64) * x)
        + ((skeleton.world_matrices[(b + 2.0_f64) as usize] as f64) * y))
        + (skeleton.world_matrices[(b + 4.0_f64) as usize] as f64));
    out.y = ((((skeleton.world_matrices[(b + 1.0_f64) as usize] as f64) * x)
        + ((skeleton.world_matrices[(b + 3.0_f64) as usize] as f64) * y))
        + (skeleton.world_matrices[(b + 5.0_f64) as usize] as f64));
}

// Source: upstream/packages/skeleton2d/src/pointAttachment2D.ts:29 (sha256:9ddcf079a2c38db27bcbdeea3d0afe7a40b6be7f2604d64744291ede6922f160)
pub fn compute_skeleton2_d_point_attachment_rotation(
    attachment: &PointAttachment2D,
    skeleton: &Skeleton2D,
    bone_index: f64,
) -> f64 {
    if (bone_index < 0.0_f64)
        || ((bone_index * MATRIX_STRIDE) >= (skeleton.world_matrices.len() as f64))
    {
        return attachment.rotation;
    }
    let b = (bone_index * MATRIX_STRIDE);
    let radians = (attachment.rotation * deg_to_rad_constant);
    let cos = (radians).cos();
    let sin = (radians).sin();
    let x = (((skeleton.world_matrices[b as usize] as f64) * cos)
        + ((skeleton.world_matrices[(b + 2.0_f64) as usize] as f64) * sin));
    let y = (((skeleton.world_matrices[(b + 1.0_f64) as usize] as f64) * cos)
        + ((skeleton.world_matrices[(b + 3.0_f64) as usize] as f64) * sin));
    return ((y).atan2(x) * rad_to_deg_constant);
}
