// @generated from upstream/packages/bitmap/src/bitmapFingerprint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BitmapFingerprint;

// Source: upstream/packages/bitmap/src/bitmapFingerprint.ts:24 (sha256:d302c76f297ce383d6df2b313ffb72c2186ee3251c9d5730a16e7496760779aa)
pub fn compare_bitmap_fingerprints(a: &BitmapFingerprint, b: &BitmapFingerprint) -> f64 {
    if (a.grid_size != b.grid_size) {
        panic!(
            "{}",
            format!(
                "compareBitmapFingerprints: gridSize mismatch ({} vs {})",
                a.grid_size, b.grid_size
            )
        );
    }
    if ((a.cells.len() as f64) == 0.0_f64) {
        return 0.0_f64;
    }
    let mut sum = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (a.cells.len() as f64)) {
            sum += ((a.cells[i as usize] as f64) - (b.cells[i as usize] as f64)).abs();
            {
                i += 1.0;
                i
            };
        }
    }
    return (sum / (a.cells.len() as f64));
}
