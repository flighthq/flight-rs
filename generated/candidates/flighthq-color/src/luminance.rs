// @generated from upstream/packages/color/src/luminance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::srgb_channel_to_linear;

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

// Source: upstream/packages/color/src/luminance.ts:6 (sha256:640b8755719a84dc7ecc7562902e1f8d77171102fefd7926e663bc23bbd66de8)
pub fn get_color_contrast_ratio(a: f64, b: f64) -> f64 {
    let la = get_color_luminance(a);
    let lb = get_color_luminance(b);
    let lighter = (la).max(lb);
    let darker = (la).min(lb);
    return ((lighter + 0.05_f64) / (darker + 0.05_f64));
}

// Source: upstream/packages/color/src/luminance.ts:17 (sha256:647b7af039964d68f0f420c35b7f060228d95d994c611381bb773912cfbc3cae)
pub fn get_color_luminance(color: f64) -> f64 {
    let r = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let g = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    let b = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    return (((0.2126_f64 * r) + (0.7152_f64 * g)) + (0.0722_f64 * b));
}

// Source: upstream/packages/color/src/luminance.ts:26 (sha256:508d9693c6028b1e35117eadf038ee6df7d9c6aeee5e4b6f1326acfedb99f57f)
pub fn get_rec2020_luminance_weights(out: &mut Vec<f64>) -> () {
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = 0.2627_f64;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = 0.678_f64;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = 0.0593_f64;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/color/src/luminance.ts:34 (sha256:f5c5f394f0063d151af6e51f78dada78bdc339bc7560f07c2e0b01e3de185ab7)
pub fn get_rec709_luminance_weights(out: &mut Vec<f64>) -> () {
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = 0.2126_f64;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = 0.7152_f64;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = 0.0722_f64;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}
