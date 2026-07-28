// @generated from upstream/packages/effects/src/blurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BlurEffect;

// Source: upstream/packages/effects/src/blurEffect.ts:6 (sha256:c201dc944b5cc997ed7759922a210418e3312ca5fa5dc98527912545d52f144e)
#[derive(Clone)]
struct CreateBlurEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBlurEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_blur_effect(options: Option<BlurEffect>) -> BlurEffect {
    let options = options.unwrap_or(BlurEffect {
        __flight_identity: std::sync::Arc::new(()),
        blur_x: None,
        blur_y: None,
    });
    return BlurEffect {
        kind: "BlurEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
