// @generated from upstream/packages/materials/src/anisotropyPbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{is_valid_material_weight, is_valid_pbr_uv_set};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    ANISOTROPY_PBR_EXTENSION_KIND as anisotropy_pbr_extension_kind_constant,
    AnisotropyPbrExtension, EntityConstruction, Kind, PbrUvSet, Texture,
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

// Source: upstream/packages/materials/src/anisotropyPbrExtension.ts:8 (sha256:6469d0ca006ff43420f787ccb75271f2a08326a7ade8d584be23fe92e70291de)
pub fn create_anisotropy_pbr_extension(
    opts: Option<FlightPartialRecord1898382904>,
) -> AnisotropyPbrExtension {
    let mut out = allocate_entity();
    initialize_anisotropy_pbr_extension(
        (out).clone(),
        ((opts).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord1898382904 {
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

// Source: upstream/packages/materials/src/anisotropyPbrExtension.ts:14 (sha256:54624e2c553cea101c9ae1de5fc4c30eae2e076d8b6fe28e67a9458c7f84ae4c)
pub fn initialize_anisotropy_pbr_extension(
    out: EntityConstruction<AnisotropyPbrExtension>,
    opts: Option<FlightPartialRecord1898382904>,
) -> () {
    crate::host_set(
        "host.anisotropyMap",
        opts.as_ref()
            .and_then(|value| (value.anisotropy_map).clone()),
    );
    crate::host_set(
        "host.anisotropyMapUvSet",
        (opts.as_ref().and_then(|value| value.anisotropy_map_uv_set)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.anisotropyRotation",
        (opts.as_ref().and_then(|value| value.anisotropy_rotation)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.anisotropyStrength",
        (opts.as_ref().and_then(|value| value.anisotropy_strength)).unwrap_or(0.0_f64),
    );
    crate::host_set("host.kind", anisotropy_pbr_extension_kind_constant);
}

// Source: upstream/packages/materials/src/anisotropyPbrExtension.ts:25 (sha256:28eae29fa8dc434c61db6e7f3661cbbd3e4a5767c597f8131cd88cadc2faea53)
pub fn is_valid_anisotropy_pbr_extension(value: &AnisotropyPbrExtension) -> bool {
    return ((is_valid_material_weight(value.anisotropy_strength))
        && ((value.anisotropy_rotation).is_finite()))
        && (is_valid_pbr_uv_set(value.anisotropy_map_uv_set));
}
