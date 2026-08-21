// @generated from upstream/packages/easing/src/easeBounce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeBounce.ts:3 (sha256:76adfe8205c912b164dd16eeaaf8dd10d1d34bbd133a41a66aed849847af5495)
pub fn ease_in_bounce(t: f64) -> f64 {
    return (1.0_f64 - bounce_out((1.0_f64 - t)));
}

// Source: upstream/packages/easing/src/easeBounce.ts:5 (sha256:5811895ae55c72f2443deeea8f3ff1fd3ae47686bdaacad35eefe4fbd8710b03)
pub fn ease_in_out_bounce(t: f64) -> f64 {
    return if (t < 0.5_f64) {
        ((1.0_f64 - bounce_out((1.0_f64 - (2.0_f64 * t)))) / 2.0_f64)
    } else {
        ((1.0_f64 + bounce_out(((2.0_f64 * t) - 1.0_f64))) / 2.0_f64)
    };
}

// Source: upstream/packages/easing/src/easeBounce.ts:8 (sha256:bb6bfe554356b17dd881e25a99cedfb713ca48dae0401d8d33de9ee1116cbf10)
pub fn ease_out_bounce(t: f64) -> f64 {
    return bounce_out(t);
}

// Source: upstream/packages/easing/src/easeBounce.ts:10 (sha256:cb2180c15f9f37d7bdb399c8e57c0f24eceed007fc59bce8d818d74982083a5f)
fn bounce_out(mut t: f64) -> f64 {
    if (t < (1.0_f64 / 2.75_f64)) {
        return ((7.5625_f64 * t) * t);
    }
    if (t < (2.0_f64 / 2.75_f64)) {
        return (((7.5625_f64 * {
            t -= (1.5_f64 / 2.75_f64);
            t.clone()
        }) * t)
            + 0.75_f64);
    }
    if (t < (2.5_f64 / 2.75_f64)) {
        return (((7.5625_f64 * {
            t -= (2.25_f64 / 2.75_f64);
            t.clone()
        }) * t)
            + 0.9375_f64);
    }
    return (((7.5625_f64 * {
        t -= (2.625_f64 / 2.75_f64);
        t.clone()
    }) * t)
        + 0.984375_f64);
}
