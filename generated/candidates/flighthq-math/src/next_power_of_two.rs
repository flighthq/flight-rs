// @generated from upstream/packages/math/src/nextPowerOfTwo.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

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

// Source: upstream/packages/math/src/nextPowerOfTwo.ts:5 (sha256:3b71240a4831585ba3c45ac3f7631da706025c857bcea6fe7d632b91e0afc52b)
pub fn is_power_of_two(n: f64) -> bool {
    return (n > 0.0_f64)
        && ((__flight_js_to_i32(n) & __flight_js_to_i32((n - 1.0_f64))) as f64 == 0.0_f64);
}

// Source: upstream/packages/math/src/nextPowerOfTwo.ts:13 (sha256:1e2d1bc2e7f38cc13726108b6ece732e5a750f00ce6e1a50d726011d9ca76bc3)
pub fn next_multiple_of(value: f64, multiple: f64) -> f64 {
    if (multiple <= 0.0_f64) {
        return value;
    }
    let remainder = (value % multiple);
    return if (remainder == 0.0_f64) {
        value
    } else {
        ((value + multiple) - remainder)
    };
}

// Source: upstream/packages/math/src/nextPowerOfTwo.ts:24 (sha256:c2f8436b20b6ac2bbeb281a97498d5d4a279441baab85b3f049c047c33829183)
pub fn next_power_of_two(mut n: f64) -> f64 {
    if (n <= 1.0_f64) {
        return 1.0_f64;
    }
    n = (__flight_js_to_i32((n - 1.0_f64)) | __flight_js_to_i32(0.0_f64)) as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(2.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(4.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64))
        as f64;
    return (__flight_js_to_u32((n + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/math/src/nextPowerOfTwo.ts:41 (sha256:8ea939a05d9008d9252ccbe6072de4c00babae51ccc9b4e0fd148026bffe1db8)
pub fn previous_power_of_two(mut n: f64) -> f64 {
    if (n <= 1.0_f64) {
        return 1.0_f64;
    }
    n = (__flight_js_to_i32(n) | __flight_js_to_i32(0.0_f64)) as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(2.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(4.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64))
        as f64;
    n = (__flight_js_to_i32(n)
        | __flight_js_to_i32((__flight_js_to_i32(n) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64))
        as f64;
    return (__flight_js_to_i32((n + 1.0_f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
}
