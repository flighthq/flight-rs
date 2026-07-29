// @generated from upstream/packages/materials/src/pbrExtensionMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_standard_pbr_material_properties, create_surface_material};
use flighthq_types::{
    ANISOTROPY_PBR_MATERIAL_KIND as anisotropy_pbr_material_kind_constant, AlphaType,
    AnisotropyPbrMaterial, BlendMode,
    CLEARCOAT_PBR_MATERIAL_KIND as clearcoat_pbr_material_kind_constant, ClearcoatPbrMaterial,
    IRIDESCENCE_PBR_MATERIAL_KIND as iridescence_pbr_material_kind_constant,
    IridescencePbrMaterial, Kind, MaterialAlphaMode,
    SHEEN_PBR_MATERIAL_KIND as sheen_pbr_material_kind_constant,
    SPECULAR_PBR_MATERIAL_KIND as specular_pbr_material_kind_constant,
    SUBSURFACE_PBR_MATERIAL_KIND as subsurface_pbr_material_kind_constant, SheenPbrMaterial,
    SpecularPbrMaterial, StandardPbrMaterialProperties, SubsurfacePbrMaterial,
    TRANSMISSION_VOLUME_PBR_MATERIAL_KIND as transmission_volume_pbr_material_kind_constant,
    Texture, TransmissionVolumePbrMaterial,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub anisotropy_map: Option<Texture>,
    pub anisotropy_rotation: Option<f64>,
    pub anisotropy_strength: Option<f64>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub clearcoat: Option<f64>,
    pub clearcoat_map: Option<Texture>,
    pub clearcoat_normal_map: Option<Texture>,
    pub clearcoat_roughness: Option<f64>,
    pub clearcoat_roughness_map: Option<Texture>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub iridescence: Option<f64>,
    pub iridescence_ior: Option<f64>,
    pub iridescence_map: Option<Texture>,
    pub iridescence_thickness_map: Option<Texture>,
    pub iridescence_thickness_max: Option<f64>,
    pub iridescence_thickness_min: Option<f64>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub sheen_color: Option<f64>,
    pub sheen_color_map: Option<Texture>,
    pub sheen_roughness: Option<f64>,
    pub sheen_roughness_map: Option<Texture>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub specular: Option<f64>,
    pub specular_color: Option<f64>,
    pub specular_color_map: Option<Texture>,
    pub specular_map: Option<Texture>,
    pub standard: Option<StandardPbrMaterialProperties>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub subsurface: Option<f64>,
    pub subsurface_color: Option<f64>,
    pub subsurface_map: Option<Texture>,
    pub thickness: Option<f64>,
    pub thickness_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub attenuation_color: Option<f64>,
    pub attenuation_distance: Option<f64>,
    pub ior: Option<f64>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub thickness: Option<f64>,
    pub thickness_map: Option<Texture>,
    pub transmission: Option<f64>,
    pub transmission_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
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
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
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
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub alpha_type: Option<AlphaType>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
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
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:26 (sha256:a0d95d6969ffc9c94eeea68ba99deb4103d544bc3c6758e06a3982ad13ddfe16)
pub fn create_anisotropy_pbr_material(opts: Option<FlightPartialRecord1>) -> AnisotropyPbrMaterial {
    let mut material = create_surface_material((anisotropy_pbr_material_kind_constant).to_owned());
    material.anisotropy_map = opts
        .as_ref()
        .and_then(|value| (value.anisotropy_map).clone());
    material.anisotropy_rotation =
        (opts.as_ref().and_then(|value| value.anisotropy_rotation)).unwrap_or(0.0_f64);
    material.anisotropy_strength =
        (opts.as_ref().and_then(|value| value.anisotropy_strength)).unwrap_or(0.0_f64);
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return material;
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:38 (sha256:bacc12c834452ca5e37d88c3120fd960c7021b34348697f2814e1b1c2564e271)
pub fn create_clearcoat_pbr_material(opts: Option<FlightPartialRecord2>) -> ClearcoatPbrMaterial {
    let mut material = create_surface_material((clearcoat_pbr_material_kind_constant).to_owned());
    material.clearcoat = (opts.as_ref().and_then(|value| value.clearcoat)).unwrap_or(0.0_f64);
    material.clearcoat_map = opts
        .as_ref()
        .and_then(|value| (value.clearcoat_map).clone());
    material.clearcoat_normal_map = opts
        .as_ref()
        .and_then(|value| (value.clearcoat_normal_map).clone());
    material.clearcoat_roughness =
        (opts.as_ref().and_then(|value| value.clearcoat_roughness)).unwrap_or(0.0_f64);
    material.clearcoat_roughness_map = opts
        .as_ref()
        .and_then(|value| (value.clearcoat_roughness_map).clone());
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return material;
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:52 (sha256:249f28d834353d9f2618852fbf5d0a55bcaec9634177dc8e88d540b6475530f3)
pub fn create_iridescence_pbr_material(
    opts: Option<FlightPartialRecord3>,
) -> IridescencePbrMaterial {
    let mut material = create_surface_material((iridescence_pbr_material_kind_constant).to_owned());
    material.iridescence = (opts.as_ref().and_then(|value| value.iridescence)).unwrap_or(0.0_f64);
    material.iridescence_ior =
        (opts.as_ref().and_then(|value| value.iridescence_ior)).unwrap_or(1.3_f64);
    material.iridescence_map = opts
        .as_ref()
        .and_then(|value| (value.iridescence_map).clone());
    material.iridescence_thickness_map = opts
        .as_ref()
        .and_then(|value| (value.iridescence_thickness_map).clone());
    material.iridescence_thickness_max = (opts
        .as_ref()
        .and_then(|value| value.iridescence_thickness_max))
    .unwrap_or(400.0_f64);
    material.iridescence_thickness_min = (opts
        .as_ref()
        .and_then(|value| value.iridescence_thickness_min))
    .unwrap_or(100.0_f64);
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return material;
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:66 (sha256:1c8df165b9e747f2ee412eb525194739568029823de250c899e34c192f728802)
pub fn create_sheen_pbr_material(opts: Option<FlightPartialRecord4>) -> SheenPbrMaterial {
    let mut material = create_surface_material((sheen_pbr_material_kind_constant).to_owned());
    material.sheen_color = (opts.as_ref().and_then(|value| value.sheen_color)).unwrap_or(255.0_f64);
    material.sheen_color_map = opts
        .as_ref()
        .and_then(|value| (value.sheen_color_map).clone());
    material.sheen_roughness =
        (opts.as_ref().and_then(|value| value.sheen_roughness)).unwrap_or(0.0_f64);
    material.sheen_roughness_map = opts
        .as_ref()
        .and_then(|value| (value.sheen_roughness_map).clone());
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return material;
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:78 (sha256:aedf6f7bd647e67e2972e45199ef02e58aec34f5b29c9ba7c98d282ae15d6adf)
pub fn create_specular_pbr_material(opts: Option<FlightPartialRecord5>) -> SpecularPbrMaterial {
    let mut material = create_surface_material((specular_pbr_material_kind_constant).to_owned());
    material.specular = (opts.as_ref().and_then(|value| value.specular)).unwrap_or(1.0_f64);
    material.specular_color =
        (opts.as_ref().and_then(|value| value.specular_color)).unwrap_or(4294967295.0_f64);
    material.specular_color_map = opts
        .as_ref()
        .and_then(|value| (value.specular_color_map).clone());
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return material;
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:91 (sha256:e27ed21c62be14dd9fe930b6e42c858dbcc628ea0846a95713e7e6f3b01b7927)
pub fn create_subsurface_pbr_material(opts: Option<FlightPartialRecord6>) -> SubsurfacePbrMaterial {
    let mut material = create_surface_material((subsurface_pbr_material_kind_constant).to_owned());
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    material.subsurface = (opts.as_ref().and_then(|value| value.subsurface)).unwrap_or(0.0_f64);
    material.subsurface_color =
        (opts.as_ref().and_then(|value| value.subsurface_color)).unwrap_or(4294967295.0_f64);
    material.subsurface_map = opts
        .as_ref()
        .and_then(|value| (value.subsurface_map).clone());
    material.thickness = (opts.as_ref().and_then(|value| value.thickness)).unwrap_or(0.0_f64);
    material.thickness_map = opts
        .as_ref()
        .and_then(|value| (value.thickness_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:106 (sha256:3c6e424d049a3225554ce25277982f92430910648c55bc2f1252647cf5465448)
pub fn create_transmission_volume_pbr_material(
    opts: Option<FlightPartialRecord7>,
) -> TransmissionVolumePbrMaterial {
    let mut material =
        create_surface_material((transmission_volume_pbr_material_kind_constant).to_owned());
    material.attenuation_color =
        (opts.as_ref().and_then(|value| value.attenuation_color)).unwrap_or(4294967295.0_f64);
    material.attenuation_distance =
        (opts.as_ref().and_then(|value| value.attenuation_distance)).unwrap_or(f64::INFINITY);
    material.ior = (opts.as_ref().and_then(|value| value.ior)).unwrap_or(1.5_f64);
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    material.thickness = (opts.as_ref().and_then(|value| value.thickness)).unwrap_or(0.0_f64);
    material.thickness_map = opts
        .as_ref()
        .and_then(|value| (value.thickness_map).clone());
    material.transmission = (opts.as_ref().and_then(|value| value.transmission)).unwrap_or(0.0_f64);
    material.transmission_map = opts
        .as_ref()
        .and_then(|value| (value.transmission_map).clone());
    return material;
}
