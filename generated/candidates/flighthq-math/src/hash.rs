// @generated from upstream/packages/math/src/hash.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_random_source;
use flighthq_types::RandomSource;

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/math/src/hash.ts:15 (sha256:6b1a54f465e245adf9fff6cd015b3eaae39ce51c4421dd711b1aea9c9bbac506)
pub fn create_random_source_from_hash(x: f64, y: f64) -> RandomSource {
    return create_random_source(hash2_d(x, y));
}

// Source: upstream/packages/math/src/hash.ts:24 (sha256:10879b04dfdf82452392620bb4e5b2e992eba6b7731b304691811882fd5bc052)
pub fn hash2_d(x: f64, y: f64) -> f64 {
    return hash_combine(
        hash_uint32((__flight_js_to_i32(x) | __flight_js_to_i32(0.0_f64)) as f64),
        (__flight_js_to_i32(y) | __flight_js_to_i32(0.0_f64)) as f64,
    );
}

// Source: upstream/packages/math/src/hash.ts:29 (sha256:bf19ed4b2fdea75f3b63afdf7f7205a759555f936083b391c1501d82a6b8d0f0)
pub fn hash3_d(x: f64, y: f64, z: f64) -> f64 {
    return hash_combine(
        hash2_d(x, y),
        (__flight_js_to_i32(z) | __flight_js_to_i32(0.0_f64)) as f64,
    );
}

// Source: upstream/packages/math/src/hash.ts:40 (sha256:fccc19b94946895fc1256a5937cea22999c3965ebf682ac638c4281c0da14f76)
pub fn hash_combine(seed: f64, value: f64) -> f64 {
    return hash_uint32(
        (__flight_js_to_i32(seed)
            ^ __flight_js_to_i32(
                (((value + 2654435769.0_f64)
                    + __flight_js_to_i32(seed).wrapping_shl((__flight_js_to_u32(6.0_f64) & 31))
                        as f64)
                    + (__flight_js_to_i32(seed) >> (__flight_js_to_u32(2.0_f64) & 31)) as f64),
            )) as f64,
    );
}

// Source: upstream/packages/math/src/hash.ts:49 (sha256:08b5f83ba11d2da6eb69e5f671a3876c4a99207a573e896b8a62a190f3630b01)
pub fn hash_uint32(value: f64) -> f64 {
    let mut h = (__flight_js_to_i32(value) | __flight_js_to_i32(0.0_f64)) as f64;
    h = (__flight_js_to_i32(h)
        ^ __flight_js_to_i32((__flight_js_to_u32(h) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64))
        as f64;
    h = (__flight_js_to_i32(
        __flight_js_to_i32(h).wrapping_mul(__flight_js_to_i32(2246822507.0_f64)) as f64,
    ) | __flight_js_to_i32(0.0_f64)) as f64;
    h = (__flight_js_to_i32(h)
        ^ __flight_js_to_i32((__flight_js_to_u32(h) >> (__flight_js_to_u32(13.0_f64) & 31)) as f64))
        as f64;
    h = (__flight_js_to_i32(
        __flight_js_to_i32(h).wrapping_mul(__flight_js_to_i32(3266489909.0_f64)) as f64,
    ) | __flight_js_to_i32(0.0_f64)) as f64;
    h = (__flight_js_to_i32(h)
        ^ __flight_js_to_i32((__flight_js_to_u32(h) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64))
        as f64;
    return (__flight_js_to_u32(h) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}
