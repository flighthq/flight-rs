// @generated from upstream/packages/materials/src/sheenPbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{is_valid_material_weight, is_valid_pbr_uv_set};
use flighthq_entity::create_entity;
use flighthq_types::{
    Kind, PbrUvSet, SHEEN_PBR_EXTENSION_KIND as sheen_pbr_extension_kind_constant,
    SheenPbrExtension, Texture,
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

// Source: upstream/packages/materials/src/sheenPbrExtension.ts:8 (sha256:bad19531bcb91e470292c790b19a1b294e8d7fb3b615d82a6fd3635ee7d93a45)
pub fn create_sheen_pbr_extension(opts: Option<FlightPartialRecord1>) -> SheenPbrExtension {
    return create_entity(Some(SheenPbrExtension {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        kind: (sheen_pbr_extension_kind_constant).to_owned(),
        sheen_color: (opts.as_ref().and_then(|value| value.sheen_color))
            .clone()
            .unwrap_or(255.0_f64),
        sheen_color_map: opts
            .as_ref()
            .and_then(|value| (value.sheen_color_map).clone()),
        sheen_color_map_uv_set: (opts.as_ref().and_then(|value| value.sheen_color_map_uv_set))
            .clone()
            .unwrap_or(0.0_f64),
        sheen_roughness: (opts.as_ref().and_then(|value| value.sheen_roughness))
            .clone()
            .unwrap_or(0.0_f64),
        sheen_roughness_map: opts
            .as_ref()
            .and_then(|value| (value.sheen_roughness_map).clone()),
        sheen_roughness_map_uv_set: (opts
            .as_ref()
            .and_then(|value| value.sheen_roughness_map_uv_set))
        .clone()
        .unwrap_or(0.0_f64),
        ..Default::default()
    }));
}

// Source: upstream/packages/materials/src/sheenPbrExtension.ts:20 (sha256:bcd21e33932a285777c418874a51c4a243732daa6a94ed290f3830656e68b057)
pub fn is_valid_sheen_pbr_extension(value: &SheenPbrExtension) -> bool {
    return ((is_valid_material_weight(value.sheen_roughness))
        && (is_valid_pbr_uv_set(value.sheen_color_map_uv_set)))
        && (is_valid_pbr_uv_set(value.sheen_roughness_map_uv_set));
}
