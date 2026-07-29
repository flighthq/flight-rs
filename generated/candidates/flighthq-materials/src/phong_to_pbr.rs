// @generated from upstream/packages/materials/src/phongToPbr.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_standard_pbr_material;
use flighthq_color::get_color_luminance;
use flighthq_types::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, PhongMaterial, StandardPbrMaterial, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: Option<f64>,
    pub metallic: Option<f64>,
    pub metallic_roughness_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: Option<f64>,
    pub roughness: Option<f64>,
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
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: Option<f64>,
    pub glossiness: Option<f64>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: Option<f64>,
    pub specular: Option<f64>,
    pub specular_glossiness_map: Option<Texture>,
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
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: Option<f64>,
    pub metallic: Option<f64>,
    pub metallic_roughness_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: Option<f64>,
    pub roughness: Option<f64>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/phongToPbr.ts:17 (sha256:4f0ee09a628abe5b3f48687b2bbca15325445116f4b5e22318de1aa8cadd0c92)
pub fn convert_phong_to_standard_pbr_material(
    phong: &PhongMaterial,
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord3 {
        base_color: Some(phong.diffuse),
        base_color_map: (phong.diffuse_map).clone(),
        metallic: Some(get_pbr_metallic_from_phong_specular(
            phong.specular,
            phong.diffuse,
        )),
        normal_map: (phong.normal_map).clone(),
        normal_scale: Some(phong.normal_scale),
        roughness: Some(get_pbr_roughness_from_phong_shininess(phong.shininess)),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/phongToPbr.ts:36 (sha256:df6462ac13f10902000aa41bf7bb6d3963e14e4ae5c8aa50116192de3b47d264)
pub fn get_pbr_metallic_from_phong_specular(specular: f64, diffuse: f64) -> f64 {
    return if (get_color_luminance(specular) > 0.5_f64) && (get_color_luminance(diffuse) < 0.04_f64)
    {
        1.0_f64
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/materials/src/phongToPbr.ts:43 (sha256:522973c86a96c5b585de450437f9095e2be72bee298053f386222ab3a0b477e1)
pub fn get_pbr_roughness_from_phong_shininess(shininess: f64) -> f64 {
    return (1.0_f64).min((0.0_f64).max((2.0_f64 / ((0.0_f64).max(shininess) + 2.0_f64)).sqrt()));
}

// Source: upstream/packages/materials/src/phongToPbr.ts:59 (sha256:9d2d27b5f98df85910b1947d7df678b517e71163a596782fedeec122107c656e)
pub fn get_phong_to_pbr_light_exposure() -> f64 {
    return (std::f64::consts::PI).log2();
}
