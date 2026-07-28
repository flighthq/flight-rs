// @generated from upstream/packages/easing/src/easeElastic.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeElastic.ts:3 (sha256:fb65745f3b406ea7cfd8b309fb0697d770eb8a9b2745edbca64da42834df7b8a)
pub fn ease_in_elastic(mut t: f64) -> f64 {
    if ((t == 0.0_f64) || (t == 1.0_f64)) {
        return t;
    }
    return (-((2.0_f64).powf(
        (10.0_f64 * {
            t -= 1.0_f64;
            t
        }),
    ) * (((t - S) * (2.0_f64 * std::f64::consts::PI)) / P).sin()));
}

// Source: upstream/packages/easing/src/easeElastic.ts:8 (sha256:486aca4392dc2ed3c79caf688d63b3f9e50d6cc44dabaae15c7ce4c6940a57dc)
pub fn ease_in_out_elastic(mut t: f64) -> f64 {
    if ((t == 0.0_f64) || (t == 1.0_f64)) {
        return t;
    }
    if ({
        t *= 2.0_f64;
        t
    } < 1.0_f64)
    {
        return ((-0.5_f64)
            * ((2.0_f64).powf(
                (10.0_f64 * {
                    t -= 1.0_f64;
                    t
                }),
            ) * (((t - S2) * (2.0_f64 * std::f64::consts::PI)) / P2).sin()));
    }
    return (((0.5_f64
        * (2.0_f64).powf(
            ((-10.0_f64) * {
                t -= 1.0_f64;
                t
            }),
        ))
        * (((t - S2) * (2.0_f64 * std::f64::consts::PI)) / P2).sin())
        + 1.0_f64);
}

// Source: upstream/packages/easing/src/easeElastic.ts:14 (sha256:1b00d8c204f9edcd8dec94c70d2efdebd6e5099797902be35831d354bb6574c0)
pub fn ease_out_elastic(t: f64) -> f64 {
    if ((t == 0.0_f64) || (t == 1.0_f64)) {
        return t;
    }
    return (((2.0_f64).powf(((-10.0_f64) * t))
        * (((t - S) * (2.0_f64 * std::f64::consts::PI)) / P).sin())
        + 1.0_f64);
}

// Source: upstream/packages/easing/src/easeElastic.ts:19 (sha256:a9d927816335f2a62a4110c2def31b21379ff0d31d9a4c1bdcf4752856322156)
const P: f64 = 0.4_f64;

// Source: upstream/packages/easing/src/easeElastic.ts:20 (sha256:978909773cd06e9fb4ac0d6eff02f77f12e712e1278b37e9a1502b1c82855a7e)
const P2: f64 = 0.45_f64;

// Source: upstream/packages/easing/src/easeElastic.ts:21 (sha256:47bc2e91ec14cd4d57553cb042cc5c62b7a72437e3fc00104fa014a026e30cb5)
const S: f64 = 0.09999999999999999_f64;

// Source: upstream/packages/easing/src/easeElastic.ts:22 (sha256:8f7ae79d71b799ed7136dfe12717f6c606cf72e2ec22c5127cb133b38fb3cb5f)
const S2: f64 = 0.1125_f64;
