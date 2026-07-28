// @generated from upstream/packages/camera/src/shadowCamera.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    OrthographicProjectionOptions, create_orthographic_projection,
    set_camera_view_matrix4_from_look_at,
};
use flighthq_types::{AabbLike, Camera, Vector3Like};

// Source: upstream/packages/camera/src/shadowCamera.ts:14 (sha256:eb979e2b4bf5c3c206dd3d53c7d12f8aa5a828040d8344420ca9ce2076ad42ae)
pub fn configure_directional_shadow_camera(
    camera: &mut Camera,
    light_direction: &Vector3Like,
    scene_bounds: &AabbLike,
) -> () {
    let cx = ((scene_bounds.min.x + scene_bounds.max.x) * 0.5_f64);
    let cy = ((scene_bounds.min.y + scene_bounds.max.y) * 0.5_f64);
    let cz = ((scene_bounds.min.z + scene_bounds.max.z) * 0.5_f64);
    let mut radius = ((scene_bounds.max.x - cx).powi(2)
        + (scene_bounds.max.y - cy).powi(2)
        + (scene_bounds.max.z - cz).powi(2))
    .sqrt();
    if (radius == 0.0_f64) {
        radius = 1.0_f64;
    }
    let dl =
        (((light_direction.x).powi(2) + (light_direction.y).powi(2) + (light_direction.z).powi(2))
            .sqrt()
            || 1.0_f64);
    let dx = (light_direction.x / dl);
    let dy = (light_direction.y / dl);
    let dz = (light_direction.z / dl);
    let distance = (radius * 2.0_f64);
    (*_EYE.lock().unwrap()).x = (cx - (dx * distance));
    (*_EYE.lock().unwrap()).y = (cy - (dy * distance));
    (*_EYE.lock().unwrap()).z = (cz - (dz * distance));
    (*_TARGET.lock().unwrap()).x = cx;
    (*_TARGET.lock().unwrap()).y = cy;
    (*_TARGET.lock().unwrap()).z = cz;
    let up = if ((dy).abs() > 0.99_f64) {
        ((*_UP_Z).clone()).clone()
    } else {
        ((*_UP_Y).clone()).clone()
    };
    set_camera_view_matrix4_from_look_at(
        camera,
        &(*_EYE.lock().unwrap()),
        &(*_TARGET.lock().unwrap()),
        &up,
    );
    camera.near = radius;
    camera.far = (radius * 3.0_f64);
    camera.projection = create_orthographic_projection(&OrthographicProjectionOptions {
        __flight_identity: std::sync::Arc::new(()),
        half_height: radius,
        half_width: radius,
    });
}

// Source: upstream/packages/camera/src/shadowCamera.ts:50 (sha256:f905567836d6d1525a8a8a4c8f583b89fe0b92ac550f2ea16e2ae37619f7b39e)
static _EYE: std::sync::LazyLock<std::sync::Mutex<Vector3Like>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector3Like {
        __flight_identity: std::sync::Arc::new(()),
        x: 0.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    })
});

// Source: upstream/packages/camera/src/shadowCamera.ts:51 (sha256:08ea600988417111f59dbabd52f237be39d8058cf35921fdea2f739efe83cc41)
static _TARGET: std::sync::LazyLock<std::sync::Mutex<Vector3Like>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(Vector3Like {
            __flight_identity: std::sync::Arc::new(()),
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f64,
        })
    });

// Source: upstream/packages/camera/src/shadowCamera.ts:52 (sha256:9d8d483e494babc422e1a78566746f1abcff2c7c5f464f53116565950a0f3dde)
struct _upY;
impl _upY {
    pub const x: f64 = 0.0_f64;
    pub const y: f64 = 1.0_f64;
    pub const z: f64 = 0.0_f64;
}

// Source: upstream/packages/camera/src/shadowCamera.ts:53 (sha256:2e44e7b15a62c7b63d69d3f9edfd7ffae4d503734d7c2460349e0f8b8bc65a4e)
struct _upZ;
impl _upZ {
    pub const x: f64 = 0.0_f64;
    pub const y: f64 = 0.0_f64;
    pub const z: f64 = 1.0_f64;
}
