// @generated from upstream/packages/camera/src/camera2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{Camera2D, Camera2DOptions, EntityConstruction};

// Source: upstream/packages/camera/src/camera2d.ts:7 (sha256:24dd64ac9f051cd8192657f8ece729ca0c138f65de3b7794dc144347411966c9)
pub fn create_camera2_d(options: Option<Camera2DOptions>) -> Camera2D {
    let mut out = allocate_entity();
    initialize_camera2_d((out).clone(), ((options).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/camera/src/camera2d.ts:13 (sha256:cecfbe98aca015897ad285e3a43b6d4c9cb681298bcd7b51b50f29c6491916ca)
pub fn initialize_camera2_d(
    out: EntityConstruction<Camera2D>,
    options: Option<Camera2DOptions>,
) -> () {
    crate::host_set(
        "host.rotation",
        (options.as_ref().and_then(|value| value.rotation)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.x",
        (options.as_ref().and_then(|value| value.x)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.y",
        (options.as_ref().and_then(|value| value.y)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.zoom",
        (options.as_ref().and_then(|value| value.zoom)).unwrap_or(1.0_f64),
    );
}

// Source: upstream/packages/camera/src/camera2d.ts:22 (sha256:5296be06d3625914ff77bba54724890e884fb336d3aae6abdb3595dbfd77c348)
pub fn set_camera2_d_look_at(camera: &mut Camera2D, x: f64, y: f64) -> () {
    camera.x = x;
    camera.y = y;
}
