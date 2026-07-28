// @generated from upstream/packages/surface/src/surfaceFormat.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/surface/src/surfaceFormat.ts:41 (sha256:ebbd82dfc3362d6c8164d3794d0769a23cc7a2b76add51d33688d93d170aa3e0)
pub fn premultiply_surface_pixels(out: &mut Vec<u8>, source: &Vec<u8>, length: f64) -> () {
    {
        let mut i = 0.0_f64;
        while (i < length) {
            let a = ((source[(i + 3.0_f64) as usize] as f64) / 255.0_f64);
            out[i as usize] = (((source[i as usize] as f64) * a).round()) as u8;
            out[(i + 1.0_f64) as usize] =
                (((source[(i + 1.0_f64) as usize] as f64) * a).round()) as u8;
            out[(i + 2.0_f64) as usize] =
                (((source[(i + 2.0_f64) as usize] as f64) * a).round()) as u8;
            out[(i + 3.0_f64) as usize] = (source[(i + 3.0_f64) as usize] as f64) as u8;
            {
                i += 4.0_f64;
                i
            };
        }
    }
}

// Source: upstream/packages/surface/src/surfaceFormat.ts:63 (sha256:7e333b600e3c267761362cded5c0284a1f828b744fc034d58da6ca71a43ffa05)
pub fn unpremultiply_surface_pixels(out: &mut Vec<u8>, source: &Vec<u8>, length: f64) -> () {
    {
        let mut i = 0.0_f64;
        while (i < length) {
            let a = (source[(i + 3.0_f64) as usize] as f64);
            if (a == 0.0_f64) {
                out[i as usize] = (0.0_f64) as u8;
                out[(i + 1.0_f64) as usize] = (0.0_f64) as u8;
                out[(i + 2.0_f64) as usize] = (0.0_f64) as u8;
                out[(i + 3.0_f64) as usize] = (0.0_f64) as u8;
            } else {
                let inv = (255.0_f64 / a);
                out[i as usize] =
                    ((255.0_f64).min(((source[i as usize] as f64) * inv).round())) as u8;
                out[(i + 1.0_f64) as usize] = ((255.0_f64)
                    .min(((source[(i + 1.0_f64) as usize] as f64) * inv).round()))
                    as u8;
                out[(i + 2.0_f64) as usize] = ((255.0_f64)
                    .min(((source[(i + 2.0_f64) as usize] as f64) * inv).round()))
                    as u8;
                out[(i + 3.0_f64) as usize] = (a) as u8;
            }
            {
                i += 4.0_f64;
                i
            };
        }
    }
}
