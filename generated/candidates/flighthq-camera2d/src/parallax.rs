// @generated from upstream/packages/camera2d/src/parallax.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_camera2_d_view_matrix;
use flighthq_geometry::create_matrix;
use flighthq_types::{Camera2D, Matrix, Vector2Like};

// Source: upstream/packages/camera2d/src/parallax.ts:20 (sha256:962087bbafadcdeaebc43af571e6fa5dc60e938a1e66c11737ec974badfd1bdb)
pub fn get_camera2_d_parallax_point(camera: &Camera2D, factor: f64, out: &mut Vector2Like) -> () {
    get_camera2_d_view_matrix(camera, &mut (*SCRATCH_MATRIX.lock().unwrap()));
    out.x = (((*SCRATCH_MATRIX.lock().unwrap()).tx - (camera.viewport_width * 0.5_f64)) * factor);
    out.y = (((*SCRATCH_MATRIX.lock().unwrap()).ty - (camera.viewport_height * 0.5_f64)) * factor);
}

// Source: upstream/packages/camera2d/src/parallax.ts:26 (sha256:bd1c7961ccdf3d194ced82b6528787dba39e75a0c7340b479ed42f02a0bfe67b)
static SCRATCH_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix(None, None, None, None, None, None))
    });
