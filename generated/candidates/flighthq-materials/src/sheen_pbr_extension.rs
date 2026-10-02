// @generated from upstream/packages/materials/src/sheenPbrExtension.ts; do not edit.
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
    SHEEN_PBR_EXTENSION_KIND as sheen_pbr_extension_kind_constant, SheenPbrExtension, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1860699151 {
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
impl PartialEq for FlightPartialRecord1860699151 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/sheenPbrExtension.ts:8 (sha256:efd2afea3fd997e167a682dec0a3d240c2b98faf4d62b10872c47eea082d9c50)
pub fn create_sheen_pbr_extension(
    opts: Option<FlightPartialRecord1860699151>,
) -> SheenPbrExtension {
    let mut out = allocate_entity();
    initialize_sheen_pbr_extension(
        (out).clone(),
        ((opts).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord1860699151 {
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

// Source: upstream/packages/materials/src/sheenPbrExtension.ts:14 (sha256:6e9bd4ee1161ebf3b0e8c1a9ef06c87ebad5f85ec81cf9773040794dcd672c01)
pub fn initialize_sheen_pbr_extension(
    out: EntityConstruction<SheenPbrExtension>,
    opts: Option<FlightPartialRecord1860699151>,
) -> () {
    crate::host_set("host.kind", sheen_pbr_extension_kind_constant);
    crate::host_set(
        "host.sheenColor",
        (opts.as_ref().and_then(|value| value.sheen_color)).unwrap_or(255.0_f64),
    );
    crate::host_set(
        "host.sheenColorMap",
        opts.as_ref()
            .and_then(|value| (value.sheen_color_map).clone()),
    );
    crate::host_set(
        "host.sheenColorMapUvSet",
        (opts.as_ref().and_then(|value| value.sheen_color_map_uv_set)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.sheenRoughness",
        (opts.as_ref().and_then(|value| value.sheen_roughness)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.sheenRoughnessMap",
        opts.as_ref()
            .and_then(|value| (value.sheen_roughness_map).clone()),
    );
    crate::host_set(
        "host.sheenRoughnessMapUvSet",
        (opts
            .as_ref()
            .and_then(|value| value.sheen_roughness_map_uv_set))
        .unwrap_or(0.0_f64),
    );
}

// Source: upstream/packages/materials/src/sheenPbrExtension.ts:27 (sha256:bcd21e33932a285777c418874a51c4a243732daa6a94ed290f3830656e68b057)
pub fn is_valid_sheen_pbr_extension(value: &SheenPbrExtension) -> bool {
    return ((is_valid_material_weight(value.sheen_roughness))
        && (is_valid_pbr_uv_set(value.sheen_color_map_uv_set)))
        && (is_valid_pbr_uv_set(value.sheen_roughness_map_uv_set));
}
