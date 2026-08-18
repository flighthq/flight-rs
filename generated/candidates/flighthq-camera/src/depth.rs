// @generated from upstream/packages/camera/src/depth.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Camera3D;

// Source: upstream/packages/camera/src/depth.ts:14 (sha256:d43b0a6a9ab32860642ff8a507b8775d25f4b5ea610e5a93904422d9a6de0efd)
pub fn get_camera3_d_linear_depth(camera: &Camera3D, ndc_z: f64) -> f64 {
    let near = camera.near;
    let far = camera.far;
    let range = (far - near);
    if (range == 0.0_f64) {
        return 0.0_f64;
    }
    if (camera.projection.kind == "orthographic") {
        return (-(near + (((ndc_z + 1.0_f64) * range) / 2.0_f64)));
    }
    let denominator = ((ndc_z * range) - (far + near));
    if (denominator == 0.0_f64) {
        return 0.0_f64;
    }
    return (((2.0_f64 * far) * near) / denominator);
}

// Source: upstream/packages/camera/src/depth.ts:41 (sha256:e9de24c1e33c08e4ab7cc75eefadf4ef39f7d91810f1d44e98faf5046fed6696)
pub fn get_camera3_d_view_space_z(camera: &Camera3D, ndc_z: f64) -> f64 {
    return (-get_camera3_d_linear_depth(camera, ndc_z));
}
