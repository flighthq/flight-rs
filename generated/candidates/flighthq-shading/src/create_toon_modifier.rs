// @generated from upstream/packages/shading/src/createToonModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    MODIFIER_SLOT as modifier_slot_constant, TOON_MODIFIER_KIND as toon_modifier_kind_constant,
    ToonModifier,
};

// Source: upstream/packages/shading/src/createToonModifier.ts:7 (sha256:d7d97edb945a99e12f016a5cecf37e6b874d9b5b38f459c0e5a09568992b6233)
#[derive(Clone, Default)]
pub struct ToonModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub steps: f64,
    pub smoothness: Option<f64>,
}
impl PartialEq for ToonModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/createToonModifier.ts:17 (sha256:c15b55cbaa8c33522076928d6440b90450a37baf565c365751ba97bb70f61b3c)
pub fn create_toon_modifier(options: &ToonModifierOptions) -> ToonModifier {
    return ToonModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (toon_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        steps: options.steps,
        smoothness: Some((options.smoothness).unwrap_or(0.0_f64)),
        ..Default::default()
    };
}
