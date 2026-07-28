// @generated from upstream/packages/math/src/comparison.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EPSILON as epsilon_constant;

// Source: upstream/packages/math/src/comparison.ts:8 (sha256:21bf28c6c3f01d78d03ffe5659bdf3216ad93e48b30d07707182aeac923f69d9)
pub fn approx_equal(a: f64, b: f64, epsilon: Option<f64>) -> bool {
    let epsilon = epsilon.unwrap_or(epsilon_constant);
    return ((a - b).abs() <= epsilon);
}

// Source: upstream/packages/math/src/comparison.ts:19 (sha256:4d8b3171f64862721bf124df409487764d96a29c109fa6cae951046326c94e71)
pub fn approx_equal_relative(a: f64, b: f64, relative_epsilon: Option<f64>) -> bool {
    let relative_epsilon = relative_epsilon.unwrap_or(epsilon_constant);
    let diff = (a - b).abs();
    let largest = ((a).abs()).max((b).abs());
    return (diff <= (relative_epsilon * largest).max(epsilon_constant));
}

// Source: upstream/packages/math/src/comparison.ts:26 (sha256:3937e3bb9d2f8001e466c75b1b546759e8da2d17dcb72b54ac6db6acdac3d786)
pub fn approx_zero(value: f64, epsilon: Option<f64>) -> bool {
    let epsilon = epsilon.unwrap_or(epsilon_constant);
    return ((value).abs() <= epsilon);
}
