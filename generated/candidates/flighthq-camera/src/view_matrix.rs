// @generated from upstream/packages/camera/src/viewMatrix.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{set_transform_matrix, translate_matrix_by_vector_xy};
use flighthq_types::{Camera2D, MatrixLike};

// Source: upstream/packages/camera/src/viewMatrix.ts:16 (sha256:20cd5e745310990406a81ff3f3550cee9439a743b0c3e0f789c3a885318893c1)
pub fn get_camera2_d_view_matrix(camera: &Camera2D, out: &mut MatrixLike) -> () {
    let zoom = camera.zoom;
    set_transform_matrix(
        out,
        zoom,
        zoom,
        Some((-camera.rotation)),
        Some((camera.viewport_width * 0.5_f64)),
        Some((camera.viewport_height * 0.5_f64)),
    );
    {
        let __flight_argument_1 = (out).clone();
        let __flight_result =
            translate_matrix_by_vector_xy(out, &__flight_argument_1, (-camera.x), (-camera.y));
        __flight_result
    };
}
