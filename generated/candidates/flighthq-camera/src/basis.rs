// @generated from upstream/packages/camera/src/basis.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Camera3D, Vector3Like};

// Source: upstream/packages/camera/src/basis.ts:8 (sha256:acc95f98bf7beddd35b5759dd4a7f4577ca47e6aeddc5ae378ee478201fbba5a)
pub fn get_camera3_d_forward(out: &mut Vector3Like, camera: &Camera3D) -> () {
    out.x = (-(camera.view.m[2.0_f64 as usize] as f64));
    out.y = (-(camera.view.m[6.0_f64 as usize] as f64));
    out.z = (-(camera.view.m[10.0_f64 as usize] as f64));
}

// Source: upstream/packages/camera/src/basis.ts:26 (sha256:935bebcfbbd43f035783b5a1e5c26640ce23835fece339efdcca44822e6e7627)
pub fn get_camera3_d_position(out: &mut Vector3Like, camera: &Camera3D) -> () {
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

// Source: upstream/packages/camera/src/basis.ts:55 (sha256:19da344f6c67806949f7e02094039bbbd7e020532ff35ea63483ab0cf226b09c)
pub fn get_camera3_d_right(out: &mut Vector3Like, camera: &Camera3D) -> () {
    out.x = (camera.view.m[0.0_f64 as usize] as f64);
    out.y = (camera.view.m[4.0_f64 as usize] as f64);
    out.z = (camera.view.m[8.0_f64 as usize] as f64);
}

// Source: upstream/packages/camera/src/basis.ts:66 (sha256:6cb846478b344018a2c8459e0931c75eaf2d7d2470eb1531fbdc6e45e8c65789)
pub fn get_camera3_d_up(out: &mut Vector3Like, camera: &Camera3D) -> () {
    out.x = (camera.view.m[1.0_f64 as usize] as f64);
    out.y = (camera.view.m[5.0_f64 as usize] as f64);
    out.z = (camera.view.m[9.0_f64 as usize] as f64);
}
