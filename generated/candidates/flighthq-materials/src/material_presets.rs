// @generated from upstream/packages/materials/src/materialPresets.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    create_extended_pbr_material, create_standard_pbr_material,
    create_standard_pbr_material_properties, create_transmission_volume_pbr_extension,
};
use flighthq_types::{
    ExtendedPbrMaterial, GlassExtendedPbrMaterialOptions, PbrExtension, StandardPbrMaterial,
};

// Source: upstream/packages/materials/src/materialPresets.ts:20 (sha256:37bf6a37ffd0849714ad8499eb0e6b52ef5b1c6a8f276385256df8c490589610)
pub fn create_aluminum_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:31 (sha256:8fad8dc882e56f36710e1f987034293853e63b686144e0d61ab7b4efb621c0ff)
pub fn create_carbon_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:42 (sha256:7537cd465e39eea15930c2f626aeb82b57541e4a5b8f693773edd3cf64f4c081)
pub fn create_glass_extended_pbr_material(
    opts: Option<GlassExtendedPbrMaterialOptions>,
) -> ExtendedPbrMaterial {
    return create_extended_pbr_material(Some({
        let __flight_spread_0 = ((opts).clone()).unwrap_or_default();
        crate::extended_pbr_material::FlightPartialRecord3120887473 {
            __flight_identity: std::sync::Arc::new(()),
            kind: None,
            name: None,
            alpha_cutoff: __flight_spread_0.alpha_cutoff,
            alpha_mode: (__flight_spread_0.alpha_mode).clone(),
            blend_mode: (__flight_spread_0.blend_mode).clone(),
            double_sided: __flight_spread_0.double_sided,
            extensions: Some(vec![{
                let __flight_source = &(create_transmission_volume_pbr_extension(Some({
                    let __flight_spread_2 = (opts
                        .as_ref()
                        .and_then(|value| (value.transmission_volume).clone()))
                    .unwrap_or_default();
                    crate::transmission_volume_pbr_extension::FlightPartialRecord2066421274 {
                        __flight_identity: std::sync::Arc::new(()),
                        kind: (__flight_spread_2.kind).clone(),
                        thickness: __flight_spread_2.thickness,
                        thickness_map: (__flight_spread_2.thickness_map).clone(),
                        thickness_map_uv_set: __flight_spread_2.thickness_map_uv_set,
                        wrapped_diffuse_color: __flight_spread_2.wrapped_diffuse_color,
                        wrapped_diffuse_map: (__flight_spread_2.wrapped_diffuse_map).clone(),
                        wrapped_diffuse_map_uv_set: __flight_spread_2.wrapped_diffuse_map_uv_set,
                        wrapped_diffuse_strength: __flight_spread_2.wrapped_diffuse_strength,
                        attenuation_color: __flight_spread_2.attenuation_color,
                        attenuation_distance: __flight_spread_2.attenuation_distance,
                        ior: __flight_spread_2.ior,
                        transmission: __flight_spread_2.transmission,
                        transmission_map: (__flight_spread_2.transmission_map).clone(),
                        transmission_map_uv_set: __flight_spread_2.transmission_map_uv_set,
                        specular: __flight_spread_2.specular,
                        specular_color: __flight_spread_2.specular_color,
                        specular_color_map: (__flight_spread_2.specular_color_map).clone(),
                        specular_color_map_uv_set: __flight_spread_2.specular_color_map_uv_set,
                        specular_map: (__flight_spread_2.specular_map).clone(),
                        specular_map_uv_set: __flight_spread_2.specular_map_uv_set,
                        sheen_color: __flight_spread_2.sheen_color,
                        sheen_color_map: (__flight_spread_2.sheen_color_map).clone(),
                        sheen_color_map_uv_set: __flight_spread_2.sheen_color_map_uv_set,
                        sheen_roughness: __flight_spread_2.sheen_roughness,
                        sheen_roughness_map: (__flight_spread_2.sheen_roughness_map).clone(),
                        sheen_roughness_map_uv_set: __flight_spread_2.sheen_roughness_map_uv_set,
                        iridescence: __flight_spread_2.iridescence,
                        iridescence_ior: __flight_spread_2.iridescence_ior,
                        iridescence_map: (__flight_spread_2.iridescence_map).clone(),
                        iridescence_map_uv_set: __flight_spread_2.iridescence_map_uv_set,
                        iridescence_thickness_map: (__flight_spread_2.iridescence_thickness_map)
                            .clone(),
                        iridescence_thickness_map_uv_set: __flight_spread_2
                            .iridescence_thickness_map_uv_set,
                        iridescence_thickness_max: __flight_spread_2.iridescence_thickness_max,
                        iridescence_thickness_min: __flight_spread_2.iridescence_thickness_min,
                        clearcoat: __flight_spread_2.clearcoat,
                        clearcoat_map: (__flight_spread_2.clearcoat_map).clone(),
                        clearcoat_map_uv_set: __flight_spread_2.clearcoat_map_uv_set,
                        clearcoat_normal_map: (__flight_spread_2.clearcoat_normal_map).clone(),
                        clearcoat_normal_map_uv_set: __flight_spread_2.clearcoat_normal_map_uv_set,
                        clearcoat_normal_scale: __flight_spread_2.clearcoat_normal_scale,
                        clearcoat_roughness: __flight_spread_2.clearcoat_roughness,
                        clearcoat_roughness_map: (__flight_spread_2.clearcoat_roughness_map)
                            .clone(),
                        clearcoat_roughness_map_uv_set: __flight_spread_2
                            .clearcoat_roughness_map_uv_set,
                        anisotropy_map: (__flight_spread_2.anisotropy_map).clone(),
                        anisotropy_map_uv_set: __flight_spread_2.anisotropy_map_uv_set,
                        anisotropy_rotation: __flight_spread_2.anisotropy_rotation,
                        anisotropy_strength: __flight_spread_2.anisotropy_strength,
                    }
                })));
                PbrExtension {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    __flight_entity_snapshot: __flight_source
                        .__flight_entity_snapshot
                        .clone()
                        .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
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
                    iridescence_thickness_map_uv_set: __flight_source
                        .iridescence_thickness_map_uv_set,
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
                    ..Default::default()
                }
            }]),
            standard: Some(create_standard_pbr_material_properties(Some({
                let __flight_spread_3 =
                    (opts.as_ref().and_then(|value| (value.standard).clone())).unwrap_or_default();
                crate::pbr_materials::FlightPartialRecord3905749610 {
                    __flight_identity: std::sync::Arc::new(()),
                    alpha_map: (__flight_spread_3.alpha_map).clone(),
                    base_color: __flight_spread_3.base_color,
                    base_color_map: (__flight_spread_3.base_color_map).clone(),
                    emissive: __flight_spread_3.emissive,
                    emissive_map: (__flight_spread_3.emissive_map).clone(),
                    emissive_strength: __flight_spread_3.emissive_strength,
                    metallic: __flight_spread_3.metallic,
                    metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
                    normal_map: (__flight_spread_3.normal_map).clone(),
                    normal_scale: __flight_spread_3.normal_scale,
                    occlusion_map: (__flight_spread_3.occlusion_map).clone(),
                    occlusion_strength: __flight_spread_3.occlusion_strength,
                    roughness: __flight_spread_3.roughness,
                }
            }))),
            shader_key: None,
            textures: None,
            uniforms: None,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:63 (sha256:d98f9f1d62d29f41e2d5d19de7b0cb28609ad353bb45184ca449a58d7af5d370)
pub fn create_gold_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:74 (sha256:107a10d2a6ef94f08211dd15c88d8ce086e4f396393dea1ada79f9a53dbd4139)
pub fn create_iron_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:85 (sha256:491e857dd109b8011c2343482b72a1e95b4b38e617401ed58fcf03677af9bc5a)
pub fn create_marble_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:96 (sha256:870a311b88865e43c684dea86b257a880568246fe8825ab79dffa10ef7a3a382)
pub fn create_plastic_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:107 (sha256:a04bd6f96f8726b3e73a294602398a742949366ced747c5ce3ab071bb02f2c2e)
pub fn create_rubber_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:118 (sha256:a57f595677317b19c0f1c4b13ec257dceb02b01df554c311e346a167e5377d8e)
pub fn create_silver_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:129 (sha256:6530a0ced19c503c71c857238735c33f55a792f056c13bcb0d4542e845537d0b)
pub fn create_skin_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:140 (sha256:246fb70543d8587a017230d1fc082db9d37112b929d9cc869aafb2533cd44e99)
pub fn create_wood_standard_pbr_material(
    opts: Option<crate::pbr_materials::FlightPartialRecord3584171057>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some({
        let __flight_spread_3 = ((opts).clone()).unwrap_or_default();
        crate::pbr_materials::FlightPartialRecord3584171057 {
            __flight_identity: std::sync::Arc::new(()),
            kind: (__flight_spread_3.kind).clone(),
            name: (__flight_spread_3.name).clone(),
            alpha_cutoff: __flight_spread_3.alpha_cutoff,
            alpha_mode: (__flight_spread_3.alpha_mode).clone(),
            blend_mode: (__flight_spread_3.blend_mode).clone(),
            double_sided: __flight_spread_3.double_sided,
            extensions: (__flight_spread_3.extensions).clone(),
            standard: (__flight_spread_3.standard).clone(),
            shader_key: (__flight_spread_3.shader_key).clone(),
            textures: (__flight_spread_3.textures).clone(),
            uniforms: (__flight_spread_3.uniforms).clone(),
            alpha_map: (__flight_spread_3.alpha_map).clone(),
            base_color: __flight_spread_3.base_color,
            base_color_map: (__flight_spread_3.base_color_map).clone(),
            emissive: __flight_spread_3.emissive,
            emissive_map: (__flight_spread_3.emissive_map).clone(),
            emissive_strength: __flight_spread_3.emissive_strength,
            metallic: __flight_spread_3.metallic,
            metallic_roughness_map: (__flight_spread_3.metallic_roughness_map).clone(),
            normal_map: (__flight_spread_3.normal_map).clone(),
            normal_scale: __flight_spread_3.normal_scale,
            occlusion_map: (__flight_spread_3.occlusion_map).clone(),
            occlusion_strength: __flight_spread_3.occlusion_strength,
            roughness: __flight_spread_3.roughness,
        }
    }));
}
