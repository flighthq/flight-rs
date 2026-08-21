// @generated from upstream/packages/effects/src/blendModeMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ADVANCED_BLEND_MODE as advanced_blend_mode_constant;

// Source: upstream/packages/effects/src/blendModeMath.ts:18 (sha256:42b8aaae1d777b0fc23f3f646319b0f80130e4c66a73125550e63443d437425f)
pub fn blend_non_separable_rgb(
    mode: AdvancedBlendMode,
    cb_r: f64,
    cb_g: f64,
    cb_b: f64,
    cs_r: f64,
    cs_g: f64,
    cs_b: f64,
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
) -> () {
    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    {
        let __switch_value = mode;
        let __flight_case = if __switch_value == advanced_blend_mode_constant.hue {
            0_usize
        } else if __switch_value == advanced_blend_mode_constant.saturation {
            1_usize
        } else if __switch_value == advanced_blend_mode_constant.color {
            2_usize
        } else if __switch_value == advanced_blend_mode_constant.luminosity {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                vec![r, g, b] =
                    set_blend_saturation(cs_r, cs_g, cs_b, blend_saturation(cb_r, cb_g, cb_b));
                vec![r, g, b] = set_blend_luminosity(r, g, b, blend_luminosity(cb_r, cb_g, cb_b));
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                vec![r, g, b] =
                    set_blend_saturation(cb_r, cb_g, cb_b, blend_saturation(cs_r, cs_g, cs_b));
                vec![r, g, b] = set_blend_luminosity(r, g, b, blend_luminosity(cb_r, cb_g, cb_b));
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                vec![r, g, b] =
                    set_blend_luminosity(cs_r, cs_g, cs_b, blend_luminosity(cb_r, cb_g, cb_b));
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                vec![r, g, b] =
                    set_blend_luminosity(cb_r, cb_g, cb_b, blend_luminosity(cs_r, cs_g, cs_b));
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                r = cs_r;
                g = cs_g;
                b = cs_b;
                break '__flight_switch;
            }
            unreachable!("exhaustive TypeScript switch completed without exiting");
        }
    }
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = r;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = g;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = b;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
}

// Source: upstream/packages/effects/src/blendModeMath.ts:64 (sha256:d0305567c82c7ea797bc19a049f1b861a6562f61b0fd2d68d6de34bf751a036e)
pub fn get_advanced_blend_rgb(
    mode: AdvancedBlendMode,
    cb_r: f64,
    cb_g: f64,
    cb_b: f64,
    cs_r: f64,
    cs_g: f64,
    cs_b: f64,
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
) -> () {
    if is_non_separable_blend_mode((mode).clone()) {
        blend_non_separable_rgb((mode).clone(), cb_r, cb_g, cb_b, cs_r, cs_g, cs_b, out);
        return;
    }
    let r = get_separable_blend_channel((mode).clone(), cb_r, cs_r);
    let g = get_separable_blend_channel((mode).clone(), cb_g, cs_g);
    let b = get_separable_blend_channel((mode).clone(), cb_b, cs_b);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = r;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = g;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = b;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
}

