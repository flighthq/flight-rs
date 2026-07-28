// @generated from upstream/packages/math/src/scalar.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{next_power_of_two, previous_power_of_two};

// Source: upstream/packages/math/src/scalar.ts:8 (sha256:6d095c3cc9d0000af419e3e177ff6f18cf7d82aa0fb3887ca016baa694e2eda2)
pub fn ceil_power_of_two(n: f64) -> f64 {
    return next_power_of_two(n);
}

// Source: upstream/packages/math/src/scalar.ts:17 (sha256:aa88bd9d65c399bfa98cde94ece836c99d5d066f6224c665b78ac6992637e008)
pub fn floor_power_of_two(n: f64) -> f64 {
    return previous_power_of_two(n);
}

// Source: upstream/packages/math/src/scalar.ts:31 (sha256:3e3da3029ce7411777b09437541ec88265ea50159e104262c16a8cbef33e37dc)
pub fn quantize(value: f64, steps: f64, min: f64, max: f64) -> f64 {
    if (steps <= 0.0_f64) || (min == max) {
        return min;
    }
    let t = ((value - min) / (max - min));
    return (min + ((((0.0_f64).max((1.0_f64).min(t)) * steps).round() / steps) * (max - min)));
}

// Source: upstream/packages/math/src/scalar.ts:42 (sha256:410a41b48781c538547599c38c28973f1876958383151e15a4c1a15c9d726889)
pub fn sign(value: f64) -> f64 {
    return if (value > 0.0_f64) {
        1.0_f64
    } else {
        if (value < 0.0_f64) {
            (-1.0_f64)
        } else {
            0.0_f64
        }
    };
}
