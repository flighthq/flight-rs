// @generated from upstream/packages/materials/src/specularPbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{is_valid_material_weight, is_valid_pbr_uv_set};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    EntityConstruction, Kind, PbrUvSet,
    SPECULAR_PBR_EXTENSION_KIND as specular_pbr_extension_kind_constant, SpecularPbrExtension,
    Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord3736316967 {
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
impl PartialEq for FlightPartialRecord3736316967 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/specularPbrExtension.ts:8 (sha256:2dbdb8cc68b37cf52ea1aa554cedbdb77e7b239639c0915235a3f10efbce8586)
pub fn create_specular_pbr_extension(
    opts: Option<FlightPartialRecord3736316967>,
) -> SpecularPbrExtension {
    let mut out = allocate_entity();
    initialize_specular_pbr_extension(
        (out).clone(),
        ((opts).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord3736316967 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                kind: (__flight_source.kind).clone(),
                thickness: __flight_source.thickness,
                thickness_map: (__flight_source.thickness_map).clone(),
                thickness_map_uv_set: __flight_source.thickness_map_uv_set,
                wrapped_diffuse_color: __flight_source.wrapped_diffuse_color,
                wrapped_diffuse_map: (__flight_source.wrapped_diffuse_map).clone(),
                wrapped_diffuse_map_uv_set: __flight_source.wrapped_diffuse_map_uv_set,
                wrapped_diffuse_strength: __flight_source.wrapped_diffuse_strength,
                attenuation_color: __flight_source.attenuation_color,
                attenuation_distance: __flight_source.attenuation_distance,
                ior: __flight_source.ior,
                transmission: __flight_source.transmission,
                transmission_map: (__flight_source.transmission_map).clone(),
                transmission_map_uv_set: __flight_source.transmission_map_uv_set,
                specular: __flight_source.specular,
                specular_color: __flight_source.specular_color,
                specular_color_map: (__flight_source.specular_color_map).clone(),
                specular_color_map_uv_set: __flight_source.specular_color_map_uv_set,
                specular_map: (__flight_source.specular_map).clone(),
                specular_map_uv_set: __flight_source.specular_map_uv_set,
                sheen_color: __flight_source.sheen_color,
                sheen_color_map: (__flight_source.sheen_color_map).clone(),
                sheen_color_map_uv_set: __flight_source.sheen_color_map_uv_set,
                sheen_roughness: __flight_source.sheen_roughness,
                sheen_roughness_map: (__flight_source.sheen_roughness_map).clone(),
                sheen_roughness_map_uv_set: __flight_source.sheen_roughness_map_uv_set,
                iridescence: __flight_source.iridescence,
                iridescence_ior: __flight_source.iridescence_ior,
                iridescence_map: (__flight_source.iridescence_map).clone(),
                iridescence_map_uv_set: __flight_source.iridescence_map_uv_set,
                iridescence_thickness_map: (__flight_source.iridescence_thickness_map).clone(),
                iridescence_thickness_map_uv_set: __flight_source.iridescence_thickness_map_uv_set,
                iridescence_thickness_max: __flight_source.iridescence_thickness_max,
                iridescence_thickness_min: __flight_source.iridescence_thickness_min,
                clearcoat: __flight_source.clearcoat,
                clearcoat_map: (__flight_source.clearcoat_map).clone(),
                clearcoat_map_uv_set: __flight_source.clearcoat_map_uv_set,
                clearcoat_normal_map: (__flight_source.clearcoat_normal_map).clone(),
                clearcoat_normal_map_uv_set: __flight_source.clearcoat_normal_map_uv_set,
                clearcoat_normal_scale: __flight_source.clearcoat_normal_scale,
                clearcoat_roughness: __flight_source.clearcoat_roughness,
                clearcoat_roughness_map: (__flight_source.clearcoat_roughness_map).clone(),
                clearcoat_roughness_map_uv_set: __flight_source.clearcoat_roughness_map_uv_set,
                anisotropy_map: (__flight_source.anisotropy_map).clone(),
                anisotropy_map_uv_set: __flight_source.anisotropy_map_uv_set,
                anisotropy_rotation: __flight_source.anisotropy_rotation,
                anisotropy_strength: __flight_source.anisotropy_strength,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/materials/src/specularPbrExtension.ts:14 (sha256:ed7fb28968b19dd4716b2b59f326941dc2b07b6833be543269a76fc49b01e04d)
pub fn initialize_specular_pbr_extension(
    out: EntityConstruction<SpecularPbrExtension>,
    opts: Option<FlightPartialRecord3736316967>,
) -> () {
    crate::host_set("host.kind", specular_pbr_extension_kind_constant);
    crate::host_set(
        "host.specular",
        (opts.as_ref().and_then(|value| value.specular)).unwrap_or(1.0_f64),
    );
    crate::host_set(
        "host.specularColor",
        (opts.as_ref().and_then(|value| value.specular_color)).unwrap_or(4294967295.0_f64),
    );
    crate::host_set(
        "host.specularColorMap",
        opts.as_ref()
            .and_then(|value| (value.specular_color_map).clone()),
    );
    crate::host_set(
        "host.specularColorMapUvSet",
        (opts
            .as_ref()
            .and_then(|value| value.specular_color_map_uv_set))
        .unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.specularMap",
        opts.as_ref().and_then(|value| (value.specular_map).clone()),
    );
    crate::host_set(
        "host.specularMapUvSet",
        (opts.as_ref().and_then(|value| value.specular_map_uv_set)).unwrap_or(0.0_f64),
    );
}

// Source: upstream/packages/materials/src/specularPbrExtension.ts:27 (sha256:443dc7f521a64f1ce73e92e2f162087a252e5e7fcc2441ee83f14608888c5ea0)
pub fn is_valid_specular_pbr_extension(value: &SpecularPbrExtension) -> bool {
    return ((is_valid_material_weight(value.specular))
        && (is_valid_pbr_uv_set(value.specular_color_map_uv_set)))
        && (is_valid_pbr_uv_set(value.specular_map_uv_set));
}
