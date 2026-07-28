// @generated from upstream/packages/effects/src/medianEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::MedianEffect;

// Source: upstream/packages/effects/src/medianEffect.ts:3 (sha256:242f61ead3733bc4a73a7602e314a77353bb6dff116c0b4df976d02486f19625)
#[derive(Clone)]
struct CreateMedianEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateMedianEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_median_effect(options: Option<MedianEffect>) -> MedianEffect {
    let options = options.unwrap_or(MedianEffect {
        __flight_identity: std::sync::Arc::new(()),
        radius: None,
    });
    return MedianEffect {
        kind: "MedianEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
