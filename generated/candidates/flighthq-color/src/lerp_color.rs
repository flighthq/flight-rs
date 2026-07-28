// @generated from upstream/packages/color/src/lerpColor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{pack_linear_to_color, srgb_channel_to_linear};
use flighthq_types::LinearColor;

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/color/src/lerpColor.ts:10 (sha256:0af99d7aad5401eaf27c042095c568b4f3a2ed2faeca58ab500a4da456acb0bb)
pub fn lerp_color(start: f64, end: f64, t: f64) -> f64 {
    let tc = (1.0_f64).min((0.0_f64).max(t));
    let sr = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(start) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let sg = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(start) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let sb = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(start) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let sa = ((__flight_js_to_i32(start) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    let er = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(end) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let eg = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(end) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let eb = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(end) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let ea = ((__flight_js_to_i32(end) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    let r = (sr + ((er - sr) * tc));
    let g = (sg + ((eg - sg) * tc));
    let b = (sb + ((eb - sb) * tc));
    let a = (sa + ((ea - sa) * tc));
    return pack_linear_to_color(&vec![r, g, b, a]);
}

// Source: upstream/packages/color/src/lerpColor.ts:29 (sha256:8a2bd5375ebab48f930029badd636774a0df99a68f8644384a45298517f87da8)
pub fn lerp_linear_color(
    out: &mut LinearColor,
    start: &LinearColor,
    end: &LinearColor,
    t: f64,
) -> LinearColor {
    let tc = (1.0_f64).min((0.0_f64).max(t));
    let r0 = start[0.0_f64 as usize].clone();
    let g0 = start[1.0_f64 as usize].clone();
    let b0 = start[2.0_f64 as usize].clone();
    let a0 = start[3.0_f64 as usize].clone();
    let r1 = end[0.0_f64 as usize].clone();
    let g1 = end[1.0_f64 as usize].clone();
    let b1 = end[2.0_f64 as usize].clone();
    let a1 = end[3.0_f64 as usize].clone();
    out[0.0_f64 as usize] = (r0 + ((r1 - r0) * tc));
    out[1.0_f64 as usize] = (g0 + ((g1 - g0) * tc));
    out[2.0_f64 as usize] = (b0 + ((b1 - b0) * tc));
    out[3.0_f64 as usize] = (a0 + ((a1 - a0) * tc));
    return out.clone();
}
