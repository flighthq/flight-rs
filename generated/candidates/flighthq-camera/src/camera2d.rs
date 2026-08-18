// @generated from upstream/packages/camera/src/camera2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Camera2D, Camera2DOptions};

// Source: upstream/packages/camera/src/camera2d.ts:7 (sha256:5f9014b60b0df6f2136ccbe35b5980dd34bf633472e07d7c794d1505903a46c7)
pub fn create_camera2_d(
    viewport_width: f64,
    viewport_height: f64,
    options: Option<Camera2DOptions>,
) -> Camera2D {
    return Camera2D {
        __flight_identity: std::sync::Arc::new(()),
        rotation: (options.as_ref().and_then(|value| value.rotation)).unwrap_or(0.0_f64),
        viewport_height: viewport_height,
        viewport_width: viewport_width,
        x: (options.as_ref().and_then(|value| value.x)).unwrap_or(0.0_f64),
        y: (options.as_ref().and_then(|value| value.y)).unwrap_or(0.0_f64),
        zoom: (options.as_ref().and_then(|value| value.zoom)).unwrap_or(1.0_f64),
    };
}
