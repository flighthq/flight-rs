// @generated from upstream/packages/effects/src/compositeEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{CompositeEffect, CompositeOperator};

// Source: upstream/packages/effects/src/compositeEffect.ts:9 (sha256:ddeffc215988871bb79eb05f6c45224a0027a6c2fa04a0a818ef1f17b8c1cb2f)
#[derive(Clone)]
struct CreateCompositeEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateCompositeEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_composite_effect(
    operator: CompositeOperator,
    options: Option<CompositeEffect>,
) -> CompositeEffect {
    let options = options.unwrap_or(CompositeEffect {
        __flight_identity: std::sync::Arc::new(()),
        backdrop_key: None,
    });
    return CompositeEffect {
        kind: "CompositeEffect".to_owned(),
        operator: (operator).clone(),
        ..((options).clone()).clone()
    };
}
