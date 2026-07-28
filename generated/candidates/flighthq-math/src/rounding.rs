// @generated from upstream/packages/math/src/rounding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/math/src/rounding.ts:5 (sha256:d68a9ef29b72f03c124120f82afa9846cbc3c353878ae46019a6caafbfcd2263)
pub fn ceil_to(value: f64, step: f64) -> f64 {
    if (step <= 0.0_f64) {
        return value;
    }
    return ((value / step).ceil() * step);
}

// Source: upstream/packages/math/src/rounding.ts:21 (sha256:a921d4c2afb8bab48b54093a4e85999dc2aece23a0f82f8d9b9e4c6a374270f2)
pub fn euclidean_mod(value: f64, divisor: f64) -> f64 {
    if (divisor == 0.0_f64) {
        panic!("{}", "generated Flight function threw");
    }
    return (((value % divisor) + divisor) % divisor);
}

// Source: upstream/packages/math/src/rounding.ts:30 (sha256:1a5b372c164abe9dd6b621aef6576c4abfcf28ea605e917b0d871382f58d419d)
pub fn floor_to(value: f64, step: f64) -> f64 {
    if (step <= 0.0_f64) {
        return value;
    }
    return ((value / step).floor() * step);
}

// Source: upstream/packages/math/src/rounding.ts:40 (sha256:1d2f328cb7fc8c9f288aff2d829bec51e245e9fb658be25d83087e74a702ba72)
pub fn fract(value: f64) -> f64 {
    return (value - (value).trunc());
}

// Source: upstream/packages/math/src/rounding.ts:49 (sha256:930f63a2f47ae5e5b9c4047fc72f04d64def424775bb193df8fa19f21b391409)
pub fn round_to(value: f64, step: f64) -> f64 {
    if (step <= 0.0_f64) {
        return value;
    }
    return ((value / step).round() * step);
}
