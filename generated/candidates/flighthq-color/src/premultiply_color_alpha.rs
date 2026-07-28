// @generated from upstream/packages/color/src/premultiplyColorAlpha.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

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

// Source: upstream/packages/color/src/premultiplyColorAlpha.ts:5 (sha256:f56cb2ebc506444c966189f5e758a2f05fedb93df401065e995fded35dd2ab70)
pub fn premultiply_color_alpha(color: f64) -> f64 {
    let a = ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    let r = ((__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        * a)
        .round();
    let g = ((__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        * a)
        .round();
    let b = ((__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        * a)
        .round();
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
        ) | __flight_js_to_i32((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64))
            as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/color/src/premultiplyColorAlpha.ts:16 (sha256:46b76154a97dc527a7fe0d1286b61ddb9e2b590d2a12da2048ed370257f0644d)
pub fn unpremultiply_color_alpha(color: f64) -> f64 {
    let a = ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    if (a == 0.0_f64) {
        return color;
    }
    let r = (255.0_f64).min(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / a)
            .round(),
    );
    let g = (255.0_f64).min(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / a)
            .round(),
    );
    let b = (255.0_f64).min(
        ((__flight_js_to_i32(
            (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64
            / a)
            .round(),
    );
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
        ) | __flight_js_to_i32((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64))
            as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}
