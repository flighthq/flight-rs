// @generated from upstream/packages/math/src/angle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    DEG_TO_RAD as deg_to_rad_constant, RAD_TO_DEG as rad_to_deg_constant, TAU as tau_constant,
};

// Source: upstream/packages/math/src/angle.ts:4 (sha256:e9ad5523df27a17708dd34966bdbc65b502483529620a9895a734311dd75e79f)
pub fn deg_to_rad(degrees: f64) -> f64 {
    return (degrees * deg_to_rad_constant);
}

// Source: upstream/packages/math/src/angle.ts:13 (sha256:0da500b2a617a5d8ecd225ef55da2e64e83f5ec32aa2f5f27914a085d2fd6f67)
pub fn delta_angle(from: f64, to: f64) -> f64 {
    let diff = ((((to - from) % tau_constant) + tau_constant) % tau_constant);
    return if (diff > std::f64::consts::PI) {
        (diff - tau_constant)
    } else {
        diff
    };
}

// Source: upstream/packages/math/src/angle.ts:19 (sha256:f8d831f4f664221dc877d8ebed260bc6ce89b63ecf02922a1dcf7256b5f8b389)
pub fn normalize_angle(radians: f64) -> f64 {
    let wrapped = (((radians % tau_constant) + tau_constant) % tau_constant);
    return if (wrapped >= std::f64::consts::PI) {
        (wrapped - tau_constant)
    } else {
        wrapped
    };
}

// Source: upstream/packages/math/src/angle.ts:25 (sha256:e712edc7c0b60826376f14a3dacd8d1555baaf7ba5759875df83e1620c7f5fab)
pub fn rad_to_deg(radians: f64) -> f64 {
    return (radians * rad_to_deg_constant);
}
