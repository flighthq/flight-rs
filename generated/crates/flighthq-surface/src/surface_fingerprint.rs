// @generated from upstream/packages/surface/src/surfaceFingerprint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Surface, SurfaceFingerprint};

// Source: upstream/packages/surface/src/surfaceFingerprint.ts:14 (sha256:bf72ebe0d9ab44684608e4efa3281f6ce020e0bd30412ecdd71e94811d0153c5)
pub fn compare_surface_fingerprints(a: &SurfaceFingerprint, b: &SurfaceFingerprint) -> f64 {
    if (a.grid_size != b.grid_size) {
        panic!(
            "{}",
            format!(
                "compareSurfaceFingerprints: gridSize mismatch ({} vs {})",
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

// Source: upstream/packages/surface/src/surfaceFingerprint.ts:30 (sha256:2b1fc53062fabefb063813791b7043e06321badd3fa289763b0eb7107412e191)
pub fn create_surface_fingerprint(source: &Surface, grid_size: Option<f64>) -> SurfaceFingerprint {
    let grid_size = grid_size.unwrap_or(16.0_f64);
    if (grid_size < 1.0_f64) {
        panic!(
            "{}",
            format!(
                "createSurfaceFingerprint: gridSize must be >= 1 (got {})",
                grid_size
            )
        );
    }
    let mut cells = vec![0_u8; ((grid_size * grid_size) * 3.0_f64) as usize];
    let width = source.width;
    let height = source.height;
    if ((width == 0.0_f64) || (height == 0.0_f64)) {
        return SurfaceFingerprint {
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
                        while ((y < y1) && (y < height)) {
                            let mut i = (((y * width) + x0) * 4.0_f64);
                            {
                                let mut x = x0;
                                while ((x < x1) && (x < width)) {
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
    return SurfaceFingerprint {
        __flight_identity: std::sync::Arc::new(()),
        grid_size: grid_size,
        cells: (cells).clone(),
    };
}
