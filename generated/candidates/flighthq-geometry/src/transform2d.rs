// @generated from upstream/packages/geometry/src/transform2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_math::RAD_TO_DEG as rad_to_deg_constant;
use flighthq_types::{MatrixLike, Transform2D, Transform2DLike};

// Source: upstream/packages/geometry/src/transform2d.ts:7 (sha256:6ce0056dbae519968f71d85e4880231545ea7fdcddbcd5330a2f602062721c15)
pub fn create_transform2_d(
    x: Option<f64>,
    y: Option<f64>,
    rotation: Option<f64>,
    scale_x: Option<f64>,
    scale_y: Option<f64>,
    skew_x: Option<f64>,
    skew_y: Option<f64>,
    pivot_x: Option<f64>,
    pivot_y: Option<f64>,
) -> Transform2D {
    return create_entity(Some(Transform2D {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        pivot_x: (pivot_x).unwrap_or(0.0_f64),
        pivot_y: (pivot_y).unwrap_or(0.0_f64),
        rotation: (rotation).unwrap_or(0.0_f64),
        scale_x: (scale_x).unwrap_or(1.0_f64),
        scale_y: (scale_y).unwrap_or(1.0_f64),
        skew_x: (skew_x).unwrap_or(0.0_f64),
        skew_y: (skew_y).unwrap_or(0.0_f64),
        x: (x).unwrap_or(0.0_f64),
        y: (y).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/transform2d.ts:37 (sha256:8a448cc78e4a55f672d7ba6915f25be61f8c1727ef632cfd3706d039132f6354)
pub fn decompose_matrix_to_transform2_d(out: &mut Transform2DLike, source: &MatrixLike) -> () {
    let a = source.a;
    let b = source.b;
    let c = source.c;
    let d = source.d;
    let scale_x = ((a * a) + (b * b)).sqrt();
    let reflected = (((a * d) - (b * c)) < 0.0_f64);
    let scale_y = if reflected {
        (-((c * c) + (d * d)).sqrt())
    } else {
        ((c * c) + (d * d)).sqrt()
    };
    let skew_x_degrees = (if reflected {
        (c).atan2((-d))
    } else {
        (-c).atan2(d)
    } * rad_to_deg_constant);
    let skew_y_degrees = ((b).atan2(a) * rad_to_deg_constant);
    if (skew_x_degrees == skew_y_degrees) {
        out.rotation = skew_y_degrees;
        out.skew_x = 0.0_f64;
        out.skew_y = 0.0_f64;
    } else {
        out.rotation = 0.0_f64;
        out.skew_x = skew_x_degrees;
        out.skew_y = skew_y_degrees;
    }
    out.pivot_x = 0.0_f64;
    out.pivot_y = 0.0_f64;
    out.scale_x = scale_x;
    out.scale_y = scale_y;
    out.x = source.tx;
    out.y = source.ty;
}
