// @generated from upstream/packages/bitmap/src/bitmapCompare.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BitmapMismatch;

// Source: upstream/packages/bitmap/src/bitmapCompare.ts:53 (sha256:c3547aceb9449c56113872d599c79e28078cb5dd0ebfb48d4976232d1821556b)
pub fn get_bitmap_mismatch(
    source: BitmapComparisonSource,
    other: BitmapComparisonSource,
    channel_tolerance: Option<f64>,
) -> BitmapMismatch {
    let channel_tolerance = channel_tolerance.unwrap_or(0.0_f64);
    if (source.width != other.width) || (source.height != other.height) {
        panic!(
            "{}",
            format!(
                "getBitmapMismatch: bitmap dimensions do not match ({}×{} vs {}×{})",
                source.width, source.height, other.width, other.height
            )
        );
    }
    let a = source.data;
    let b = other.data;
    let total_pixels = (source.width * source.height);
    let mut mismatched_pixels = 0.0_f64;
    let mut max_channel_delta = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < a.length) {
            let dr = (a[i as usize].clone() - b[i as usize].clone()).abs();
            let dg = (a[(i + 1.0_f64) as usize].clone() - b[(i + 1.0_f64) as usize].clone()).abs();
            let db = (a[(i + 2.0_f64) as usize].clone() - b[(i + 2.0_f64) as usize].clone()).abs();
            let da = (a[(i + 3.0_f64) as usize].clone() - b[(i + 3.0_f64) as usize].clone()).abs();
            let pixel_delta = (((dr).max(dg)).max(db)).max(da);
            if (pixel_delta > max_channel_delta) {
                max_channel_delta = pixel_delta;
            }
            if (pixel_delta > channel_tolerance) {
                {
                    mismatched_pixels += 1.0;
                    mismatched_pixels
                };
            }
            {
                i += 4.0_f64;
                i.clone()
            };
        }
    }
    return BitmapMismatch {
        __flight_identity: std::sync::Arc::new(()),
        mismatched_pixels: mismatched_pixels,
        total_pixels: total_pixels,
        fraction: if (total_pixels == 0.0_f64) {
            0.0_f64
        } else {
            (mismatched_pixels / total_pixels)
        },
        max_channel_delta: max_channel_delta,
    };
}
