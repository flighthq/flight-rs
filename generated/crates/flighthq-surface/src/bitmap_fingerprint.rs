// @generated from upstream/packages/bitmap/src/bitmapFingerprint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Bitmap, BitmapFingerprint};

// Source: upstream/packages/bitmap/src/bitmapFingerprint.ts:23 (sha256:d302c76f297ce383d6df2b313ffb72c2186ee3251c9d5730a16e7496760779aa)
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

// Source: upstream/packages/bitmap/src/bitmapFingerprint.ts:39 (sha256:a9aa31a47c2d19af3acb9f6f2d9aa3496e258998da9b7104ff4b559276d29665)
pub fn create_bitmap_fingerprint(source: &Bitmap, grid_size: Option<f64>) -> BitmapFingerprint {
    let grid_size = grid_size.unwrap_or(16.0_f64);
    if (grid_size < 1.0_f64) {
        panic!(
            "{}",
            format!(
                "createBitmapFingerprint: gridSize must be >= 1 (got {})",
                grid_size
            )
        );
    }
    let mut cells: Vec<u8> = vec![0_u8; ((grid_size * grid_size) * 3.0_f64) as usize];
    let width = source.width;
    let height = source.height;
    if (width == 0.0_f64) || (height == 0.0_f64) {
        return BitmapFingerprint {
            __flight_identity: std::sync::Arc::new(()),
            grid_size: grid_size,
            cells: (cells).clone(),
        };
    }
    {
        let mut cy = 0.0_f64;
        while (cy < grid_size) {
            let mut y0 = ((cy * height) / grid_size).floor();
            let y1 = (y0 + 1.0_f64).max((((cy + 1.0_f64) * height) / grid_size).floor());
            {
                let mut cx = 0.0_f64;
                while (cx < grid_size) {
                    let mut x0 = ((cx * width) / grid_size).floor();
                    let x1 = (x0 + 1.0_f64).max((((cx + 1.0_f64) * width) / grid_size).floor());
                    let mut sum_r = 0.0_f64;
                    let mut sum_g = 0.0_f64;
                    let mut sum_b = 0.0_f64;
                    let mut count = 0.0_f64;
                    {
                        let mut y = y0;
                        while (y < y1) && (y < height) {
                            let mut i = (((y * width) + x0) * 4.0_f64);
                            {
                                let mut x = x0;
                                while (x < x1) && (x < width) {
                                    sum_r += (source.data[i as usize] as f64);
                                    sum_g += (source.data[(i + 1.0_f64) as usize] as f64);
                                    sum_b += (source.data[(i + 2.0_f64) as usize] as f64);
                                    {
                                        count += 1.0;
                                        count
                                    };
                                    i += 4.0_f64;
                                    {
                                        x += 1.0;
                                        x
                                    };
                                }
                            }
                            {
                                y += 1.0;
                                y
                            };
                        }
                    }
                    let c = (((cy * grid_size) + cx) * 3.0_f64);
                    cells[c as usize] = if (count == 0.0_f64) {
                        (0.0_f64) as u8
                    } else {
                        ((sum_r / count).round()) as u8
                    };
                    cells[(c + 1.0_f64) as usize] = if (count == 0.0_f64) {
                        (0.0_f64) as u8
                    } else {
                        ((sum_g / count).round()) as u8
                    };
                    cells[(c + 2.0_f64) as usize] = if (count == 0.0_f64) {
                        (0.0_f64) as u8
                    } else {
                        ((sum_b / count).round()) as u8
                    };
                    {
                        cx += 1.0;
                        cx
                    };
                }
            }
            {
                cy += 1.0;
                cy
            };
        }
    }
    return BitmapFingerprint {
        __flight_identity: std::sync::Arc::new(()),
        grid_size: grid_size,
        cells: (cells).clone(),
    };
}
