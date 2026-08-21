// @generated from upstream/packages/color/src/hsvColor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::HsvColor;

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

// Source: upstream/packages/color/src/hsvColor.ts:4 (sha256:31b255e04f31fe7cdf5a7a30cfe887978dd0ec43f6645d87db002e09d612e84d)
pub fn allocate_hsv_color() -> HsvColor {
    return vec![0.0_f64, 0.0_f64, 0.0_f64];
}

// Source: upstream/packages/color/src/hsvColor.ts:10 (sha256:fe38a13db423ff703451b988227e0b65f38d7c6973bedb0e86d456fc031c2f57)
pub fn hsv_to_rgb(out: &mut Vec<f64>, h: f64, s: f64, v: f64) -> () {
    if (s == 0.0_f64) {
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = v;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (1.0_f64) as usize;
            let __flight_value = v;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (2.0_f64) as usize;
            let __flight_value = v;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        return;
    }
    let hn = (((h % 360.0_f64) + 360.0_f64) % 360.0_f64);
    let i = ((hn / 60.0_f64).floor() % 6.0_f64);
    let f = ((hn / 60.0_f64) - (hn / 60.0_f64).floor());
    let p = (v * (1.0_f64 - s));
    let q = (v * (1.0_f64 - (f * s)));
    let t = (v * (1.0_f64 - ((1.0_f64 - f) * s)));
    {
        let __switch_value = i;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else if __switch_value == 3.0_f64 {
            3_usize
        } else if __switch_value == 4.0_f64 {
            4_usize
        } else {
            5_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = v;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = t;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = p;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = q;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = v;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = p;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = p;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = v;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = t;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = p;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = q;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = v;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = t;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = p;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = v;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                break '__flight_switch;
            }
            if __flight_case <= 5_usize {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = v;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = p;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = q;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                break '__flight_switch;
            }
            unreachable!("exhaustive TypeScript switch completed without exiting");
        }
    }
}

// Source: upstream/packages/color/src/hsvColor.ts:60 (sha256:7b07e3570c335a91069bf29fe3a2b40b37ae6c72a27944bcd859da4a92236c83)
pub fn rgb_to_hsv(out: &mut HsvColor, color: f64) -> HsvColor {
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
    let d = (max - min);
    let v = max;
    let s = if (max == 0.0_f64) { 0.0_f64 } else { (d / max) };
    let mut h: f64;
    if (d == 0.0_f64) {
        h = 0.0_f64;
    } else {
        if (max == r) {
            h = ((((g - b) / d) + if (g < b) { 6.0_f64 } else { 0.0_f64 }) / 6.0_f64);
        } else {
            if (max == g) {
                h = ((((b - r) / d) + 2.0_f64) / 6.0_f64);
            } else {
                h = ((((r - g) / d) + 4.0_f64) / 6.0_f64);
            }
        }
    }
    out[0.0_f64 as usize] = (h * 360.0_f64);
    out[1.0_f64 as usize] = s;
    out[2.0_f64 as usize] = v;
    return out.clone();
}
