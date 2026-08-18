// @generated from upstream/packages/bitmap/src/bitmapHistogram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BitmapHistogram, BitmapRegion};

// Source: upstream/packages/bitmap/src/bitmapHistogram.ts:37 (sha256:2791606e51dbd789d90451c417442f32b3aa236cc13b1bb8932c9040df0eeed6)
pub fn get_bitmap_histogram(source: &BitmapRegion) -> BitmapHistogram {
    let mut red = vec![0.0_f64; (256.0_f64) as usize];
    let mut green = vec![0.0_f64; (256.0_f64) as usize];
    let mut blue = vec![0.0_f64; (256.0_f64) as usize];
    let mut alpha = vec![0.0_f64; (256.0_f64) as usize];
    let bitmap_width = source.bitmap.width;
    {
        let mut py = 0.0_f64;
        while (py < source.height) {
            let y = (source.y + py);
            if (y < 0.0_f64) || (y >= source.bitmap.height) {
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
                    if (x < 0.0_f64) || (x >= bitmap_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = (((y * bitmap_width) + x) * 4.0_f64);
                    {
                        red[(source.bitmap.data[i as usize] as f64) as usize] += 1.0;
                        red[(source.bitmap.data[i as usize] as f64) as usize]
                    };
                    {
                        green[(source.bitmap.data[(i + 1.0_f64) as usize] as f64) as usize] += 1.0;
                        green[(source.bitmap.data[(i + 1.0_f64) as usize] as f64) as usize]
                    };
                    {
                        blue[(source.bitmap.data[(i + 2.0_f64) as usize] as f64) as usize] += 1.0;
                        blue[(source.bitmap.data[(i + 2.0_f64) as usize] as f64) as usize]
                    };
                    {
                        alpha[(source.bitmap.data[(i + 3.0_f64) as usize] as f64) as usize] += 1.0;
                        alpha[(source.bitmap.data[(i + 3.0_f64) as usize] as f64) as usize]
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
    return BitmapHistogram {
        __flight_identity: std::sync::Arc::new(()),
        alpha: (alpha).clone(),
        blue: (blue).clone(),
        green: (green).clone(),
        red: (red).clone(),
    };
}
