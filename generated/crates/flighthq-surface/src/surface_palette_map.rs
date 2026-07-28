// @generated from upstream/packages/surface/src/surfacePaletteMap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SurfaceRegion;

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

// Source: upstream/packages/surface/src/surfacePaletteMap.ts:18 (sha256:ba8a8ae918ecd3407003d27304d3cea190c8a36e6748fcb59a0e1747f0151b8b)
pub fn apply_surface_palette_map(
    dest: &mut SurfaceRegion,
    source: &SurfaceRegion,
    red_map: Option<Vec<f64>>,
    green_map: Option<Vec<f64>>,
    blue_map: Option<Vec<f64>>,
    alpha_map: Option<Vec<f64>>,
) -> () {
    let w = (dest.width).min(source.width);
    let h = (dest.height).min(source.height);
    {
        let mut py = 0.0_f64;
        while (py < h) {
            let sy = (source.y + py);
            let dy = (dest.y + py);
            if (((sy < 0.0_f64) || (sy >= source.surface.height)) || (dy < 0.0_f64))
                || (dy >= dest.surface.height)
            {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < w) {
                    let sx = (source.x + px);
                    let dx = (dest.x + px);
                    if (((sx < 0.0_f64) || (sx >= source.surface.width)) || (dx < 0.0_f64))
                        || (dx >= dest.surface.width)
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((sy * source.surface.width) + sx) * 4.0_f64);
                    let di = (((dy * dest.surface.width) + dx) * 4.0_f64);
                    let r = (source.surface.data[si as usize] as f64);
                    let g = (source.surface.data[(si + 1.0_f64) as usize] as f64);
                    let b = (source.surface.data[(si + 2.0_f64) as usize] as f64);
                    let a = (source.surface.data[(si + 3.0_f64) as usize] as f64);
                    dest.surface.data[di as usize] = if (red_map).is_some() {
                        (red_map.as_ref().unwrap()[r as usize].clone()) as u8
                    } else {
                        ((r).clone()) as u8
                    };
                    dest.surface.data[(di + 1.0_f64) as usize] = if (green_map).is_some() {
                        (green_map.as_ref().unwrap()[g as usize].clone()) as u8
                    } else {
                        ((g).clone()) as u8
                    };
                    dest.surface.data[(di + 2.0_f64) as usize] = if (blue_map).is_some() {
                        (blue_map.as_ref().unwrap()[b as usize].clone()) as u8
                    } else {
                        ((b).clone()) as u8
                    };
                    dest.surface.data[(di + 3.0_f64) as usize] = if (alpha_map).is_some() {
                        (alpha_map.as_ref().unwrap()[a as usize].clone()) as u8
                    } else {
                        ((a).clone()) as u8
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
    {
        dest.surface.version = (__flight_js_to_u32((dest.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}
