// @generated from upstream/packages/surface/src/surfacePixel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Surface;

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

// Source: upstream/packages/surface/src/surfacePixel.ts:7 (sha256:52a7b91c9b9bae622bacab58f5d68535e747ad2d9a1712642c08960cef9b47e4)
const LUMA_R: f64 = 0.2126_f64;

// Source: upstream/packages/surface/src/surfacePixel.ts:8 (sha256:ecab8018c5b47a1522bcf6e606c7704e67cf492967f4eb8d2c643f7ef51b0ad5)
const LUMA_G: f64 = 0.7152_f64;

// Source: upstream/packages/surface/src/surfacePixel.ts:9 (sha256:567a6664ea73589e609fe6510e658b901f093a125ba87f0994a4d1ac369438d1)
const LUMA_B: f64 = 0.0722_f64;

// Source: upstream/packages/surface/src/surfacePixel.ts:11 (sha256:3734116bf416a0df6ac2c0958cfb0384b979bfdadf213b240cc6bb7725095019)
pub fn get_surface_pixel(source: &Surface, x: f64, y: f64) -> f64 {
    let i = (((y * source.width) + x) * 4.0_f64);
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32((source.data[i as usize] as f64))
                        .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31))
                        as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32((source.data[(i + 1.0_f64) as usize] as f64))
                        .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31))
                        as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32((source.data[(i + 2.0_f64) as usize] as f64))
                    .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32((source.data[(i + 3.0_f64) as usize] as f64))) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/surface/src/surfacePixel.ts:20 (sha256:83730f7753200271f70c237e2deab57b299157006d3fae24e2ce92651f5106bb)
pub fn get_surface_pixel_luminance(source: &Surface, x: f64, y: f64) -> f64 {
    let i = (((y * source.width) + x) * 4.0_f64);
    return ((((source.data[i as usize] as f64) * LUMA_R)
        + ((source.data[(i + 1.0_f64) as usize] as f64) * LUMA_G))
        + ((source.data[(i + 2.0_f64) as usize] as f64) * LUMA_B))
        .round();
}

// Source: upstream/packages/surface/src/surfacePixel.ts:25 (sha256:e363a28e587861439d4d144b82365133f2c390e8c9eead3e17d80e8608905a27)
pub fn get_surface_pixel_rgb(source: &Surface, x: f64, y: f64) -> f64 {
    let i = (((y * source.width) + x) * 4.0_f64);
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                __flight_js_to_i32((source.data[i as usize] as f64))
                    .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32((source.data[(i + 1.0_f64) as usize] as f64))
                    .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32((source.data[(i + 2.0_f64) as usize] as f64))) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/surface/src/surfacePixel.ts:30 (sha256:12cef1a1ee5dd62ad91a34414f0a39ea5e426d6136a8655e3ecc9eb6bda91d1f)
pub fn set_surface_pixel(out: &mut Surface, x: f64, y: f64, color: f64) -> () {
    let i = (((y * out.width) + x) * 4.0_f64);
    out.data[i as usize] = ((__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    out.data[(i + 1.0_f64) as usize] = ((__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    out.data[(i + 2.0_f64) as usize] = ((__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    out.data[(i + 3.0_f64) as usize] =
        ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    {
        out.version = (__flight_js_to_u32((out.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}

// Source: upstream/packages/surface/src/surfacePixel.ts:39 (sha256:96bca1ddeafa39b413bb2c4e903478860e8ef82fdf22326d4535633d2cb31166)
pub fn set_surface_pixel_rgb(out: &mut Surface, x: f64, y: f64, color: f64) -> () {
    let i = (((y * out.width) + x) * 4.0_f64);
    out.data[i as usize] = ((__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    out.data[(i + 1.0_f64) as usize] = ((__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    out.data[(i + 2.0_f64) as usize] =
        ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    {
        out.version = (__flight_js_to_u32((out.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}
