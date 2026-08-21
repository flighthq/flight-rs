// @generated from upstream/packages/materials/src/customShaderMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_types::{
    BlendMode, CUSTOM_SHADER_MATERIAL_KIND as custom_shader_material_kind_constant,
    CustomShaderMaterial, Kind, MaterialAlphaMode, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/customShaderMaterial.ts:10 (sha256:3374da3795413d8a2eb1a87d9b4c61fa7b6076e9c136ad053adcb0500ff2fb88)
pub fn create_custom_shader_material(opts: Option<FlightPartialRecord1>) -> CustomShaderMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (custom_shader_material_kind_constant).to_owned(),
            Some((((opts).clone()).clone().unwrap()).clone()),
        ));
        CustomShaderMaterial {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            alpha_cutoff: __flight_source.alpha_cutoff,
            alpha_mode: (__flight_source.alpha_mode).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            ..Default::default()
        }
    };
    material.shader_key = (opts.as_ref().and_then(|value| (value.shader_key).clone()))
        .clone()
        .unwrap_or("".to_owned());
    material.textures = opts.as_ref().and_then(|value| (value.textures).clone());
    material.uniforms = opts.as_ref().and_then(|value| (value.uniforms).clone());
    return material;
}
