// @generated from upstream/packages/node/src/hasBlendMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BlendMode, HasBlendMode};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/node/src/hasBlendMode.ts:3 (sha256:c3e028c7ceaad82cb426a3ae1e6c5ef92bf842e4a4c0bdf76c459fbc1c06f7d8)
pub fn init_blend_mode_trait(target: &mut HasBlendMode, obj: Option<FlightPartialRecord1>) -> () {
    target.blend_mode = obj.as_ref().and_then(|value| (value.blend_mode).clone());
}
