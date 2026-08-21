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

// Source: upstream/packages/camera/src/visibleBounds.ts:20 (sha256:b28227654051241757ef81216b778cc4c5dab70af0287506f16a17db6bb1ed75)
pub fn get_camera2_d_visible_bounds(camera: &Camera2D, out: &mut RectangleLike) -> () {
    get_camera2_d_view_matrix(camera, &mut (*SCRATCH_MATRIX.lock().unwrap()));
    if (!inverse_matrix(&mut (*SCRATCH_INVERSE.lock().unwrap()), &{
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
    })) {
        out.x = *UNBOUNDED_ORIGIN;
        out.y = *UNBOUNDED_ORIGIN;
        out.width = *UNBOUNDED_EXTENT;
        out.height = *UNBOUNDED_EXTENT;
        {
            let __flight_callback = (*DEGENERATE_VISIBLE_BOUNDS_GUARD.lock().unwrap()).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()((*camera).clone()))
        };
        return;
    }
    matrix_transform_bounds(
        out,
        &{
            let __flight_source = &(*SCRATCH_INVERSE.lock().unwrap());
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
        0.0_f64,
        0.0_f64,
        camera.viewport_width,
        camera.viewport_height,
    );
}

// Source: upstream/packages/camera/src/visibleBounds.ts:35 (sha256:90e9a7d21befcceede4f67c1b76b278efadbfea3df268e4f74178e952e8e2615)
pub fn set_camera2_d_visible_bounds_guard(
    guard: &Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Camera2D) -> () + Send + 'static>>>,
    >,
) -> () {
    (*DEGENERATE_VISIBLE_BOUNDS_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/camera/src/visibleBounds.ts:45 (sha256:fe1df067ae72b070f8eefe7927c49a15156cd5f1265f37f70c3b4a6c7cb298eb)
static UNBOUNDED_EXTENT: std::sync::LazyLock<f64> = std::sync::LazyLock::new(|| f64::MAX);

// Source: upstream/packages/camera/src/visibleBounds.ts:46 (sha256:c9b44dad173d8b82b7aecc2f127619626b94222aa9bdf8615b5d5b864ad2ec94)
static UNBOUNDED_ORIGIN: std::sync::LazyLock<f64> =
    std::sync::LazyLock::new(|| ((-f64::MAX) / 2.0_f64));

// Source: upstream/packages/camera/src/visibleBounds.ts:48 (sha256:66c3402002d6d9e174035f625959c44a130d6816837e32a09e6e2d86e0e721ce)
static DEGENERATE_VISIBLE_BOUNDS_GUARD: std::sync::LazyLock<
    std::sync::Mutex<
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Camera2D) -> () + Send + 'static>>>>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/camera/src/visibleBounds.ts:49 (sha256:5a5912d6fd6c54ce85b8340e33c7399d57ff876b273e4908ca3da2e3c0e4d054)
static SCRATCH_INVERSE: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });

// Source: upstream/packages/camera/src/visibleBounds.ts:50 (sha256:bd1c7961ccdf3d194ced82b6528787dba39e75a0c7340b479ed42f02a0bfe67b)
static SCRATCH_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });
