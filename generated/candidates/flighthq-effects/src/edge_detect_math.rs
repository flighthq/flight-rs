// @generated from upstream/packages/effects/src/edgeDetectMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{OutlineEffect, SketchEffect};

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

// Source: upstream/packages/effects/src/edgeDetectMath.ts:14 (sha256:7ae730f11c39f7d6e2770821ab6f7d4cff153fb3afddfed5706b0f33086a55ad)
pub fn compute_outline_edge_params(effect: &OutlineEffect, out: &mut Vec<f64>) -> () {
    let threshold = (0.0_f64).max((effect.threshold).clone().unwrap_or(0.1_f64));
    let feather = (threshold * 0.5_f64);
    let color = (effect.color).clone().unwrap_or(255.0_f64);
    let r = ((__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let g = ((__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let b = ((__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let a = ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = threshold;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = feather;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = r;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = g;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (4.0_f64) as usize;
        let __flight_value = b;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (5.0_f64) as usize;
        let __flight_value = a;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/edgeDetectMath.ts:35 (sha256:fb2be4626564eceed93e0d812916c009fbd03cf310f66f0c951b50fe6fcc5a35)
pub fn compute_outline_thickness_px(effect: &OutlineEffect) -> f64 {
    return (0.0_f64).max(((effect.thickness).clone().unwrap_or(1.0_f64)).round());
}

// Source: upstream/packages/effects/src/edgeDetectMath.ts:45 (sha256:1a87eaf73bedbb64b8ca5b60faa4172d6a93f079bf47d04b4b3696759c94de93)
pub fn compute_sketch_edge_params(effect: &SketchEffect, out: &mut Vec<f64>) -> () {
    let strength = (0.0_f64).max((1.0_f64).min((effect.strength).clone().unwrap_or(1.0_f64)));
    let threshold = (0.01_f64).max((1.0_f64).min((1.0_f64 - (strength * 0.95_f64))));
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = threshold;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = strength;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/effects/src/edgeDetectMath.ts:59 (sha256:accd5eada344bdfa86876ffe94f29d935ccdaa989b74b2af40d400299ad06a47)
pub fn get_sobel_kernel_coefficients(out: &mut Vec<f32>) -> () {
    out[0.0_f64 as usize] = (-1.0_f64) as f32;
    out[1.0_f64 as usize] = (0.0_f64) as f32;
    out[2.0_f64 as usize] = (1.0_f64) as f32;
    out[3.0_f64 as usize] = (-2.0_f64) as f32;
    out[4.0_f64 as usize] = (0.0_f64) as f32;
    out[5.0_f64 as usize] = (2.0_f64) as f32;
    out[6.0_f64 as usize] = (-1.0_f64) as f32;
    out[7.0_f64 as usize] = (0.0_f64) as f32;
    out[8.0_f64 as usize] = (1.0_f64) as f32;
    out[9.0_f64 as usize] = (-1.0_f64) as f32;
    out[10.0_f64 as usize] = (-2.0_f64) as f32;
    out[11.0_f64 as usize] = (-1.0_f64) as f32;
    out[12.0_f64 as usize] = (0.0_f64) as f32;
    out[13.0_f64 as usize] = (0.0_f64) as f32;
    out[14.0_f64 as usize] = (0.0_f64) as f32;
    out[15.0_f64 as usize] = (1.0_f64) as f32;
    out[16.0_f64 as usize] = (2.0_f64) as f32;
    out[17.0_f64 as usize] = (1.0_f64) as f32;
}
