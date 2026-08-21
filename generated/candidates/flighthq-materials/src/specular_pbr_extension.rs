// @generated from upstream/packages/materials/src/specularPbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{is_valid_material_weight, is_valid_pbr_uv_set};
use flighthq_entity::create_entity;
use flighthq_types::{
    Kind, PbrUvSet, SPECULAR_PBR_EXTENSION_KIND as specular_pbr_extension_kind_constant,
    SpecularPbrExtension, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/specularPbrExtension.ts:8 (sha256:db879f1b45905bbd3f59d72dfe2e11a91ead6f87d0c07692336c5d17fdcd358c)
pub fn create_specular_pbr_extension(opts: Option<FlightPartialRecord1>) -> SpecularPbrExtension {
    return create_entity(Some(SpecularPbrExtension {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        kind: (specular_pbr_extension_kind_constant).to_owned(),
        specular: (opts.as_ref().and_then(|value| value.specular))
            .clone()
            .unwrap_or(1.0_f64),
        specular_color: (opts.as_ref().and_then(|value| value.specular_color))
            .clone()
            .unwrap_or(4294967295.0_f64),
        specular_color_map: opts
            .as_ref()
            .and_then(|value| (value.specular_color_map).clone()),
        specular_color_map_uv_set: (opts
            .as_ref()
            .and_then(|value| value.specular_color_map_uv_set))
        .clone()
        .unwrap_or(0.0_f64),
        specular_map: opts.as_ref().and_then(|value| (value.specular_map).clone()),
        specular_map_uv_set: (opts.as_ref().and_then(|value| value.specular_map_uv_set))
            .clone()
            .unwrap_or(0.0_f64),
        ..Default::default()
    }));
}

// Source: upstream/packages/materials/src/specularPbrExtension.ts:20 (sha256:443dc7f521a64f1ce73e92e2f162087a252e5e7fcc2441ee83f14608888c5ea0)
pub fn is_valid_specular_pbr_extension(value: &SpecularPbrExtension) -> bool {
    return ((is_valid_material_weight(value.specular))
        && (is_valid_pbr_uv_set(value.specular_color_map_uv_set)))
        && (is_valid_pbr_uv_set(value.specular_map_uv_set));
}
