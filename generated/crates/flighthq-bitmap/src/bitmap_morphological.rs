// @generated from upstream/packages/bitmap/src/bitmapMorphological.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BitmapRegion;

// Source: upstream/packages/bitmap/src/bitmapMorphological.ts:16 (sha256:598553d06185061eea921663f7966baa4e07ad7f7d5efc6df8d6db3a6ba7172c)
pub fn dilate_bitmap(out: &mut Vec<u8>, source: &BitmapRegion, radius: f64) -> () {
    apply_morphological(out, source, radius, true);
}

// Source: upstream/packages/bitmap/src/bitmapMorphological.ts:32 (sha256:fb0ad4ec56bf288d476070e2b9abd4186f813cde642b614faeebaf767a401c65)
pub fn erode_bitmap(out: &mut Vec<u8>, source: &BitmapRegion, radius: f64) -> () {
    apply_morphological(out, source, radius, false);
}

// Source: upstream/packages/bitmap/src/bitmapMorphological.ts:36 (sha256:872bd9eeb6a66f97d743b6f7749648e01a570d4f5fbfa7680df241584bead5d6)
fn apply_morphological(out: &mut Vec<u8>, source: &BitmapRegion, radius: f64, dilate: bool) -> () {
    let r = (0.0_f64).max((radius).round());
    let w = source.width;
    let h = source.height;
    let bitmap_width = source.bitmap.width;
    let bitmap_height = source.bitmap.height;
    let identity = if dilate { 0.0_f64 } else { 255.0_f64 };
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
                                .max((bitmap_height - 1.0_f64).min(((source.y + py) + ky)));
                            {
                                let mut kx = (-r);
                                while (kx <= r) {
                                    let sx = (0.0_f64)
                                        .max((bitmap_width - 1.0_f64).min(((source.x + px) + kx)));
                                    let si = (((sy * bitmap_width) + sx) * 4.0_f64);
                                    if dilate {
                                        if ((source.bitmap.data[si as usize] as f64) > v_r) {
                                            v_r = (source.bitmap.data[si as usize] as f64);
                                        }
                                        if ((source.bitmap.data[(si + 1.0_f64) as usize] as f64)
                                            > v_g)
                                        {
                                            v_g = (source.bitmap.data[(si + 1.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.bitmap.data[(si + 2.0_f64) as usize] as f64)
                                            > v_b)
                                        {
                                            v_b = (source.bitmap.data[(si + 2.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.bitmap.data[(si + 3.0_f64) as usize] as f64)
                                            > v_a)
                                        {
                                            v_a = (source.bitmap.data[(si + 3.0_f64) as usize]
                                                as f64);
                                        }
                                    } else {
                                        if ((source.bitmap.data[si as usize] as f64) < v_r) {
                                            v_r = (source.bitmap.data[si as usize] as f64);
                                        }
                                        if ((source.bitmap.data[(si + 1.0_f64) as usize] as f64)
                                            < v_g)
                                        {
                                            v_g = (source.bitmap.data[(si + 1.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.bitmap.data[(si + 2.0_f64) as usize] as f64)
                                            < v_b)
                                        {
                                            v_b = (source.bitmap.data[(si + 2.0_f64) as usize]
                                                as f64);
                                        }
                                        if ((source.bitmap.data[(si + 3.0_f64) as usize] as f64)
                                            < v_a)
                                        {
                                            v_a = (source.bitmap.data[(si + 3.0_f64) as usize]
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
