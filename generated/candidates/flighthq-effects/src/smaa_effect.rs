// @generated from upstream/packages/effects/src/smaaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SmaaEffect;

// Source: upstream/packages/effects/src/smaaEffect.ts:3 (sha256:9c5fa8e51982ceba3092993c452263c5c6afaeac130240b525438d9105dbb230)
#[derive(Clone)]
struct CreateSmaaEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSmaaEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_smaa_effect(options: Option<SmaaEffect>) -> SmaaEffect {
    let options = options.unwrap_or(SmaaEffect {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
    });
    return SmaaEffect {
        kind: "SmaaEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
