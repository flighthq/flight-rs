// @generated from upstream/packages/effects/src/gradientGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GradientGlowEffect;

// Source: upstream/packages/effects/src/gradientGlowEffect.ts:4 (sha256:156011c5d67f1c2c702b7a6a7438ffb0bcb8c11b33c0424e0d96193ce6833158)
pub fn create_gradient_glow_effect(options: &GradientGlowEffect) -> GradientGlowEffect {
    return GradientGlowEffect {
        kind: "GradientGlowEffect".to_owned(),
        ..((*options).clone()).clone()
    };
}
