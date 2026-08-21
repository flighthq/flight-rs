// @generated from upstream/packages/materials/src/standardMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    BlendMode, Kind, MaterialAlphaMode, PbrExtension,
    STANDARD_MATERIAL_KIND as standard_material_kind_constant, StandardMaterial,
    StandardPbrMaterialProperties, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord118067091 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub extensions: Option<Vec<PbrExtension>>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for FlightPartialRecord118067091 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/standardMaterial.ts:5 (sha256:ec8e2881e1a5917f8d69a6499cefe084e1bc866899af7ba435d1be7519a2f40b)
pub fn create_standard_material(options: Option<FlightPartialRecord118067091>) -> StandardMaterial {
    return create_entity(Some(StandardMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        kind: (standard_material_kind_constant).to_owned(),
        name: options.as_ref().and_then(|value| (value.name).clone()),
        ..Default::default()
    }));
}
