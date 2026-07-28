// @generated from upstream/packages/color/src/packColor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{linear_channel_to_srgb, srgb_channel_to_linear};
pub use flighthq_types::LinearColor;

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

// Source: upstream/packages/color/src/packColor.ts:10 (sha256:78dae939b6157197b3aaca2c58a4fa07b402333aaeb9e7bec0c519b9b11827b1)
pub fn compute_rgb_hex_string(color: f64) -> String {
    return format!("#{}", (((__flight_js_to_i32(color) & __flight_js_to_i32(16777215.0_f64)) as f64.to_string)(16.0_f64).pad_start)(6.0_f64, "0"));
}

// Source: upstream/packages/color/src/packColor.ts:15 (sha256:996de9a4f58776bbe434b22983c12c761ee03d1da27ea78cf55ee228a9cc8b31)
pub fn create_linear_color() -> LinearColor {
    return vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64];
}

// Source: upstream/packages/color/src/packColor.ts:22 (sha256:42275e9921000979638ff98fee5f158bdd9ec6a2d3e1dfe94baae19ca34057cb)
pub fn get_color_alpha(color: f64) -> f64 {
    return ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
}

// Source: upstream/packages/color/src/packColor.ts:29 (sha256:17da9d5f9c8f874513714dffe1f81c2b1372460cc9a70d94b1e733e8fa495f7e)
pub fn get_color_rgb(color: f64) -> f64 {
    return (__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(16777215.0_f64)) as f64;
}

// Source: upstream/packages/color/src/packColor.ts:36 (sha256:e55c9396bccb160042def11ed4f64d2ecf552866768bda556cb68e9819657844)
pub fn pack_color(r: f64, g: f64, b: f64, a: f64) -> f64 {
    let ri = ((1.0_f64).min((0.0_f64).max(r)) * 255.0_f64).round();
    let gi = ((1.0_f64).min((0.0_f64).max(g)) * 255.0_f64).round();
    let bi = ((1.0_f64).min((0.0_f64).max(b)) * 255.0_f64).round();
    let ai = ((1.0_f64).min((0.0_f64).max(a)) * 255.0_f64).round();
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32(ri).wrapping_shl((__flight_js_to_u32(24.0_f64) & 31)) as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32(gi).wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32(bi).wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(ai)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/color/src/packColor.ts:47 (sha256:c57d8e63028e7b7fb0f5488d4d77e63815e160cf85a9588b2b93838dc456d115)
pub fn pack_linear_to_color(color: &LinearColor) -> f64 {
    let r = ((1.0_f64).min((0.0_f64).max(linear_channel_to_srgb(color[0.0_f64 as usize].clone())))
        * 255.0_f64)
        .round();
    let g = ((1.0_f64).min((0.0_f64).max(linear_channel_to_srgb(color[1.0_f64 as usize].clone())))
        * 255.0_f64)
        .round();
    let b = ((1.0_f64).min((0.0_f64).max(linear_channel_to_srgb(color[2.0_f64 as usize].clone())))
        * 255.0_f64)
        .round();
    let a = ((1.0_f64).min((0.0_f64).max(color[3.0_f64 as usize].clone())) * 255.0_f64).round();
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32(r).wrapping_shl((__flight_js_to_u32(24.0_f64) & 31)) as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32(g).wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32(b).wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(a)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/color/src/packColor.ts:61 (sha256:2e3ae112eeafb99c7d1a8121f889de3b44ba9a6d87f10487f4f12d0ba887dae8)
pub fn pack_opaque_color(rgb: f64) -> f64 {
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            __flight_js_to_i32(
                (__flight_js_to_i32(rgb) & __flight_js_to_i32(16777215.0_f64)) as f64,
            )
            .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) | __flight_js_to_i32(255.0_f64)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/color/src/packColor.ts:68 (sha256:eb83949e804b910348530e49a9dca096f44b4b02c132503e42de719589bde98d)
pub fn set_color_alpha(color: f64, alpha: f64) -> f64 {
    let a = ((1.0_f64).min((0.0_f64).max(alpha)) * 255.0_f64).round();
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(color) & __flight_js_to_i32(4294967040.0_f64)) as f64,
        ) | __flight_js_to_i32(a)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/color/src/packColor.ts:75 (sha256:55b2a28a4649a1ae79b22ef574d4db34df11fa5cc4343f37122521b1466a71af)
pub fn unpack_color_rgba(out: &mut Vec<f64>, color: f64) -> () {
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value =
            ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/color/src/packColor.ts:88 (sha256:f468de4428cc0a9ebce537c8ead03ff9d40e7c396f384f6e08a8bb386e9b21b8)
pub fn unpack_color_to_linear(out: &mut LinearColor, color: f64) -> LinearColor {
    out[0.0_f64 as usize] = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    out[1.0_f64 as usize] = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    out[2.0_f64 as usize] = srgb_channel_to_linear(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / 255.0_f64),
    );
    out[3.0_f64 as usize] =
        ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    return out.clone();
}
