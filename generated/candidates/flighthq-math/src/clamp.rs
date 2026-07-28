// @generated from upstream/packages/math/src/clamp.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/math/src/clamp.ts:5 (sha256:fb5c6e519b477b91cd032d5bc89a282188697e8fd36e3866ddb4710c1ceed37b)
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    return if (value < min) {
        min
    } else {
        if (value > max) { max } else { value }
    };
}

// Source: upstream/packages/math/src/clamp.ts:10 (sha256:cad3641434ac42729d3e837e869b3e6c11e1cd56795d8486da2f62c2dc90cdfc)
pub fn in_range(value: f64, min: f64, max: f64) -> bool {
    return (value >= min) && (value <= max);
}

// Source: upstream/packages/math/src/clamp.ts:20 (sha256:142b6e44f5644295d5df65fc924152079714b792bd6a4703b1605c82df4bfb3d)
pub fn saturate(value: f64) -> f64 {
    if (value != value) {
        return 0.0_f64;
    }
    return if (value < 0.0_f64) {
        0.0_f64
    } else {
        if (value > 1.0_f64) { 1.0_f64 } else { value }
    };
}
