// @generated from upstream/packages/shading/src/createEmissiveModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    EMISSIVE_MODIFIER_FACING as emissive_modifier_facing_constant,
    EMISSIVE_MODIFIER_KIND as emissive_modifier_kind_constant, EmissiveModifier,
    MODIFIER_SLOT as modifier_slot_constant, Texture,
};

// Source: upstream/packages/shading/src/createEmissiveModifier.ts:8 (sha256:3178f9ac65a057f14a0f654c4380a341fe76b57d865832040c2b7e3f3a6bf79c)
#[derive(Clone, Default)]
pub struct EmissiveModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: f64,
    pub strength: Option<f64>,
    pub mask: Option<Texture>,
    pub facing: Option<EmissiveModifierFacing>,
    pub facing_softness: Option<f64>,
}
impl PartialEq for EmissiveModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/createEmissiveModifier.ts:22 (sha256:5a994d85eaed421d81eed5e0745f70f852bdf712f1f1902d992eec8c0fa9f722)
pub fn create_emissive_modifier(options: &EmissiveModifierOptions) -> EmissiveModifier {
    let mut modifier: EmissiveModifier = EmissiveModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (emissive_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.emissive).clone(),
        color: options.color,
        strength: (options.strength).unwrap_or(1.0_f64),
        facing: Some(
            ((options.facing).clone())
                .unwrap_or((emissive_modifier_facing_constant.ignore).clone()),
        ),
        facing_softness: Some((options.facing_softness).unwrap_or(0.0_f64)),
        mask: None,
        ..Default::default()
    };
    if ((options.mask).clone()).is_some() {
        modifier.mask = (options.mask).clone();
    }
    return modifier;
}
