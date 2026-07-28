// @generated from upstream/packages/effects/src/customShaderEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::CustomShaderEffect;

// Source: upstream/packages/effects/src/customShaderEffect.ts:3 (sha256:9542b2a533be83b99807f767481c502e2e2e937e696bda8edfbf4375f0e41a57)
pub fn create_custom_shader_effect(options: &CustomShaderEffect) -> CustomShaderEffect {
    return CustomShaderEffect {
        kind: "CustomShaderEffect".to_owned(),
        ..((*options).clone()).clone()
    };
}
