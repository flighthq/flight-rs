// @generated from upstream/packages/math/src/randomRange.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/math/src/randomRange.ts:7 (sha256:8f208154453c53b7e03234be546540742a30cbc7027fcfc55e05527d19080280)
pub fn random_bool(random: &mut impl FnMut() -> f64, probability: Option<f64>) -> bool {
    let probability = probability.unwrap_or(0.5_f64);
    return (random() < probability);
}

// Source: upstream/packages/math/src/randomRange.ts:17 (sha256:fe71df8d01af8fb4bf7e2664044cd7b756cca9627c8cab6b1bb2e5cacc2a9838)
pub fn random_int(random: &mut impl FnMut() -> f64, min: f64, max: f64) -> f64 {
    let lo = (min).floor();
    let hi = (max).floor();
    if (lo > hi) {
        panic!("{}", "generated Flight function threw");
    }
    return (lo + (random() * ((hi - lo) + 1.0_f64)).floor());
}

// Source: upstream/packages/math/src/randomRange.ts:25 (sha256:8235edb4e7bfd9ba366494e35d4b341cb63e1cf00251849b24c3d2fe53c4d4d5)
pub fn random_range(random: &mut impl FnMut() -> f64, min: f64, max: f64) -> f64 {
    return (min + (random() * (max - min)));
}

// Source: upstream/packages/math/src/randomRange.ts:30 (sha256:3976089e518ad162fe4083f8dc43307fcc7d475551e74d9313e99eaedb2c2816)
pub fn random_sign(random: &mut impl FnMut() -> f64) -> f64 {
    return if (random() < 0.5_f64) {
        (-1.0_f64)
    } else {
        1.0_f64
    };
}
