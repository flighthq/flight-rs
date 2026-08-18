// @generated from upstream/packages/bitmap/src/bitmapCompare.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Bitmap, BitmapMismatch};

// Source: upstream/packages/bitmap/src/bitmapCompare.ts:46 (sha256:a10b53c95ced94e765c594cf08e313c2dafa7b358b150eebb4e545247da40e68)
pub fn get_bitmap_mismatch(
    source: &Bitmap,
    other: &Bitmap,
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
    let total_pixels = (source.width * source.height);
    let mut mismatched_pixels = 0.0_f64;
    let mut max_channel_delta = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (source.data.len() as f64)) {
            let dr = ((source.data[i as usize] as f64) - (other.data[i as usize] as f64)).abs();
            let dg = ((source.data[(i + 1.0_f64) as usize] as f64)
                - (other.data[(i + 1.0_f64) as usize] as f64))
                .abs();
            let db = ((source.data[(i + 2.0_f64) as usize] as f64)
                - (other.data[(i + 2.0_f64) as usize] as f64))
                .abs();
            let da = ((source.data[(i + 3.0_f64) as usize] as f64)
                - (other.data[(i + 3.0_f64) as usize] as f64))
                .abs();
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
                i
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