// Source: upstream/packages/effects/src/blendModeMath.ts:88 (sha256:d6bbf33a79bcb7d4cf7e79d9ae1618dc44fe19cef69dfc71af36f21c07dceb87)
pub fn get_separable_blend_channel(mode: AdvancedBlendMode, cb: f64, cs: f64) -> f64 {
    {
        let __switch_value = mode;
        let __flight_case = if __switch_value == advanced_blend_mode_constant.overlay {
            0_usize
        } else if __switch_value == advanced_blend_mode_constant.hard_light {
            1_usize
        } else if __switch_value == advanced_blend_mode_constant.soft_light {
            2_usize
        } else if __switch_value == advanced_blend_mode_constant.darken {
            3_usize
        } else if __switch_value == advanced_blend_mode_constant.difference {
            4_usize
        } else if __switch_value == advanced_blend_mode_constant.exclusion {
            5_usize
        } else if __switch_value == advanced_blend_mode_constant.color_dodge {
            6_usize
        } else if __switch_value == advanced_blend_mode_constant.lighten {
            7_usize
        } else if __switch_value == advanced_blend_mode_constant.color_burn {
            8_usize
        } else {
            9_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return if (cb <= 0.5_f64) {
                    ((2.0_f64 * cb) * cs)
                } else {
                    (1.0_f64 - ((2.0_f64 * (1.0_f64 - cb)) * (1.0_f64 - cs)))
                };
            }
            if __flight_case <= 1_usize {
                return if (cs <= 0.5_f64) {
                    ((2.0_f64 * cb) * cs)
                } else {
                    (1.0_f64 - ((2.0_f64 * (1.0_f64 - cb)) * (1.0_f64 - cs)))
                };
            }
            if __flight_case <= 2_usize {
                {
                    let d = if (cb <= 0.25_f64) {
                        (((((16.0_f64 * cb) - 12.0_f64) * cb) + 4.0_f64) * cb)
                    } else {
                        (cb).sqrt()
                    };
                    return if (cs <= 0.5_f64) {
                        (cb - (((1.0_f64 - (2.0_f64 * cs)) * cb) * (1.0_f64 - cb)))
                    } else {
                        (cb + (((2.0_f64 * cs) - 1.0_f64) * (d - cb)))
                    };
                }
            }
            if __flight_case <= 3_usize {
                return (cb).min(cs);
            }
            if __flight_case <= 4_usize {
                return (cb - cs).abs();
            }
            if __flight_case <= 5_usize {
                return ((cb + cs) - ((2.0_f64 * cb) * cs));
            }
            if __flight_case <= 6_usize {
                if (cb <= 0.0_f64) {
                    return 0.0_f64;
                }
                if (cs >= 1.0_f64) {
                    return 1.0_f64;
                }
                return (1.0_f64).min((cb / (1.0_f64 - cs)));
            }
            if __flight_case <= 7_usize {
                return (cb).max(cs);
            }
            if __flight_case <= 8_usize {
                if (cb >= 1.0_f64) {
                    return 1.0_f64;
                }
                if (cs <= 0.0_f64) {
                    return 0.0_f64;
                }
                return (1.0_f64 - (1.0_f64).min(((1.0_f64 - cb) / cs)));
            }
            if __flight_case <= 9_usize {
                return cs;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}

// Source: upstream/packages/effects/src/blendModeMath.ts:124 (sha256:4be93976d04aad2e1d12f1d98bd449ee2b9e2bf7f15a90d4f54b620a4a59d09b)
pub fn is_non_separable_blend_mode(mode: AdvancedBlendMode) -> bool {
    return (((mode == advanced_blend_mode_constant.hue)
        || (mode == advanced_blend_mode_constant.saturation))
        || (mode == advanced_blend_mode_constant.color))
        || (mode == advanced_blend_mode_constant.luminosity);
}

// Source: upstream/packages/effects/src/blendModeMath.ts:134 (sha256:443d6cadca0ac7d73a27748b698259ead53ed14183143ed9a9a2c58feb93b5c8)
fn blend_luminosity(r: f64, g: f64, b: f64) -> f64 {
    return (((0.3_f64 * r) + (0.59_f64 * g)) + (0.11_f64 * b));
}

// Source: upstream/packages/effects/src/blendModeMath.ts:139 (sha256:37fdd7e60ca73ddd2e4ad9a6f9ac21f6145006d743ab0912a81f691c4ccfa2d7)
fn blend_saturation(r: f64, g: f64, b: f64) -> f64 {
    return (((r).max(g)).max(b) - ((r).min(g)).min(b));
}

// Source: upstream/packages/effects/src/blendModeMath.ts:145 (sha256:013d17202e913c6bbb3d4a0aad3667cafa920f25f2094f208f07bace7e98a751)
fn clip_blend_color(r: f64, g: f64, b: f64) -> Vec<f64> {
    let l = blend_luminosity(r, g, b);
    let min = ((r).min(g)).min(b);
    let max = ((r).max(g)).max(b);
    let mut cr = r;
    let mut cg = g;
    let mut cb = b;
    if (min < 0.0_f64) {
        let denom = (l - min);
        cr = (l + (((cr - l) * l) / denom));
        cg = (l + (((cg - l) * l) / denom));
        cb = (l + (((cb - l) * l) / denom));
    }
    if (max > 1.0_f64) {
        let denom = (max - l);
        cr = (l + (((cr - l) * (1.0_f64 - l)) / denom));
        cg = (l + (((cg - l) * (1.0_f64 - l)) / denom));
        cb = (l + (((cb - l) * (1.0_f64 - l)) / denom));
    }
    return vec![cr, cg, cb];
}

// Source: upstream/packages/effects/src/blendModeMath.ts:168 (sha256:ede08854234aeb529a3ea3c7b215dfe13feab0927620805f9fa11aa3f35c3c78)
fn set_blend_luminosity(r: f64, g: f64, b: f64, target: f64) -> Vec<f64> {
    let d = (target - blend_luminosity(r, g, b));
    return clip_blend_color((r + d), (g + d), (b + d));
}

// Source: upstream/packages/effects/src/blendModeMath.ts:175 (sha256:f71a3267ff673fd73226b212b678f5ca387764b2a83759af40f2dfbe42e607e1)
fn set_blend_saturation(r: f64, g: f64, b: f64, target: f64) -> Vec<f64> {
    let mut out: Vec<f64> = vec![r, g, b];
    let mut i_min = 0.0_f64;
    let mut i_max = 0.0_f64;
    {
        let mut i = 1.0_f64;
        while (i < 3.0_f64) {
            if (out[i as usize].clone() < out[i_min as usize].clone()) {
                i_min = i;
            }
            if (out[i as usize].clone() > out[i_max as usize].clone()) {
                i_max = i;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if (i_min == i_max) {
        i_max = ((i_min + 1.0_f64) % 3.0_f64);
    }
    let i_mid = ((3.0_f64 - i_min) - i_max);
    if (out[i_max as usize].clone() > out[i_min as usize].clone()) {
        {
            let __flight_index = (i_mid) as usize;
            let __flight_value = (((out[i_mid as usize].clone() - out[i_min as usize].clone())
                * target)
                / (out[i_max as usize].clone() - out[i_min as usize].clone()));
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (i_max) as usize;
            let __flight_value = target;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
    } else {
        {
            let __flight_index = (i_mid) as usize;
            let __flight_value = 0.0_f64;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (i_max) as usize;
            let __flight_value = 0.0_f64;
            if __flight_index == out.len() {
                out.push(__flight_value);
            } else {
                out[__flight_index] = __flight_value;
            }
        };
    }
    {
        let __flight_index = (i_min) as usize;
        let __flight_value = 0.0_f64;
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    return out;
}
