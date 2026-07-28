// @generated from upstream/packages/camera/src/depth.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Camera;

// Source: upstream/packages/camera/src/depth.ts:15 (sha256:8b0ac96a3e4e7fde5b18c26793ec4307c041f01a3ce279f2bc5fe3bf77df5253)
pub fn get_camera_linear_depth(camera: &Camera, ndc_z: f64) -> f64 {
    let near = camera.near;
    let far = camera.far;
    let range = (far - near);
    if (range == 0.0_f64) {
        return 0.0_f64;
    }
    let denominator = ((ndc_z * range) - (far + near));
    if (denominator == 0.0_f64) {
        return 0.0_f64;
    }
    return (((2.0_f64 * far) * near) / denominator);
}

// Source: upstream/packages/camera/src/depth.ts:39 (sha256:b0e168a8ead7155f024426c5e876808cf92fc88ee1601a862b44971a5252f84e)
pub fn get_camera_view_space_z(camera: &Camera, ndc_z: f64) -> f64 {
    return (-get_camera_linear_depth(camera, ndc_z));
}
