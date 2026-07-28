// @generated from upstream/packages/effects/src/chromaticAberrationEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ChromaticAberrationEffect;

// Source: upstream/packages/effects/src/chromaticAberrationEffect.ts:3 (sha256:456450d4b5d8fd416d14fc8ec9a8ec513ca50ec885dd193183d640ea2b163629)
#[derive(Clone)]
struct CreateChromaticAberrationEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateChromaticAberrationEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_chromatic_aberration_effect(
    options: Option<ChromaticAberrationEffect>,
) -> ChromaticAberrationEffect {
    let options = options.unwrap_or(ChromaticAberrationEffect {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        radial: None,
    });
    return ChromaticAberrationEffect {
        kind: "ChromaticAberrationEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
