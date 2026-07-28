// @generated from upstream/packages/effects/src/bloomEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BloomEffect;

// Source: upstream/packages/effects/src/bloomEffect.ts:7 (sha256:f34073dbd363e96e331b2a1ba428bb190daccbacf4c0dbd957021b90b65025fd)
pub fn compute_bloom_blur_radius(effect: &BloomEffect) -> f64 {
    return (0.0_f64).max((effect.radius).unwrap_or(8.0_f64));
}

// Source: upstream/packages/effects/src/bloomEffect.ts:11 (sha256:4a39a4516008094d20a658965d4a461ad85821a29fc50ae8aeecc9a69bd381f7)
pub fn compute_bloom_intensity(effect: &BloomEffect) -> f64 {
    return (effect.intensity).unwrap_or(1.0_f64);
}

// Source: upstream/packages/effects/src/bloomEffect.ts:15 (sha256:cd461a958a93139b3234d1695ca5bfc70a5c93c0989ea26a312389baa4929218)
pub fn compute_bloom_threshold(effect: &BloomEffect) -> f64 {
    return (effect.threshold).unwrap_or(0.8_f64);
}

// Source: upstream/packages/effects/src/bloomEffect.ts:19 (sha256:617321597270316572516fdf538933545aea4fa2d8cd22b426fc4e882207e95f)
#[derive(Clone)]
struct CreateBloomEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBloomEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_bloom_effect(options: Option<BloomEffect>) -> BloomEffect {
    let options = options.unwrap_or(BloomEffect {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
        intensity: None,
        radius: None,
        passes: None,
    });
    return BloomEffect {
        kind: "BloomEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
