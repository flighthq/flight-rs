// @generated from upstream/packages/math/src/interpolationAdvanced.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/math/src/interpolationAdvanced.ts:10 (sha256:1c03fab6ab24e491203839af71f3af1dafad5d185d7c4f309d5089fb6f351d21)
pub fn damp(current: f64, target: f64, lambda: f64, delta_time: f64) -> f64 {
    if (delta_time <= 0.0_f64) || (lambda <= 0.0_f64) {
        return current;
    }
    return (target + ((current - target) * ((-lambda) * delta_time).exp()));
}

// Source: upstream/packages/math/src/interpolationAdvanced.ts:22 (sha256:38a84a3e89a4d88bf3f521d4483d05f53ec650494d9a9a32332f73c5dfe59a43)
pub fn lerp_angle(a: f64, b: f64, t: f64) -> f64 {
    let tau = (std::f64::consts::PI * 2.0_f64);
    let mut diff = ((((b - a) % tau) + tau) % tau);
    if (diff > std::f64::consts::PI) {
        diff -= tau;
    }
    return (a + (diff * t));
}

// Source: upstream/packages/math/src/interpolationAdvanced.ts:34 (sha256:1b7665e47fd1cceb65437dcf319ee8b1c1b81c97a5b78f2d39b8b42b24c7e5ec)
pub fn move_towards(current: f64, target: f64, max_delta: f64) -> f64 {
    let delta = (target - current);
    if ((delta).abs() <= max_delta) {
        return target;
    }
    return (current
        + ({
            let __flight_value = delta;
            if __flight_value.is_nan() || __flight_value == 0.0 {
                __flight_value
            } else {
                __flight_value.signum()
            }
        } * max_delta));
}

// Source: upstream/packages/math/src/interpolationAdvanced.ts:44 (sha256:143617a52088ac823b0b1fc6b3881062b71198923e4d34381f7265cef539015c)
pub fn ping_pong(t: f64, length: f64) -> f64 {
    let cycle = (2.0_f64 * length);
    let mod_ = (((t % cycle) + cycle) % cycle);
    return if (mod_ <= length) {
        mod_
    } else {
        (cycle - mod_)
    };
}

// Source: upstream/packages/math/src/interpolationAdvanced.ts:58 (sha256:4043176670291537fd555f4934496882bdbeffa700b9e669b5a89e8e21219ee5)
pub fn repeat(t: f64, length: f64) -> f64 {
    return (((t % length) + length) % length);
}

// Source: upstream/packages/math/src/interpolationAdvanced.ts:72 (sha256:67e9832faf05b12214cf342c5996deacb2274acd920dd22244eae3b2bb832262)
pub fn smoother_step(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0));
    let s = if (t < 0.0_f64) {
        0.0_f64
    } else {
        if (t > 1.0_f64) { 1.0_f64 } else { t }
    };
    return (((s * s) * s) * ((s * ((s * 6.0_f64) - 15.0_f64)) + 10.0_f64));
}
