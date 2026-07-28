// @generated from upstream/packages/surface/src/surfaceCompare.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Surface, SurfaceMismatch};

// Source: upstream/packages/surface/src/surfaceCompare.ts:46 (sha256:7e6d2d3a4ca46dbaea3cda7a4d5ba321f5ce54b8bf589d5ad367fba167704591)
pub fn get_surface_mismatch(
    source: &Surface,
    other: &Surface,
    channel_tolerance: Option<f64>,
) -> SurfaceMismatch {
    let channel_tolerance = channel_tolerance.unwrap_or(0.0_f64);
    if ((source.width != other.width) || (source.height != other.height)) {
        panic!(
            "{}",
            format!(
                "getSurfaceMismatch: surface dimensions do not match ({}×{} vs {}×{})",
                source.width, source.height, other.width, other.height
            )
        );
    }
    let a = &source.data;
    let b = &other.data;
    let total_pixels = (source.width * source.height);
    let mut mismatched_pixels = 0.0_f64;
    let mut max_channel_delta = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (a.len() as f64)) {
            let dr = ((a[i as usize] as f64) - (b[i as usize] as f64)).abs();
            let dg =
                ((a[(i + 1.0_f64) as usize] as f64) - (b[(i + 1.0_f64) as usize] as f64)).abs();
            let db =
                ((a[(i + 2.0_f64) as usize] as f64) - (b[(i + 2.0_f64) as usize] as f64)).abs();
            let da =
                ((a[(i + 3.0_f64) as usize] as f64) - (b[(i + 3.0_f64) as usize] as f64)).abs();
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
    return SurfaceMismatch {
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
