// @generated from upstream/packages/node/src/hasMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{HasMaterial, Material, MaterialData};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/node/src/hasMaterial.ts:3 (sha256:d75f17511c1924a6c6bf6be032ebea903b539055bb8a7e4e4b29684c0850a7f6)
pub fn init_material_trait(target: &mut HasMaterial, obj: Option<FlightPartialRecord1>) -> () {
    target.material = obj.as_ref().and_then(|value| (value.material).clone());
    target.material_data = obj.as_ref().and_then(|value| (value.material_data).clone());
}
