// @generated from upstream/packages/surface/src/surfacePixelate.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SurfaceRegion;

// Source: upstream/packages/surface/src/surfacePixelate.ts:13 (sha256:5ba66e34f0f53ae7c642c49813e65a9f0615221f86e280bc30db3a59551256c9)
pub fn pixelate_surface(out: &mut Vec<u8>, source: &SurfaceRegion, block_size: f64) -> () {
    let block = (1.0_f64).max((block_size).round());
    let w = source.width;
    let h = source.height;
    let surface_width = source.surface.width;
    let surface_height = source.surface.height;
    {
        let mut by = 0.0_f64;
        while (by < h) {
            let y_end = (by + block).min(h);
            {
                let mut bx = 0.0_f64;
                while (bx < w) {
                    let x_end = (bx + block).min(w);
                    let mut r = 0.0_f64;
                    let mut g = 0.0_f64;
                    let mut b = 0.0_f64;
                    let mut a = 0.0_f64;
                    let mut count = 0.0_f64;
                    {
                        let mut py = by;
                        while (py < y_end) {
                            let sy = (source.y + py);
                            if (sy < 0.0_f64) || (sy >= surface_height) {
                                {
                                    py += 1.0;
                                    py
                                };
                                continue;
                            }
                            {
                                let mut px = bx;
                                while (px < x_end) {
                                    let sx = (source.x + px);
                                    if (sx < 0.0_f64) || (sx >= surface_width) {
                                        {
                                            px += 1.0;
                                            px
                                        };
                                        continue;
                                    }
                                    let si = (((sy * surface_width) + sx) * 4.0_f64);
                                    r += (source.surface.data[si as usize] as f64);
                                    g += (source.surface.data[(si + 1.0_f64) as usize] as f64);
                                    b += (source.surface.data[(si + 2.0_f64) as usize] as f64);
                                    a += (source.surface.data[(si + 3.0_f64) as usize] as f64);
                                    {
                                        count += 1.0;
                                        count
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
                    if (count == 0.0_f64) {
                        {
                            bx += block;
                            bx
                        };
                        continue;
                    }
                    let ar = (r / count).round();
                    let ag = (g / count).round();
                    let ab = (b / count).round();
                    let aa = (a / count).round();
                    {
                        let mut py = by;
                        while (py < y_end) {
                            {
                                let mut px = bx;
                                while (px < x_end) {
                                    let di = (((py * w) + px) * 4.0_f64);
                                    out[di as usize] = (ar) as u8;
                                    out[(di + 1.0_f64) as usize] = (ag) as u8;
                                    out[(di + 2.0_f64) as usize] = (ab) as u8;
                                    out[(di + 3.0_f64) as usize] = (aa) as u8;
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
                    {
                        bx += block;
                        bx
                    };
                }
            }
            {
                by += block;
                by
            };
        }
    }
}
