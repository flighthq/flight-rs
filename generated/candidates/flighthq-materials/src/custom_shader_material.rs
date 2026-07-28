// @generated from upstream/packages/materials/src/customShaderMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_types::{
    AlphaType, BlendMode, CUSTOM_SHADER_MATERIAL_KIND as custom_shader_material_kind_constant,
    CustomShaderMaterial, MaterialAlphaMode, Texture,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<String>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/customShaderMaterial.ts:11 (sha256:a643450772789ace081e49e752691665df01a1db56b29a85c016fed23fb5a3ad)
pub fn create_custom_shader_material(opts: Option<FlightPartialRecord1>) -> CustomShaderMaterial {
    let mut material = create_surface_material((custom_shader_material_kind_constant).to_owned());
    material.shader_key =
        (opts.as_ref().and_then(|value| (value.shader_key).clone())).unwrap_or("".to_owned());
    material.textures = opts.as_ref().and_then(|value| (value.textures).clone());
    material.uniforms = opts.as_ref().and_then(|value| (value.uniforms).clone());
    return material;
}
