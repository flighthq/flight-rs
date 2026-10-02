// @generated from upstream/packages/geometry/src/transform2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_math::RAD_TO_DEG as rad_to_deg_constant;
use flighthq_types::{EntityConstruction, MatrixLike, Transform2D, Transform2DLike};

// Source: upstream/packages/geometry/src/transform2d.ts:7 (sha256:1a5dc6ace4461f85fd76e87f80afd0bf2910a01313ca0c2de5bc5dcf9313397a)
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
    let mut out = allocate_entity();
    initialize_transform2_d(
        (out).clone(),
        (x).unwrap_or(0.0_f64),
        (y).unwrap_or(0.0_f64),
        (rotation).unwrap_or(0.0_f64),
        (scale_x).unwrap_or(1.0_f64),
        (scale_y).unwrap_or(1.0_f64),
        (skew_x).unwrap_or(0.0_f64),
        (skew_y).unwrap_or(0.0_f64),
        (pivot_x).unwrap_or(0.0_f64),
        (pivot_y).unwrap_or(0.0_f64),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/geometry/src/transform2d.ts:40 (sha256:8a448cc78e4a55f672d7ba6915f25be61f8c1727ef632cfd3706d039132f6354)
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

// Source: upstream/packages/geometry/src/transform2d.ts:72 (sha256:a50ffeb4b2c36970dba0eaec42b893125cfd66fabc3b27eac276ea6ced5ed3c8)
pub fn initialize_transform2_d(
    out: EntityConstruction<Transform2D>,
    x: f64,
    y: f64,
    rotation: f64,
    scale_x: f64,
    scale_y: f64,
    skew_x: f64,
    skew_y: f64,
    pivot_x: f64,
    pivot_y: f64,
) -> () {
    crate::host_set("host.x", x);
    crate::host_set("host.y", y);
    crate::host_set("host.rotation", rotation);
    crate::host_set("host.scaleX", scale_x);
    crate::host_set("host.scaleY", scale_y);
    crate::host_set("host.skewX", skew_x);
    crate::host_set("host.skewY", skew_y);
    crate::host_set("host.pivotX", pivot_x);
    crate::host_set("host.pivotY", pivot_y);
}
