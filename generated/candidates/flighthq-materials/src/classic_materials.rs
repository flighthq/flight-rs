// @generated from upstream/packages/materials/src/classicMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_types::{
    BLINN_PHONG_MATERIAL_KIND as blinn_phong_material_kind_constant, BlendMode, BlinnPhongMaterial,
    Kind, LAMBERT_MATERIAL_KIND as lambert_material_kind_constant, LambertMaterial,
    MaterialAlphaMode, PHONG_MATERIAL_KIND as phong_material_kind_constant, PhongMaterial, Texture,
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
    pub alpha_map: Option<Texture>,
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub shininess: Option<f64>,
    pub specular: Option<f64>,
    pub specular_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
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
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
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
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub shininess: Option<f64>,
    pub specular: Option<f64>,
    pub specular_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/classicMaterials.ts:8 (sha256:7045d34755592ea9e13fccfcdad918a9931c1da687e3c570e52bbc69bbb40a7c)
pub fn create_blinn_phong_material(opts: Option<FlightPartialRecord1>) -> BlinnPhongMaterial {
    let mut material = create_surface_material(
        (blinn_phong_material_kind_constant).to_owned(),
        Some((((opts).clone()).clone().unwrap()).clone()),
    );
    material.alpha_map = opts.as_ref().and_then(|value| (value.alpha_map).clone());
    material.diffuse = (opts.as_ref().and_then(|value| value.diffuse))
        .clone()
        .unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale))
        .clone()
        .unwrap_or(1.0_f64);
    material.shininess = (opts.as_ref().and_then(|value| value.shininess))
        .clone()
        .unwrap_or(32.0_f64);
    material.specular = (opts.as_ref().and_then(|value| value.specular))
        .clone()
        .unwrap_or(4294967295.0_f64);
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/classicMaterials.ts:23 (sha256:d79c4aa376c8c331d20a3019e39f21fa01aa6a8ccdfbcd9990f7187bf35506b3)
pub fn create_lambert_material(opts: Option<FlightPartialRecord2>) -> LambertMaterial {
    let mut material = create_surface_material(
        (lambert_material_kind_constant).to_owned(),
        Some((((opts).clone()).clone().unwrap()).clone()),
    );
    material.diffuse = (opts.as_ref().and_then(|value| value.diffuse))
        .clone()
        .unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.emissive = (opts.as_ref().and_then(|value| value.emissive))
        .clone()
        .unwrap_or(255.0_f64);
    material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/classicMaterials.ts:34 (sha256:d18bc4d13e57839d2e8579c260c27e2adc809be5702c7b7f1def49cfc5801768)
pub fn create_phong_material(opts: Option<FlightPartialRecord3>) -> PhongMaterial {
    let mut material = create_surface_material(
        (phong_material_kind_constant).to_owned(),
        Some((((opts).clone()).clone().unwrap()).clone()),
    );
    material.diffuse = (opts.as_ref().and_then(|value| value.diffuse))
        .clone()
        .unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale))
        .clone()
        .unwrap_or(1.0_f64);
    material.shininess = (opts.as_ref().and_then(|value| value.shininess))
        .clone()
        .unwrap_or(32.0_f64);
    material.specular = (opts.as_ref().and_then(|value| value.specular))
        .clone()
        .unwrap_or(4294967295.0_f64);
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    return material;
}
