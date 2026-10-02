// @generated from upstream/packages/camera/src/zoom.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::unproject_camera2_d_point;
use flighthq_geometry::create_vector2;
use flighthq_types::{Camera2D, Vector2};

// Source: upstream/packages/camera/src/zoom.ts:13 (sha256:de15bce2eeaf8028766e3ae18348b87b89473e9b3f4ba89572c1aa85f6891e23)
pub fn zoom_camera2_d_at_screen_point(
    camera: &mut Camera2D,
    viewport_width: f64,
    viewport_height: f64,
    screen_x: f64,
    screen_y: f64,
    zoom: f64,
) -> () {
    unproject_camera2_d_point(
        camera,
        viewport_width,
        viewport_height,
        screen_x,
        screen_y,
        &mut (*SCRATCH_BEFORE.lock().unwrap()),
    );
    camera.zoom = zoom;
    unproject_camera2_d_point(
        camera,
        viewport_width,
        viewport_height,
        screen_x,
        screen_y,
        &mut (*SCRATCH_AFTER.lock().unwrap()),
    );
    camera.x += ((*SCRATCH_BEFORE.lock().unwrap()).x - (*SCRATCH_AFTER.lock().unwrap()).x);
    camera.y += ((*SCRATCH_BEFORE.lock().unwrap()).y - (*SCRATCH_AFTER.lock().unwrap()).y);
}

// Source: upstream/packages/camera/src/zoom.ts:28 (sha256:1cc3e16a453c7b97dc36ed19fef41ccc879b75e030c86207256116abbe67c7c4)
static SCRATCH_AFTER: std::sync::LazyLock<std::sync::Mutex<Vector2>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector2(None, None)));

// Source: upstream/packages/camera/src/zoom.ts:29 (sha256:9ca507fb0c9e7a79b15ce1e3f101fa2bd1b1512e8c9d5d3fc3ca4fc93fd9bcbe)
static SCRATCH_BEFORE: std::sync::LazyLock<std::sync::Mutex<Vector2>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector2(None, None)));
