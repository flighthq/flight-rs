// @generated from upstream/packages/surface/src/surfaceCoverage.ts; do not edit.
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

// Source: upstream/packages/surface/src/surfaceCoverage.ts:11 (sha256:e5904e18dd0c714ba9f2af54123a6a79927f2ce6b611cb385aa7861e02fbb56c)
pub fn get_surface_coverage(
    source: &Surface,
    background_color: f64,
    channel_tolerance: Option<f64>,
) -> f64 {
    let channel_tolerance = channel_tolerance.unwrap_or(0.0_f64);
    let br = (__flight_js_to_i32(
        (__flight_js_to_u32(background_color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let bg = (__flight_js_to_i32(
        (__flight_js_to_i32(background_color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let bb = (__flight_js_to_i32(
        (__flight_js_to_i32(background_color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let ba = (__flight_js_to_i32(background_color) & __flight_js_to_i32(255.0_f64)) as f64;
    let total_pixels = (source.width * source.height);
    if (total_pixels == 0.0_f64) {
        return 0.0_f64;
    }
    let mut covered = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (source.data.len() as f64)) {
            if ((((((source.data[i as usize] as f64) - br).abs() > channel_tolerance)
                || (((source.data[(i + 1.0_f64) as usize] as f64) - bg).abs()
                    > channel_tolerance))
                || (((source.data[(i + 2.0_f64) as usize] as f64) - bb).abs() > channel_tolerance))
                || (((source.data[(i + 3.0_f64) as usize] as f64) - ba).abs() > channel_tolerance))
            {
                {
                    covered += 1.0;
                    covered
                };
            }
            {
                i += 4.0_f64;
                i
            };
        }
    }
    return (covered / total_pixels);
}
