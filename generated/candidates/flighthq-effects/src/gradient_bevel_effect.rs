// @generated from upstream/packages/effects/src/gradientBevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GradientBevelEffect;

// Source: upstream/packages/effects/src/gradientBevelEffect.ts:4 (sha256:4cb3bdc0dcad1e5c16719a827a7acf1156d9de60481d2bf162759e64514596d6)
pub fn create_gradient_bevel_effect(options: &GradientBevelEffect) -> GradientBevelEffect {
    return GradientBevelEffect {
        kind: "GradientBevelEffect".to_owned(),
        ..((*options).clone()).clone()
    };
}
