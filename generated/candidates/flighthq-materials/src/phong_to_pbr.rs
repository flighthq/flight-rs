// @generated from upstream/packages/materials/src/phongToPbr.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_standard_pbr_material;
use flighthq_color::get_color_luminance;
use flighthq_types::{PhongMaterial, StandardPbrMaterial, Texture};

#[derive(Clone, Default)]
pub struct FlightPartialRecord3905749610 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_map: Option<Texture>,
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
impl PartialEq for FlightPartialRecord3905749610 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/phongToPbr.ts:17 (sha256:4f0ee09a628abe5b3f48687b2bbca15325445116f4b5e22318de1aa8cadd0c92)
pub fn convert_phong_to_standard_pbr_material(
    phong: &PhongMaterial,
    opts: Option<FlightPartialRecord3905749610>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_6 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: None,
            name: None,
            alpha_cutoff: None,
            alpha_mode: None,
            blend_mode: None,
            double_sided: None,
            extensions: None,
            standard: None,
            shader_key: None,
            textures: None,
            uniforms: None,
            alpha_map: (__flight_spread_6.alpha_map).clone(),
            base_color: __flight_spread_6.base_color,
            base_color_map: (__flight_spread_6.base_color_map).clone(),
            emissive: __flight_spread_6.emissive,
            emissive_map: (__flight_spread_6.emissive_map).clone(),
            emissive_strength: __flight_spread_6.emissive_strength,
            metallic: __flight_spread_6.metallic,
            metallic_roughness_map: (__flight_spread_6.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_6.normal_map).clone(),
            normal_scale: __flight_spread_6.normal_scale,
            occlusion_map: (__flight_spread_6.occlusion_map).clone(),
            occlusion_strength: __flight_spread_6.occlusion_strength,
            roughness: __flight_spread_6.roughness,
        }
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
