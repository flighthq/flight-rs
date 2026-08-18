// @generated from upstream/packages/color/src/hslColor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::HslColor;

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

// Source: upstream/packages/color/src/hslColor.ts:4 (sha256:9415f7c0ad7e87664bb5c3a282417f76dc20ab2a9b5b9b4b4d29167ceb99c7b9)
pub fn allocate_hsl_color() -> HslColor {
    return vec![0.0_f64, 0.0_f64, 0.0_f64];
}

// Source: upstream/packages/color/src/hslColor.ts:11 (sha256:8da92d99442bf972bd6bd00ae37bd8a063cb014e19155590ad63c831996feac5)
pub fn hsl_to_rgb(out: &mut Vec<f64>, h: f64, s: f64, l: f64) -> () {
    if (s == 0.0_f64) {
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = l;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (1.0_f64) as usize;
            let __flight_value = l;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (2.0_f64) as usize;
            let __flight_value = l;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        return;
    }
    let q = if (l < 0.5_f64) {
        (l * (1.0_f64 + s))
    } else {
        ((l + s) - (l * s))
    };
    let p = ((2.0_f64 * l) - q);
    let hn = (h / 360.0_f64);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = hue_to_rgb_channel(p, q, (hn + (1.0_f64 / 3.0_f64)));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = hue_to_rgb_channel(p, q, hn);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = hue_to_rgb_channel(p, q, (hn - (1.0_f64 / 3.0_f64)));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/color/src/hslColor.ts:30 (sha256:25c545bfca80b1a63eeebf7bf49a4800ace1a2068414720527bd4f9642d1da85)
pub fn rgb_to_hsl(out: &mut HslColor, color: f64) -> HslColor {
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
    let max = ((r).max(g)).max(b);
    let min = ((r).min(g)).min(b);
    let l = ((max + min) / 2.0_f64);
    if (max == min) {
        out[0.0_f64 as usize] = 0.0_f64;
        out[1.0_f64 as usize] = 0.0_f64;
        out[2.0_f64 as usize] = l;
        return out.clone();
    }
    let d = (max - min);
    let s = if (l > 0.5_f64) {
        (d / ((2.0_f64 - max) - min))
    } else {
        (d / (max + min))
    };
    let mut h: f64;
    if (max == r) {
        h = ((((g - b) / d) + if (g < b) { 6.0_f64 } else { 0.0_f64 }) / 6.0_f64);
    } else {
        if (max == g) {
            h = ((((b - r) / d) + 2.0_f64) / 6.0_f64);
        } else {
            h = ((((r - g) / d) + 4.0_f64) / 6.0_f64);
        }
    }
    out[0.0_f64 as usize] = (h * 360.0_f64);
    out[1.0_f64 as usize] = s;
    out[2.0_f64 as usize] = l;
    return out.clone();
}

// Source: upstream/packages/color/src/hslColor.ts:60 (sha256:71fddc9783b633cf28092d766bfed16fc19de8038904f30e64f8b64f9959b250)
fn hue_to_rgb_channel(p: f64, q: f64, t: f64) -> f64 {
    let tn = (((t % 1.0_f64) + 1.0_f64) % 1.0_f64);
    if (tn < (1.0_f64 / 6.0_f64)) {
        return (p + (((q - p) * 6.0_f64) * tn));
    }
    if (tn < (1.0_f64 / 2.0_f64)) {
        return q;
    }
    if (tn < (2.0_f64 / 3.0_f64)) {
        return (p + (((q - p) * ((2.0_f64 / 3.0_f64) - tn)) * 6.0_f64));
    }
    return p;
}
