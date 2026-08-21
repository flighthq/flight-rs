// @generated from upstream/packages/bitmap/src/bitmapFormat.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/bitmap/src/bitmapFormat.ts:41 (sha256:623dd5962255af2ee6fb33ea40278ce04d433fc96dc3a399c54a11170afbd982)
pub fn premultiply_bitmap_pixels(out: &mut Vec<u8>, source: &Vec<u8>, length: f64) -> () {
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
                i.clone()
            };
        }
    }
}

// Source: upstream/packages/bitmap/src/bitmapFormat.ts:63 (sha256:879b8be1e05103cb2627420e2b86fb9605e2b1a1f5b0c1bccb0b1d791c0145ff)
pub fn unpremultiply_bitmap_pixels(out: &mut Vec<u8>, source: &Vec<u8>, length: f64) -> () {
    {
        let mut i = 0.0_f64;
        while (i < length) {
            let a = (source[(i + 3.0_f64) as usize] as f64);
            if ((a).clone() == 0.0_f64) {
                out[i as usize] = (0.0_f64) as u8;
                out[(i + 1.0_f64) as usize] = (0.0_f64) as u8;
                out[(i + 2.0_f64) as usize] = (0.0_f64) as u8;
                out[(i + 3.0_f64) as usize] = (0.0_f64) as u8;
            } else {
                let inv = (255.0_f64 / (a).clone());
                out[i as usize] =
                    ((255.0_f64).min(((source[i as usize] as f64) * inv).round())) as u8;
                out[(i + 1.0_f64) as usize] = ((255.0_f64)
                    .min(((source[(i + 1.0_f64) as usize] as f64) * inv).round()))
                    as u8;
                out[(i + 2.0_f64) as usize] = ((255.0_f64)
                    .min(((source[(i + 2.0_f64) as usize] as f64) * inv).round()))
                    as u8;
                out[(i + 3.0_f64) as usize] = ((a).clone()) as u8;
            }
            {
                i += 4.0_f64;
                i.clone()
            };
        }
    }
}
