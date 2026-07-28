// @generated from upstream/packages/node/src/hasAppearance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::HasAppearance;

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/node/src/hasAppearance.ts:3 (sha256:78f77813f37884625f00483faf457470c5d24a544ad1a3cee65fd8aea28840b5)
pub fn init_appearance_trait(target: &mut HasAppearance, obj: Option<FlightPartialRecord1>) -> () {
    target.alpha = (obj.as_ref().and_then(|value| value.alpha)).unwrap_or(1.0_f64);
    target.visible = (obj.as_ref().and_then(|value| value.visible)).unwrap_or(true);
}
