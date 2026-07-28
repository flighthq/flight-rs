// @generated from upstream/packages/materials/src/materialPresets.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_standard_pbr_material, create_transmission_volume_pbr_material};
use flighthq_types::{
    AlphaType, BlendMode, Kind, MaterialAlphaMode, StandardPbrMaterial,
    StandardPbrMaterialProperties, Texture, TransmissionVolumePbrMaterial,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: Option<f64>,
    pub metallic: Option<f64>,
    pub metallic_roughness_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: Option<f64>,
    pub roughness: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub attenuation_color: Option<f64>,
    pub attenuation_distance: Option<f64>,
    pub ior: Option<f64>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub thickness: Option<f64>,
    pub thickness_map: Option<Texture>,
    pub transmission: Option<f64>,
    pub transmission_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: Option<f64>,
    pub metallic: Option<f64>,
    pub metallic_roughness_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: Option<f64>,
    pub roughness: Option<f64>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub anisotropy_map: Option<Texture>,
    pub anisotropy_rotation: Option<f64>,
    pub anisotropy_strength: Option<f64>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub clearcoat: Option<f64>,
    pub clearcoat_map: Option<Texture>,
    pub clearcoat_normal_map: Option<Texture>,
    pub clearcoat_roughness: Option<f64>,
    pub clearcoat_roughness_map: Option<Texture>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub iridescence: Option<f64>,
    pub iridescence_ior: Option<f64>,
    pub iridescence_map: Option<Texture>,
    pub iridescence_thickness_map: Option<Texture>,
    pub iridescence_thickness_max: Option<f64>,
    pub iridescence_thickness_min: Option<f64>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub sheen_color: Option<f64>,
    pub sheen_color_map: Option<Texture>,
    pub sheen_roughness: Option<f64>,
    pub sheen_roughness_map: Option<Texture>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: Option<f64>,
    pub glossiness: Option<f64>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: Option<f64>,
    pub specular: Option<f64>,
    pub specular_glossiness_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub specular: Option<f64>,
    pub specular_color: Option<f64>,
    pub specular_color_map: Option<Texture>,
    pub specular_map: Option<Texture>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub subsurface: Option<f64>,
    pub subsurface_color: Option<f64>,
    pub subsurface_map: Option<Texture>,
    pub thickness: Option<f64>,
    pub thickness_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/materialPresets.ts:15 (sha256:37bf6a37ffd0849714ad8499eb0e6b52ef5b1c6a8f276385256df8c490589610)
pub fn create_aluminum_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(2964369663.0_f64),
        metallic: Some(1.0_f64),
        roughness: Some(0.35_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:26 (sha256:8fad8dc882e56f36710e1f987034293853e63b686144e0d61ab7b4efb621c0ff)
pub fn create_carbon_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(437918463.0_f64),
        metallic: Some(0.0_f64),
        roughness: Some(0.95_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:37 (sha256:ae18ae07c254633d04c45d00d3e8b9beff00cd33a8383f9361971a63951c0297)
pub fn create_glass_transmission_volume_pbr_material(
    opts: Option<FlightPartialRecord2>,
) -> TransmissionVolumePbrMaterial {
    return create_transmission_volume_pbr_material(Some(FlightPartialRecord2 {
        ior: Some(1.5_f64),
        transmission: Some(1.0_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:49 (sha256:d98f9f1d62d29f41e2d5d19de7b0cb28609ad353bb45184ca449a58d7af5d370)
pub fn create_gold_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(4292280575.0_f64),
        metallic: Some(1.0_f64),
        roughness: Some(0.25_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:60 (sha256:107a10d2a6ef94f08211dd15c88d8ce086e4f396393dea1ada79f9a53dbd4139)
pub fn create_iron_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(1145324799.0_f64),
        metallic: Some(1.0_f64),
        roughness: Some(0.7_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:71 (sha256:491e857dd109b8011c2343482b72a1e95b4b38e617401ed58fcf03677af9bc5a)
pub fn create_marble_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(4126537215.0_f64),
        metallic: Some(0.0_f64),
        roughness: Some(0.05_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:82 (sha256:870a311b88865e43c684dea86b257a880568246fe8825ab79dffa10ef7a3a382)
pub fn create_plastic_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(4294967295.0_f64),
        metallic: Some(0.0_f64),
        roughness: Some(0.05_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:93 (sha256:a04bd6f96f8726b3e73a294602398a742949366ced747c5ce3ab071bb02f2c2e)
pub fn create_rubber_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(471604479.0_f64),
        metallic: Some(0.0_f64),
        roughness: Some(0.9_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:104 (sha256:a57f595677317b19c0f1c4b13ec257dceb02b01df554c311e346a167e5377d8e)
pub fn create_silver_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(3233857791.0_f64),
        metallic: Some(1.0_f64),
        roughness: Some(0.1_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:115 (sha256:6530a0ced19c503c71c857238735c33f55a792f056c13bcb0d4542e845537d0b)
pub fn create_skin_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(4291598847.0_f64),
        metallic: Some(0.0_f64),
        roughness: Some(0.4_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:126 (sha256:246fb70543d8587a017230d1fc082db9d37112b929d9cc869aafb2533cd44e99)
pub fn create_wood_standard_pbr_material(
    opts: Option<FlightPartialRecord1>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(FlightPartialRecord1 {
        base_color: Some(2337942527.0_f64),
        metallic: Some(0.0_f64),
        roughness: Some(0.8_f64),
        ..((opts).clone().unwrap()).clone()
    }));
}
