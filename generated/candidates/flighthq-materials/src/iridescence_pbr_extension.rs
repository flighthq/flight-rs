// @generated from upstream/packages/materials/src/iridescencePbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    is_valid_material_ior, is_valid_material_iridescence_thickness, is_valid_material_weight,
    is_valid_pbr_uv_set,
};
use flighthq_entity::create_entity;
use flighthq_types::{
    IRIDESCENCE_PBR_EXTENSION_KIND as iridescence_pbr_extension_kind_constant,
    IridescencePbrExtension, Kind, PbrUvSet, Texture,
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

// Source: upstream/packages/materials/src/iridescencePbrExtension.ts:8 (sha256:be7030f971eb646d109da28ed9f9061ad512f460ab8e5f05a852370023912abc)
pub fn create_iridescence_pbr_extension(
    opts: Option<FlightPartialRecord1>,
) -> IridescencePbrExtension {
    return create_entity(Some(IridescencePbrExtension {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        iridescence: (opts.as_ref().and_then(|value| value.iridescence)).unwrap_or(0.0_f64),
        iridescence_ior: (opts.as_ref().and_then(|value| value.iridescence_ior)).unwrap_or(1.3_f64),
        iridescence_map: opts
            .as_ref()
            .and_then(|value| (value.iridescence_map).clone()),
        iridescence_map_uv_set: (opts.as_ref().and_then(|value| value.iridescence_map_uv_set))
            .unwrap_or(0.0_f64),
        iridescence_thickness_map: opts
            .as_ref()
            .and_then(|value| (value.iridescence_thickness_map).clone()),
        iridescence_thickness_map_uv_set: (opts
            .as_ref()
            .and_then(|value| value.iridescence_thickness_map_uv_set))
        .unwrap_or(0.0_f64),
        iridescence_thickness_max: (opts
            .as_ref()
            .and_then(|value| value.iridescence_thickness_max))
        .unwrap_or(400.0_f64),
        iridescence_thickness_min: (opts
            .as_ref()
            .and_then(|value| value.iridescence_thickness_min))
        .unwrap_or(100.0_f64),
        kind: (iridescence_pbr_extension_kind_constant).to_owned(),
        ..Default::default()
    }));
}

// Source: upstream/packages/materials/src/iridescencePbrExtension.ts:24 (sha256:4e4ec088fde2d6960f981a7a25cddbf0e6104f682978ccc1da2063cda41b1ab7)
pub fn is_valid_iridescence_pbr_extension(value: &IridescencePbrExtension) -> bool {
    return ((((((is_valid_material_weight(value.iridescence))
        && (is_valid_material_ior(value.iridescence_ior)))
        && (is_valid_material_iridescence_thickness(value.iridescence_thickness_min)))
        && (is_valid_material_iridescence_thickness(value.iridescence_thickness_max)))
        && (value.iridescence_thickness_min <= value.iridescence_thickness_max))
        && (is_valid_pbr_uv_set(value.iridescence_map_uv_set)))
        && (is_valid_pbr_uv_set(value.iridescence_thickness_map_uv_set));
}
