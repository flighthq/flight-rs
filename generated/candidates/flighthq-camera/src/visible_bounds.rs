// @generated from upstream/packages/camera/src/visibleBounds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_camera2_d_view_matrix;
use flighthq_geometry::{create_matrix, inverse_matrix, matrix_transform_bounds};
use flighthq_types::{Camera2D, Matrix, MatrixLike, RectangleLike};

// Source: upstream/packages/camera/src/visibleBounds.ts:12 (sha256:461d01346f3440b6ceb039d7aa60872eb9d0c58d9732f46a86c049c57131059e)
pub fn get_camera2_d_visible_bounds(camera: &Camera2D, out: &mut RectangleLike) -> () {
    get_camera2_d_view_matrix(camera, &mut (*SCRATCH_MATRIX.lock().unwrap()));
    inverse_matrix(&mut (*SCRATCH_INVERSE.lock().unwrap()), &{
        let __flight_source = &(*SCRATCH_MATRIX.lock().unwrap());
        MatrixLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            a: __flight_source.a,
            b: __flight_source.b,
            c: __flight_source.c,
            d: __flight_source.d,
            tx: __flight_source.tx,
            ty: __flight_source.ty,
        }
    });
    matrix_transform_bounds(
        out,
        &{
            let __flight_source = &(*SCRATCH_INVERSE.lock().unwrap());
            MatrixLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                a: __flight_source.a,
                b: __flight_source.b,
                c: __flight_source.c,
                d: __flight_source.d,
                tx: __flight_source.tx,
                ty: __flight_source.ty,
            }
        },
        0.0_f64,
        0.0_f64,
        camera.viewport_width,
        camera.viewport_height,
    );
}

// Source: upstream/packages/camera/src/visibleBounds.ts:18 (sha256:5a5912d6fd6c54ce85b8340e33c7399d57ff876b273e4908ca3da2e3c0e4d054)
static SCRATCH_INVERSE: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });

// Source: upstream/packages/camera/src/visibleBounds.ts:19 (sha256:bd1c7961ccdf3d194ced82b6528787dba39e75a0c7340b479ed42f02a0bfe67b)
static SCRATCH_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });
