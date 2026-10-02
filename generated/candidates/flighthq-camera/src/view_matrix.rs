// @generated from upstream/packages/camera/src/viewMatrix.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{set_transform_matrix, translate_matrix_by_vector_xy};
use flighthq_types::{Camera2D, MatrixLike};

// Source: upstream/packages/camera/src/viewMatrix.ts:16 (sha256:9f0fd38765bdd6cad01950ffe542df7b8f498af54d5d4e7c5ca5ac3c3396d097)
pub fn get_camera2_d_view_matrix(
    camera: &Camera2D,
    viewport_width: f64,
    viewport_height: f64,
    out: &mut MatrixLike,
) -> () {
    let zoom = camera.zoom;
    set_transform_matrix(
        out,
        zoom,
        zoom,
        Some((-camera.rotation)),
        Some((viewport_width * 0.5_f64)),
        Some((viewport_height * 0.5_f64)),
    );
    {
        let __flight_argument_1 = (out).clone();
        let __flight_result =
            translate_matrix_by_vector_xy(out, &__flight_argument_1, (-camera.x), (-camera.y));
        __flight_result
    };
}
