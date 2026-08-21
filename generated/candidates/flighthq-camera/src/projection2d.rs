// @generated from upstream/packages/camera/src/projection2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_camera2_d_view_matrix;
use flighthq_geometry::{
    create_matrix, inverse_matrix_transform_point_xy, matrix_transform_point_xy,
};
use flighthq_types::{Camera2D, Matrix, MatrixLike, Vector2Like};

// Source: upstream/packages/camera/src/projection2d.ts:8 (sha256:b7f88150b143317d505ee23dce214afe007ab8d18a57aafdc08cea395ee28b45)
pub fn project_camera2_d_point(
    camera: &Camera2D,
    world_x: f64,
    world_y: f64,
    out: &mut Vector2Like,
) -> () {
    get_camera2_d_view_matrix(camera, &mut (*SCRATCH_MATRIX.lock().unwrap()));
    matrix_transform_point_xy(
        out,
        &{
            let __flight_source = &(*SCRATCH_MATRIX.lock().unwrap());
            MatrixLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                a: __flight_source.a,
                b: __flight_source.b,
                c: __flight_source.c,
                d: __flight_source.d,
                tx: __flight_source.tx,
                ty: __flight_source.ty,
            }
        },
        world_x,
        world_y,
    );
}

// Source: upstream/packages/camera/src/projection2d.ts:21 (sha256:252f86f003289ea71e5402bef93598fbc4ba8229df84891524b081707e2008e8)
pub fn unproject_camera2_d_point(
    camera: &Camera2D,
    screen_x: f64,
    screen_y: f64,
    out: &mut Vector2Like,
) -> () {
    get_camera2_d_view_matrix(camera, &mut (*SCRATCH_MATRIX.lock().unwrap()));
    inverse_matrix_transform_point_xy(
        out,
        &{
            let __flight_source = &(*SCRATCH_MATRIX.lock().unwrap());
            MatrixLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                a: __flight_source.a,
                b: __flight_source.b,
                c: __flight_source.c,
                d: __flight_source.d,
                tx: __flight_source.tx,
                ty: __flight_source.ty,
            }
        },
        screen_x,
        screen_y,
    );
}

// Source: upstream/packages/camera/src/projection2d.ts:31 (sha256:bd1c7961ccdf3d194ced82b6528787dba39e75a0c7340b479ed42f02a0bfe67b)
static SCRATCH_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });
