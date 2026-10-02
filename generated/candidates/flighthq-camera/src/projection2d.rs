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

// Source: upstream/packages/camera/src/projection2d.ts:8 (sha256:2522ba02897cc3a7614e57bb49d6e8a8a4b1b64b454cd2ae863d74c19771b654)
pub fn project_camera2_d_point(
    camera: &Camera2D,
    viewport_width: f64,
    viewport_height: f64,
    world_x: f64,
    world_y: f64,
    out: &mut Vector2Like,
) -> () {
    get_camera2_d_view_matrix(
        camera,
        viewport_width,
        viewport_height,
        &mut (*SCRATCH_MATRIX.lock().unwrap()),
    );
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

// Source: upstream/packages/camera/src/projection2d.ts:23 (sha256:4fc813a1883f8efb7049dbb0e87dedf527481e862bdc175ad52538790afbf45a)
pub fn unproject_camera2_d_point(
    camera: &Camera2D,
    viewport_width: f64,
    viewport_height: f64,
    screen_x: f64,
    screen_y: f64,
    out: &mut Vector2Like,
) -> () {
    get_camera2_d_view_matrix(
        camera,
        viewport_width,
        viewport_height,
        &mut (*SCRATCH_MATRIX.lock().unwrap()),
    );
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

// Source: upstream/packages/camera/src/projection2d.ts:35 (sha256:bd1c7961ccdf3d194ced82b6528787dba39e75a0c7340b479ed42f02a0bfe67b)
static SCRATCH_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });
