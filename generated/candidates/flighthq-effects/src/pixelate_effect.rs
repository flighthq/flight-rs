// @generated from upstream/packages/effects/src/pixelateEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::PixelateEffect;

// Source: upstream/packages/effects/src/pixelateEffect.ts:3 (sha256:db945901f90476fd1ead5a7daf4df2b6619d7a93691e6e6f8b9eac4949e89189)
#[derive(Clone)]
struct CreatePixelateEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreatePixelateEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_pixelate_effect(options: Option<PixelateEffect>) -> PixelateEffect {
    let options = options.unwrap_or(PixelateEffect {
        __flight_identity: std::sync::Arc::new(()),
        size: None,
    });
    return PixelateEffect {
        kind: "PixelateEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
