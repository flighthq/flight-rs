// @generated from upstream/packages/surface/src/surfaceHistogram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{SurfaceHistogram, SurfaceRegion};

// Source: upstream/packages/surface/src/surfaceHistogram.ts:37 (sha256:d7b9463d819bd31ad6670d19c2db740ccfb4e5620ca4a7ffe8d0b1af677524ca)
pub fn get_surface_histogram(source: &SurfaceRegion) -> SurfaceHistogram {
    let mut red = vec![0.0_f64; (256.0_f64) as usize];
    let mut green = vec![0.0_f64; (256.0_f64) as usize];
    let mut blue = vec![0.0_f64; (256.0_f64) as usize];
    let mut alpha = vec![0.0_f64; (256.0_f64) as usize];
    let surface_width = source.surface.width;
    {
        let mut py = 0.0_f64;
        while (py < source.height) {
            let y = (source.y + py);
            if ((y < 0.0_f64) || (y >= source.surface.height)) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < source.width) {
                    let x = (source.x + px);
                    if ((x < 0.0_f64) || (x >= surface_width)) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = (((y * surface_width) + x) * 4.0_f64);
                    {
                        red[(source.surface.data[i as usize] as f64) as usize] += 1.0;
                        red[(source.surface.data[i as usize] as f64) as usize]
                    };
                    {
                        green[(source.surface.data[(i + 1.0_f64) as usize] as f64) as usize] += 1.0;
                        green[(source.surface.data[(i + 1.0_f64) as usize] as f64) as usize]
                    };
                    {
                        blue[(source.surface.data[(i + 2.0_f64) as usize] as f64) as usize] += 1.0;
                        blue[(source.surface.data[(i + 2.0_f64) as usize] as f64) as usize]
                    };
                    {
                        alpha[(source.surface.data[(i + 3.0_f64) as usize] as f64) as usize] += 1.0;
                        alpha[(source.surface.data[(i + 3.0_f64) as usize] as f64) as usize]
                    };
                    {
                        px += 1.0;
                        px
                    };
                }
            }
            {
                py += 1.0;
                py
            };
        }
    }
    return SurfaceHistogram {
        __flight_identity: std::sync::Arc::new(()),
        alpha: (alpha).clone(),
        blue: (blue).clone(),
        green: (green).clone(),
        red: (red).clone(),
    };
}
