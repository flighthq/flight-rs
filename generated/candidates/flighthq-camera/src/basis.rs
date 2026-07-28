// @generated from upstream/packages/camera/src/basis.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Camera, Vector3Like};

// Source: upstream/packages/camera/src/basis.ts:8 (sha256:b3eb6db55562d33d819cb987deac67f96206d01640188d29c61ac9b1af458089)
pub fn get_camera_forward(out: &mut Vector3Like, camera: &Camera) -> () {
    out.x = (-(camera.view.m[2.0_f64 as usize] as f64));
    out.y = (-(camera.view.m[6.0_f64 as usize] as f64));
    out.z = (-(camera.view.m[10.0_f64 as usize] as f64));
}

// Source: upstream/packages/camera/src/basis.ts:26 (sha256:4b8d7e451a489413e8fbf6154fb39f45cb45138739875e1e606e764c5412dc17)
pub fn get_camera_position(out: &mut Vector3Like, camera: &Camera) -> () {
    let m00 = (camera.view.m[0.0_f64 as usize] as f64);
    let m01 = (camera.view.m[1.0_f64 as usize] as f64);
    let m02 = (camera.view.m[2.0_f64 as usize] as f64);
    let m10 = (camera.view.m[4.0_f64 as usize] as f64);
    let m11 = (camera.view.m[5.0_f64 as usize] as f64);
    let m12 = (camera.view.m[6.0_f64 as usize] as f64);
    let m20 = (camera.view.m[8.0_f64 as usize] as f64);
    let m21 = (camera.view.m[9.0_f64 as usize] as f64);
    let m22 = (camera.view.m[10.0_f64 as usize] as f64);
    let tx = (camera.view.m[12.0_f64 as usize] as f64);
    let ty = (camera.view.m[13.0_f64 as usize] as f64);
    let tz = (camera.view.m[14.0_f64 as usize] as f64);
    out.x = (-(((m00 * tx) + (m01 * ty)) + (m02 * tz)));
    out.y = (-(((m10 * tx) + (m11 * ty)) + (m12 * tz)));
    out.z = (-(((m20 * tx) + (m21 * ty)) + (m22 * tz)));
}

// Source: upstream/packages/camera/src/basis.ts:59 (sha256:b17c4de257657155b4ae7407b5c9cde27caeffb02b4aa6c159b8e06be161cf44)
pub fn get_camera_right(out: &mut Vector3Like, camera: &Camera) -> () {
    out.x = (camera.view.m[0.0_f64 as usize] as f64);
    out.y = (camera.view.m[4.0_f64 as usize] as f64);
    out.z = (camera.view.m[8.0_f64 as usize] as f64);
}

// Source: upstream/packages/camera/src/basis.ts:70 (sha256:26025b7cca672cf20cca657d452422b7f16f700ff26ac506d03ef01269740dc7)
pub fn get_camera_up(out: &mut Vector3Like, camera: &Camera) -> () {
    out.x = (camera.view.m[1.0_f64 as usize] as f64);
    out.y = (camera.view.m[5.0_f64 as usize] as f64);
    out.z = (camera.view.m[9.0_f64 as usize] as f64);
}
