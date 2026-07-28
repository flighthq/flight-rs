// @generated from upstream/packages/easing/src/easeBack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/easeBack.ts:3 (sha256:5a407d8ee850d00ebab6789fe103585fcb94a0abebeea0bb4f8b6ae4909885a2)
pub fn ease_in_back(t: f64) -> f64 {
    return ((t * t) * (((S + 1.0_f64) * t) - S));
}

// Source: upstream/packages/easing/src/easeBack.ts:5 (sha256:584987cc817056fbdefc16905ebf530b6cea61a88bdffdded7da1eda6c5e0317)
pub fn ease_in_out_back(mut t: f64) -> f64 {
    return if ({
        t *= 2.0_f64;
        t
    } < 1.0_f64)
    {
        (0.5_f64 * ((t * t) * (((S2 + 1.0_f64) * t) - S2)))
    } else {
        (0.5_f64
            * ((({
                t -= 2.0_f64;
                t
            } * t)
                * (((S2 + 1.0_f64) * t) + S2))
                + 2.0_f64))
    };
}

// Source: upstream/packages/easing/src/easeBack.ts:8 (sha256:c6836f34b82f65191fbda8b7d1de6c31ab7752f77208ab52f1399188a9034104)
pub fn ease_out_back(mut t: f64) -> f64 {
    return ((({
        t -= 1.0_f64;
        t
    } * t)
        * (((S + 1.0_f64) * t) + S))
        + 1.0_f64);
}

// Source: upstream/packages/easing/src/easeBack.ts:10 (sha256:d7c6f133141d03781b4c2344f2cd62740f3dbd6baf6160c190f4a768655e7d9a)
const S: f64 = 1.70158_f64;

// Source: upstream/packages/easing/src/easeBack.ts:11 (sha256:ea07cd37e6377729d2093e0b6c7dc70d403aecbcf83449f66aa92360fb2ed2a2)
const S2: f64 = 2.5949095_f64;
