// @generated from upstream/packages/math/src/interpolation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/math/src/interpolation.ts:6 (sha256:2298cd224fbae6fa98967bfbf1f2808fac578b8686d6dc65091b5a2dc5cc9d8b)
pub fn inverse_lerp(a: f64, b: f64, value: f64) -> f64 {
    let range = (b - a);
    return if (range == 0.0_f64) {
        0.0_f64
    } else {
        ((value - a) / range)
    };
}

// Source: upstream/packages/math/src/interpolation.ts:16 (sha256:3e55beef1130cb67ae453ab6b7197d2fab06bb18a2e6cb7d8ae45928a75c3765)
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return (a + ((b - a) * t));
}

// Source: upstream/packages/math/src/interpolation.ts:25 (sha256:d62010e70291d43e82cf176e8db54d8a01b5f98503737da35e998935f0ad5dc0)
pub fn remap(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    let in_range = (in_max - in_min);
    if (in_range == 0.0_f64) {
        return out_min;
    }
    return (out_min + (((value - in_min) / in_range) * (out_max - out_min)));
}

// Source: upstream/packages/math/src/interpolation.ts:36 (sha256:e68eb63c477b3eda7fcda77ac52dbad20a2592aaffbc9ff57d007a7bf472fe0a)
pub fn smooth_step(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0));
    let s = if (t < 0.0_f64) {
        0.0_f64
    } else {
        if (t > 1.0_f64) { 1.0_f64 } else { t }
    };
    return ((s * s) * (3.0_f64 - (2.0_f64 * s)));
}

// Source: upstream/packages/math/src/interpolation.ts:43 (sha256:9152a4b3d89e973556524b4fb221e55a8c64b1ac9a32848c0f8512b9bf1c8f13)
pub fn step(edge: f64, x: f64) -> f64 {
    return if (x < edge) { 0.0_f64 } else { 1.0_f64 };
}
