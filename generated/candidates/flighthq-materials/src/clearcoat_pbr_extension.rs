// @generated from upstream/packages/materials/src/clearcoatPbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{is_valid_material_weight, is_valid_pbr_uv_set};
use flighthq_entity::create_entity;
use flighthq_types::{
    CLEARCOAT_PBR_EXTENSION_KIND as clearcoat_pbr_extension_kind_constant, ClearcoatPbrExtension,
    Kind, PbrUvSet, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1825521990 {
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
impl PartialEq for FlightPartialRecord1825521990 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/clearcoatPbrExtension.ts:8 (sha256:d237062b2ff6d794eb60ec116d9e4b98dd7c6c78022c87a6af00893f3ffcf9c3)
pub fn create_clearcoat_pbr_extension(
    opts: Option<FlightPartialRecord1825521990>,
) -> ClearcoatPbrExtension {
    return create_entity(Some(ClearcoatPbrExtension {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        clearcoat: (opts.as_ref().and_then(|value| value.clearcoat))
            .clone()
            .unwrap_or(0.0_f64),
        clearcoat_map: opts
            .as_ref()
            .and_then(|value| (value.clearcoat_map).clone()),
        clearcoat_map_uv_set: (opts.as_ref().and_then(|value| value.clearcoat_map_uv_set))
            .clone()
            .unwrap_or(0.0_f64),
        clearcoat_normal_map: opts
            .as_ref()
            .and_then(|value| (value.clearcoat_normal_map).clone()),
        clearcoat_normal_map_uv_set: (opts
            .as_ref()
            .and_then(|value| value.clearcoat_normal_map_uv_set))
        .clone()
        .unwrap_or(0.0_f64),
        clearcoat_normal_scale: (opts.as_ref().and_then(|value| value.clearcoat_normal_scale))
            .clone()
            .unwrap_or(1.0_f64),
        clearcoat_roughness: (opts.as_ref().and_then(|value| value.clearcoat_roughness))
            .clone()
            .unwrap_or(0.0_f64),
        clearcoat_roughness_map: opts
            .as_ref()
            .and_then(|value| (value.clearcoat_roughness_map).clone()),
        clearcoat_roughness_map_uv_set: (opts
            .as_ref()
            .and_then(|value| value.clearcoat_roughness_map_uv_set))
        .clone()
        .unwrap_or(0.0_f64),
        kind: (clearcoat_pbr_extension_kind_constant).to_owned(),
        ..Default::default()
    }));
}

// Source: upstream/packages/materials/src/clearcoatPbrExtension.ts:23 (sha256:637edc77823a849aaedefa8fefe16a63cccc98b81104ff7ff3a5aed969678133)
pub fn is_valid_clearcoat_pbr_extension(value: &ClearcoatPbrExtension) -> bool {
    return ((((((is_valid_material_weight(value.clearcoat))
        && (is_valid_material_weight(value.clearcoat_roughness)))
        && ((value.clearcoat_normal_scale).is_finite()))
        && (value.clearcoat_normal_scale >= 0.0_f64))
        && (is_valid_pbr_uv_set(value.clearcoat_map_uv_set)))
        && (is_valid_pbr_uv_set(value.clearcoat_normal_map_uv_set)))
        && (is_valid_pbr_uv_set(value.clearcoat_roughness_map_uv_set));
}
