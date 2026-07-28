// @generated from upstream/packages/surface/src/surfaceMorphological.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SurfaceRegion;

// Source: upstream/packages/surface/src/surfaceMorphological.ts:16 (sha256:154b06e75c5435a4216065d2a357331826287fd1af81360ba35216abeda0a9f6)
pub fn dilate_surface(out: &mut Vec<u8>, source: &SurfaceRegion, radius: f64) -> () {
    apply_morphological(out, source, radius, true);
}

// Source: upstream/packages/surface/src/surfaceMorphological.ts:32 (sha256:1454ba0f2686e07b3def394ba5b8efe46427c00e093292b8e6a31b04950a76da)
pub fn erode_surface(out: &mut Vec<u8>, source: &SurfaceRegion, radius: f64) -> () {
    apply_morphological(out, source, radius, false);
}

// Source: upstream/packages/surface/src/surfaceMorphological.ts:36 (sha256:761ab27a7bdc887b493292a08071bde62211d37ea60fda81e6222f4276b7d1db)
fn apply_morphological(out: &mut Vec<u8>, source: &SurfaceRegion, radius: f64, dilate: bool) -> () {
    let r = (0.0_f64).max((radius).round());
    let w = source.width;
    let h = source.height;
    let surface_width = source.surface.width;
    let surface_height = source.surface.height;
    let mut identity = if dilate { 0.0_f64 } else { 255.0_f64 };
    {
        let mut py = 0.0_f64;
        while (py < h) {
            {
                let mut px = 0.0_f64;
                while (px < w) {
                    let mut v_r = identity;
                    let mut v_g = identity;
                    let mut v_b = identity;
                    let mut v_a = identity;
                    {
                        let mut ky = (-r);
                        while (ky <= r) {
                            let sy = (0.0_f64)
                                .max((surface_height - 1.0_f64).min(((source.y + py) + ky)));
                            {
                                let mut kx = (-r);
                                while (kx <= r) {
                                    let sx = (0.0_f64)
                                        .max((surface_width - 1.0_f64).min(((source.x + px) + kx)));
                                    let si = (((sy * surface_width) + sx) * 4.0_f64);
                                    if dilate {
                                        if ((source.surface.data[si as usize] as f64) > v_r) {
                                            v_r = (source.surface.data[si as usize] as f64);
                                        }
                                        if ((source.surface.data[(si + 1.0_f64) as usize] as f64)
                                            > v_g)
                                        {
                                            v_g = (source.surface.data[(si + 1.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.surface.data[(si + 2.0_f64) as usize] as f64)
                                            > v_b)
                                        {
                                            v_b = (source.surface.data[(si + 2.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.surface.data[(si + 3.0_f64) as usize] as f64)
                                            > v_a)
                                        {
                                            v_a = (source.surface.data[(si + 3.0_f64) as usize]
                                                as f64);
                                        }
                                    } else {
                                        if ((source.surface.data[si as usize] as f64) < v_r) {
                                            v_r = (source.surface.data[si as usize] as f64);
                                        }
                                        if ((source.surface.data[(si + 1.0_f64) as usize] as f64)
                                            < v_g)
                                        {
                                            v_g = (source.surface.data[(si + 1.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.surface.data[(si + 2.0_f64) as usize] as f64)
                                            < v_b)
                                        {
                                            v_b = (source.surface.data[(si + 2.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.surface.data[(si + 3.0_f64) as usize] as f64)
                                            < v_a)
                                        {
                                            v_a = (source.surface.data[(si + 3.0_f64) as usize]
                                                as f64);
                                        }
                                    }
                                    {
                                        kx += 1.0;
                                        kx
                                    };
                                }
                            }
                            {
                                ky += 1.0;
                                ky
                            };
                        }
                    }
                    let di = (((py * w) + px) * 4.0_f64);
                    out[di as usize] = (v_r) as u8;
                    out[(di + 1.0_f64) as usize] = (v_g) as u8;
                    out[(di + 2.0_f64) as usize] = (v_b) as u8;
                    out[(di + 3.0_f64) as usize] = (v_a) as u8;
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
}
