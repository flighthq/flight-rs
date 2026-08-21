// @generated from upstream/packages/materials/src/anisotropyPbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{is_valid_material_weight, is_valid_pbr_uv_set};
use flighthq_entity::create_entity;
use flighthq_types::{
    ANISOTROPY_PBR_EXTENSION_KIND as anisotropy_pbr_extension_kind_constant,
    AnisotropyPbrExtension, Kind, PbrUvSet, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1898382904 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub thickness: Option<f64>,
    pub thickness_map: Option<Texture>,
    pub thickness_map_uv_set: Option<PbrUvSet>,
    pub wrapped_diffuse_color: Option<f64>,
    pub wrapped_diffuse_map: Option<Texture>,
    pub wrapped_diffuse_map_uv_set: Option<PbrUvSet>,
    pub wrapped_diffuse_strength: Option<f64>,
    pub attenuation_color: Option<f64>,
    pub attenuation_distance: Option<f64>,
    pub ior: Option<f64>,
    pub transmission: Option<f64>,
    pub transmission_map: Option<Texture>,
    pub transmission_map_uv_set: Option<PbrUvSet>,
    pub specular: Option<f64>,
    pub specular_color: Option<f64>,
    pub specular_color_map: Option<Texture>,
    pub specular_color_map_uv_set: Option<PbrUvSet>,
    pub specular_map: Option<Texture>,
    pub specular_map_uv_set: Option<PbrUvSet>,
    pub sheen_color: Option<f64>,
    pub sheen_color_map: Option<Texture>,
    pub sheen_color_map_uv_set: Option<PbrUvSet>,
    pub sheen_roughness: Option<f64>,
    pub sheen_roughness_map: Option<Texture>,
    pub sheen_roughness_map_uv_set: Option<PbrUvSet>,
    pub iridescence: Option<f64>,
    pub iridescence_ior: Option<f64>,
    pub iridescence_map: Option<Texture>,
    pub iridescence_map_uv_set: Option<PbrUvSet>,
    pub iridescence_thickness_map: Option<Texture>,
    pub iridescence_thickness_map_uv_set: Option<PbrUvSet>,
    pub iridescence_thickness_max: Option<f64>,
    pub iridescence_thickness_min: Option<f64>,
    pub clearcoat: Option<f64>,
    pub clearcoat_map: Option<Texture>,
    pub clearcoat_map_uv_set: Option<PbrUvSet>,
    pub clearcoat_normal_map: Option<Texture>,
    pub clearcoat_normal_map_uv_set: Option<PbrUvSet>,
    pub clearcoat_normal_scale: Option<f64>,
    pub clearcoat_roughness: Option<f64>,
    pub clearcoat_roughness_map: Option<Texture>,
    pub clearcoat_roughness_map_uv_set: Option<PbrUvSet>,
    pub anisotropy_map: Option<Texture>,
    pub anisotropy_map_uv_set: Option<PbrUvSet>,
    pub anisotropy_rotation: Option<f64>,
    pub anisotropy_strength: Option<f64>,
}
impl PartialEq for FlightPartialRecord1898382904 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/anisotropyPbrExtension.ts:8 (sha256:1ee39a09610fd71e205b47a9e49785a81224651e10b82169f682bff10d9cda50)
pub fn create_anisotropy_pbr_extension(
    opts: Option<FlightPartialRecord1898382904>,
) -> AnisotropyPbrExtension {
    return create_entity(Some(AnisotropyPbrExtension {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        anisotropy_map: opts
            .as_ref()
            .and_then(|value| (value.anisotropy_map).clone()),
        anisotropy_map_uv_set: (opts.as_ref().and_then(|value| value.anisotropy_map_uv_set))
            .clone()
            .unwrap_or(0.0_f64),
        anisotropy_rotation: (opts.as_ref().and_then(|value| value.anisotropy_rotation))
            .clone()
            .unwrap_or(0.0_f64),
        anisotropy_strength: (opts.as_ref().and_then(|value| value.anisotropy_strength))
            .clone()
            .unwrap_or(0.0_f64),
        kind: (anisotropy_pbr_extension_kind_constant).to_owned(),
        ..Default::default()
    }));
}

// Source: upstream/packages/materials/src/anisotropyPbrExtension.ts:18 (sha256:28eae29fa8dc434c61db6e7f3661cbbd3e4a5767c597f8131cd88cadc2faea53)
pub fn is_valid_anisotropy_pbr_extension(value: &AnisotropyPbrExtension) -> bool {
    return ((is_valid_material_weight(value.anisotropy_strength))
        && ((value.anisotropy_rotation).is_finite()))
        && (is_valid_pbr_uv_set(value.anisotropy_map_uv_set));
}
