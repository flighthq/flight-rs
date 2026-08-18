// @generated from upstream/packages/bitmap/src/bitmapPixel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_bitmap;
use flighthq_types::Bitmap;

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

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:7 (sha256:52a7b91c9b9bae622bacab58f5d68535e747ad2d9a1712642c08960cef9b47e4)
const LUMA_R: f64 = 0.2126_f64;

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:8 (sha256:ecab8018c5b47a1522bcf6e606c7704e67cf492967f4eb8d2c643f7ef51b0ad5)
const LUMA_G: f64 = 0.7152_f64;

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:9 (sha256:567a6664ea73589e609fe6510e658b901f093a125ba87f0994a4d1ac369438d1)
const LUMA_B: f64 = 0.0722_f64;

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:11 (sha256:84775332d9983f43edb19326a13af9254c0dc12639d136e5f7f44dde064e4168)
pub fn get_bitmap_pixel(source: &Bitmap, x: f64, y: f64) -> f64 {
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

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:20 (sha256:fe9405a31cd120a54152e06f3d71f0615b9737c5353ad2b714a7c91314ed2971)
pub fn get_bitmap_pixel_luminance(source: &Bitmap, x: f64, y: f64) -> f64 {
    let i = (((y * source.width) + x) * 4.0_f64);
    return ((((source.data[i as usize] as f64) * LUMA_R)
        + ((source.data[(i + 1.0_f64) as usize] as f64) * LUMA_G))
        + ((source.data[(i + 2.0_f64) as usize] as f64) * LUMA_B))
        .round();
}

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:25 (sha256:7f915cb3df5494c9b42cac463e930ae4f18d0002168e6e89548359668d1ca51d)
pub fn get_bitmap_pixel_rgb(source: &Bitmap, x: f64, y: f64) -> f64 {
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

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:30 (sha256:dab15d236a14bda4a451111b78fd527b455334a94e5106df66abd59b0e124f2a)
pub fn set_bitmap_pixel(out: &mut Bitmap, x: f64, y: f64, color: f64) -> () {
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
    invalidate_bitmap(out);
}

// Source: upstream/packages/bitmap/src/bitmapPixel.ts:39 (sha256:a0dc1a212afa0ce1129e6454fe7ad03c366a5d34297b0c691d0e28261daf8787)
pub fn set_bitmap_pixel_rgb(out: &mut Bitmap, x: f64, y: f64, color: f64) -> () {
    let i = (((y * out.width) + x) * 4.0_f64);
    out.data[i as usize] = ((__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    out.data[(i + 1.0_f64) as usize] = ((__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    out.data[(i + 2.0_f64) as usize] =
        ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64) as u8;
    invalidate_bitmap(out);
}
