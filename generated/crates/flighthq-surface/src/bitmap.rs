// @generated from upstream/packages/bitmap/src/bitmap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Bitmap;

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

// Source: upstream/packages/bitmap/src/bitmap.ts:86 (sha256:57ebc6182b4a8576436054bee50af43efa04bf54c250803d1e6886b95d496288)
pub fn invalidate_bitmap(bitmap: &mut Bitmap) -> () {
    bitmap.version = (__flight_js_to_u32((bitmap.version + 1.0_f64))
        >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}
