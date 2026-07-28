// @generated from upstream/packages/effects/src/toneMapEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ToneMapEffect;

// Source: upstream/packages/effects/src/toneMapEffect.ts:3 (sha256:911baa094a5c4361d603a14376b771ce11472f9a4b2778bd07ba77293ff2b88a)
#[derive(Clone)]
struct CreateToneMapEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateToneMapEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tone_map_effect(options: Option<ToneMapEffect>) -> ToneMapEffect {
    let options = options.unwrap_or(ToneMapEffect {
        __flight_identity: std::sync::Arc::new(()),
        operator: None,
        exposure: None,
        white: None,
    });
    return ToneMapEffect {
        kind: "ToneMapEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
