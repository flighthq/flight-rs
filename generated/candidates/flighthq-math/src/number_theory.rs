// @generated from upstream/packages/math/src/numberTheory.ts; do not edit.
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

// Source: upstream/packages/math/src/numberTheory.ts:7 (sha256:30e08c82ac68e529fa3df91235a1ec57f8fb59171dd2c5a8e44d4d51ab0a71d2)
pub fn factorial(n: f64) -> f64 {
    if (!(n).is_finite() && (n).fract() == 0.0_f64) || (n < 0.0_f64) {
        panic!("{}", "generated Flight function threw");
    }
    if (n == 0.0_f64) || (n == 1.0_f64) {
        return 1.0_f64;
    }
    let mut result = 1.0_f64;
    {
        let mut i = 2.0_f64;
        while (i <= n) {
            result *= i;
            {
                i += 1.0;
                i
            };
        }
    }
    return result;
}

// Source: upstream/packages/math/src/numberTheory.ts:21 (sha256:35bacbbe2e313ef7f34121ee04c6e1f6ae2da016ead068067c968ca54938b96a)
pub fn gcd(mut a: f64, mut b: f64) -> f64 {
    a = ((a).trunc()).abs();
    b = ((b).trunc()).abs();
    if (a == 0.0_f64) && (b == 0.0_f64) {
        panic!("{}", "generated Flight function threw");
    }
    while (b != 0.0_f64) {
        let t = b;
        b = (a % b);
        a = t;
    }
    return a;
}

// Source: upstream/packages/math/src/numberTheory.ts:39 (sha256:f26172e44fd6ded6e2436bca4ce5665c7173bc3de696c6e4bc996d0ae35559aa)
pub fn hypot2(x: f64, y: f64) -> f64 {
    return ((x * x) + (y * y));
}

// Source: upstream/packages/math/src/numberTheory.ts:44 (sha256:deddbf2d340247d69879add7c5afba9fe2210f4475bbbd62bbec3b14f0ee4675)
pub fn is_even(n: f64) -> bool {
    return ((__flight_js_to_i32(n) & __flight_js_to_i32(1.0_f64)) as f64 == 0.0_f64);
}

// Source: upstream/packages/math/src/numberTheory.ts:49 (sha256:eab6d3d96f0f212741193097a7b9dcbff8344d9f02998c60f7c813ee86625b79)
pub fn is_odd(n: f64) -> bool {
    return ((__flight_js_to_i32(n) & __flight_js_to_i32(1.0_f64)) as f64 == 1.0_f64);
}

// Source: upstream/packages/math/src/numberTheory.ts:59 (sha256:4740d7680d66d4f101fd0ab5108540e50cc4e1ce331897c50fb25d88bb87ff42)
pub fn lcm(a: f64, b: f64) -> f64 {
    let g = gcd(a, b);
    return ((((a).trunc()).abs() / g) * ((b).trunc()).abs());
}
