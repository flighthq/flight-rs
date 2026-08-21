// @generated from upstream/packages/skeleton2d/src/regionAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{matrix_transform_point_xy, multiply_matrix, set_transform_matrix};
use flighthq_math::DEG_TO_RAD as deg_to_rad_constant;
use flighthq_types::{MatrixLike, RegionAttachment2D, Skeleton2D, Vector2};

// Source: upstream/packages/skeleton2d/src/regionAttachment2D.ts:6 (sha256:43430cb9fcc08b2602c28ac39599e5c4ef0b3c11cc5d023d40daacc2bdcf1160)
const MATRIX_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/skeleton2d/src/regionAttachment2D.ts:13 (sha256:150a7d0db77b99567966f0541cbc67eb7eb54c2ad606e22295021fecc09e8c6b)
pub fn compute_skeleton2_d_region_attachment_vertices(
    out: &mut Vec<f32>,
    attachment: &RegionAttachment2D,
    skeleton: &Skeleton2D,
    bone_index: f64,
) -> () {
    if (bone_index < 0.0_f64)
        || ((bone_index * MATRIX_STRIDE) >= (skeleton.world_matrices.len() as f64))
    {
        return;
    }
    set_transform_matrix(
        &mut (*_LOCAL.lock().unwrap()),
        attachment.scale_x,
        attachment.scale_y,
        Some((attachment.rotation * deg_to_rad_constant)),
        Some(attachment.x),
        Some(attachment.y),
    );
    let b = (bone_index * MATRIX_STRIDE);
    (*_BONE.lock().unwrap()).a = (skeleton.world_matrices[b as usize] as f64);
    (*_BONE.lock().unwrap()).b = (skeleton.world_matrices[(b + 1.0_f64) as usize] as f64);
    (*_BONE.lock().unwrap()).c = (skeleton.world_matrices[(b + 2.0_f64) as usize] as f64);
    (*_BONE.lock().unwrap()).d = (skeleton.world_matrices[(b + 3.0_f64) as usize] as f64);
    (*_BONE.lock().unwrap()).tx = (skeleton.world_matrices[(b + 4.0_f64) as usize] as f64);
    (*_BONE.lock().unwrap()).ty = (skeleton.world_matrices[(b + 5.0_f64) as usize] as f64);
    multiply_matrix(
        &mut (*_COMBINED.lock().unwrap()),
        &(*_BONE.lock().unwrap()),
        &(*_LOCAL.lock().unwrap()),
    );
    let hw = (attachment.width / 2.0_f64);
    let hh = (attachment.height / 2.0_f64);
    matrix_transform_point_xy(
        &mut (*_CORNER.lock().unwrap()),
        &(*_COMBINED.lock().unwrap()),
        (-hw),
        (-hh),
    );
    out[0.0_f64 as usize] = ((*_CORNER.lock().unwrap()).x) as f32;
    out[1.0_f64 as usize] = ((*_CORNER.lock().unwrap()).y) as f32;
    matrix_transform_point_xy(
        &mut (*_CORNER.lock().unwrap()),
        &(*_COMBINED.lock().unwrap()),
        (-hw),
        hh,
    );
    out[2.0_f64 as usize] = ((*_CORNER.lock().unwrap()).x) as f32;
    out[3.0_f64 as usize] = ((*_CORNER.lock().unwrap()).y) as f32;
    matrix_transform_point_xy(
        &mut (*_CORNER.lock().unwrap()),
        &(*_COMBINED.lock().unwrap()),
        hw,
        hh,
    );
    out[4.0_f64 as usize] = ((*_CORNER.lock().unwrap()).x) as f32;
    out[5.0_f64 as usize] = ((*_CORNER.lock().unwrap()).y) as f32;
    matrix_transform_point_xy(
        &mut (*_CORNER.lock().unwrap()),
        &(*_COMBINED.lock().unwrap()),
        hw,
        (-hh),
    );
    out[6.0_f64 as usize] = ((*_CORNER.lock().unwrap()).x) as f32;
    out[7.0_f64 as usize] = ((*_CORNER.lock().unwrap()).y) as f32;
}

// Source: upstream/packages/skeleton2d/src/regionAttachment2D.ts:57 (sha256:aedfd35bdbaa8a14fb3966a464fb67cc058aab2370f3db84e7123f98b6f09381)
static _LOCAL: std::sync::LazyLock<std::sync::Mutex<MatrixLike>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(MatrixLike {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        a: 1.0_f64,
        b: 0.0_f64,
        c: 0.0_f64,
        d: 1.0_f64,
        tx: 0.0_f64,
        ty: 0.0_f64,
    })
});

// Source: upstream/packages/skeleton2d/src/regionAttachment2D.ts:58 (sha256:cffa6df61687ed5b4a86dac562bea8d9d6b9e76608748884053bee6cf15b478c)
static _BONE: std::sync::LazyLock<std::sync::Mutex<MatrixLike>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(MatrixLike {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        a: 1.0_f64,
        b: 0.0_f64,
        c: 0.0_f64,
        d: 1.0_f64,
        tx: 0.0_f64,
        ty: 0.0_f64,
    })
});

// Source: upstream/packages/skeleton2d/src/regionAttachment2D.ts:59 (sha256:d88859a7328eb833af94aef41cc24f254791b58e31634e726a040edcd0ccff08)
static _COMBINED: std::sync::LazyLock<std::sync::Mutex<MatrixLike>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(MatrixLike {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_snapshot: Default::default(),
            __flight_entity_runtime: Default::default(),
            a: 1.0_f64,
            b: 0.0_f64,
            c: 0.0_f64,
            d: 1.0_f64,
            tx: 0.0_f64,
            ty: 0.0_f64,
        })
    });

// Source: upstream/packages/skeleton2d/src/regionAttachment2D.ts:60 (sha256:c56afd05aec52aba343da7b0e61ccf268e61bd59130ba8e7f34cb0b6fb43b745)
static _CORNER: std::sync::LazyLock<std::sync::Mutex<Vector2>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
    })
});
