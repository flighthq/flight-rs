// @generated from upstream/packages/effects/src/outlineEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::OutlineEffect;

// Source: upstream/packages/effects/src/outlineEffect.ts:3 (sha256:f7d12c4a36e281c4b105db315466a95bd7146bfe1235db79e3ac2ac54022e1b8)
#[derive(Clone)]
struct CreateOutlineEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateOutlineEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_outline_effect(options: Option<OutlineEffect>) -> OutlineEffect {
    let options = options.unwrap_or(OutlineEffect {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
        thickness: None,
        color: None,
    });
    return OutlineEffect {
        kind: "OutlineEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
