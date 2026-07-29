// @generated from upstream/packages/shading/src/createDissolveModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    DISSOLVE_MODIFIER_KIND as dissolve_modifier_kind_constant, DissolveModifier,
    MODIFIER_SLOT as modifier_slot_constant, Texture,
};

// Source: upstream/packages/shading/src/createDissolveModifier.ts:7 (sha256:877e24a08322880ba5714aa0116f27f119d937d476e1922d21694b5d5bb03c36)
#[derive(Clone, Default)]
pub struct DissolveModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub threshold: f64,
    pub edge_color: Option<f64>,
    pub edge_width: Option<f64>,
    pub map: Option<Texture>,
    pub scale: Option<f64>,
}
impl PartialEq for DissolveModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/createDissolveModifier.ts:21 (sha256:f82b270b64ac7a079568e2f24aa4c58e97ac96fbe98ed22fd4b9fd16f78cbf43)
pub fn create_dissolve_modifier(options: &DissolveModifierOptions) -> DissolveModifier {
    let mut modifier: DissolveModifier = DissolveModifier {
        __flight_identity: std::sync::Arc::new(()),
        kind: (dissolve_modifier_kind_constant).to_owned(),
        slot: (modifier_slot_constant.effect).clone(),
        threshold: options.threshold,
        edge_color: (options.edge_color).unwrap_or(4284875007.0_f64),
        edge_width: Some((options.edge_width).unwrap_or(0.05_f64)),
        scale: Some((options.scale).unwrap_or(8.0_f64)),
        map: None,
        ..Default::default()
    };
    if ((options.map).clone()).is_some() {
        modifier.map = (options.map).clone();
    }
    return modifier;
}
