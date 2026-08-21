// @generated from upstream/packages/materials/src/transmissionVolumePbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{is_valid_material_ior, is_valid_material_weight, is_valid_pbr_uv_set};
use flighthq_entity::create_entity;
use flighthq_types::{
    Kind, PbrUvSet,
    TRANSMISSION_VOLUME_PBR_EXTENSION_KIND as transmission_volume_pbr_extension_kind_constant,
    Texture, TransmissionVolumePbrExtension,
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

// Source: upstream/packages/materials/src/transmissionVolumePbrExtension.ts:8 (sha256:29c4c367f9c43e3f766ccd805cb3b5ecc12bf73f22aa0d0419dde501c76be9f8)
pub fn create_transmission_volume_pbr_extension(
    opts: Option<FlightPartialRecord1>,
) -> TransmissionVolumePbrExtension {
    return create_entity(Some(TransmissionVolumePbrExtension {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        attenuation_color: (opts.as_ref().and_then(|value| value.attenuation_color))
            .clone()
            .unwrap_or(4294967295.0_f64),
        attenuation_distance: (opts.as_ref().and_then(|value| value.attenuation_distance))
            .clone()
            .unwrap_or(f64::INFINITY),
        ior: (opts.as_ref().and_then(|value| value.ior))
            .clone()
            .unwrap_or(1.5_f64),
        kind: (transmission_volume_pbr_extension_kind_constant).to_owned(),
        thickness: (opts.as_ref().and_then(|value| value.thickness))
            .clone()
            .unwrap_or(0.0_f64),
        thickness_map: opts
            .as_ref()
            .and_then(|value| (value.thickness_map).clone()),
        thickness_map_uv_set: (opts.as_ref().and_then(|value| value.thickness_map_uv_set))
            .clone()
            .unwrap_or(0.0_f64),
        transmission: (opts.as_ref().and_then(|value| value.transmission))
            .clone()
            .unwrap_or(0.0_f64),
        transmission_map: opts
            .as_ref()
            .and_then(|value| (value.transmission_map).clone()),
        transmission_map_uv_set: (opts
            .as_ref()
            .and_then(|value| value.transmission_map_uv_set))
        .clone()
        .unwrap_or(0.0_f64),
        ..Default::default()
    }));
}

// Source: upstream/packages/materials/src/transmissionVolumePbrExtension.ts:25 (sha256:92573a315acb28e21ca4aa9d8d77d78efe1164ffc0508f2999d332de72230f00)
pub fn is_valid_transmission_volume_pbr_extension(value: &TransmissionVolumePbrExtension) -> bool {
    let valid_attenuation_distance = (value.attenuation_distance == f64::INFINITY)
        || (((value.attenuation_distance).is_finite()) && (value.attenuation_distance > 0.0_f64));
    return ((((((is_valid_material_weight(value.transmission))
        && (is_valid_material_ior(value.ior)))
        && ((value.thickness).is_finite()))
        && (value.thickness >= 0.0_f64))
        && (valid_attenuation_distance))
        && (is_valid_pbr_uv_set(value.thickness_map_uv_set)))
        && (is_valid_pbr_uv_set(value.transmission_map_uv_set));
}
