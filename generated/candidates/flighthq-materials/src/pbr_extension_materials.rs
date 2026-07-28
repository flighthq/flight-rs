// @generated from upstream/packages/materials/src/pbrExtensionMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_standard_pbr_material_properties, create_surface_material};
use flighthq_types::{
    ANISOTROPY_PBR_MATERIAL_KIND as anisotropy_pbr_material_kind_constant, AnisotropyPbrMaterial,
    CLEARCOAT_PBR_MATERIAL_KIND as clearcoat_pbr_material_kind_constant, ClearcoatPbrMaterial,
    IRIDESCENCE_PBR_MATERIAL_KIND as iridescence_pbr_material_kind_constant,
    IridescencePbrMaterial, SHEEN_PBR_MATERIAL_KIND as sheen_pbr_material_kind_constant,
    SPECULAR_PBR_MATERIAL_KIND as specular_pbr_material_kind_constant,
    SUBSURFACE_PBR_MATERIAL_KIND as subsurface_pbr_material_kind_constant, SheenPbrMaterial,
    SpecularPbrMaterial, SubsurfacePbrMaterial,
    TRANSMISSION_VOLUME_PBR_MATERIAL_KIND as transmission_volume_pbr_material_kind_constant,
    TransmissionVolumePbrMaterial,
};

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:26 (sha256:a0d95d6969ffc9c94eeea68ba99deb4103d544bc3c6758e06a3982ad13ddfe16)
pub fn create_anisotropy_pbr_material(
    opts: Option<AnisotropyPbrMaterial>,
) -> AnisotropyPbrMaterial {
    let mut material = create_surface_material(anisotropy_pbr_material_kind_constant);
    material.anisotropy_map = opts
        .as_ref()
        .and_then(|value| (value.anisotropy_map).clone());
    material.anisotropy_rotation =
        (opts.as_ref().map(|value| value.anisotropy_rotation)).unwrap_or(0.0_f64);
    material.anisotropy_strength =
        (opts.as_ref().map(|value| value.anisotropy_strength)).unwrap_or(0.0_f64);
    material.standard = (opts.as_ref().map(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return (material).clone();
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:38 (sha256:bacc12c834452ca5e37d88c3120fd960c7021b34348697f2814e1b1c2564e271)
pub fn create_clearcoat_pbr_material(opts: Option<ClearcoatPbrMaterial>) -> ClearcoatPbrMaterial {
    let mut material = create_surface_material(clearcoat_pbr_material_kind_constant);
    material.clearcoat = (opts.as_ref().map(|value| value.clearcoat)).unwrap_or(0.0_f64);
    material.clearcoat_map = opts
        .as_ref()
        .and_then(|value| (value.clearcoat_map).clone());
    material.clearcoat_normal_map = opts
        .as_ref()
        .and_then(|value| (value.clearcoat_normal_map).clone());
    material.clearcoat_roughness =
        (opts.as_ref().map(|value| value.clearcoat_roughness)).unwrap_or(0.0_f64);
    material.clearcoat_roughness_map = opts
        .as_ref()
        .and_then(|value| (value.clearcoat_roughness_map).clone());
    material.standard = (opts.as_ref().map(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return (material).clone();
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:52 (sha256:249f28d834353d9f2618852fbf5d0a55bcaec9634177dc8e88d540b6475530f3)
pub fn create_iridescence_pbr_material(
    opts: Option<IridescencePbrMaterial>,
) -> IridescencePbrMaterial {
    let mut material = create_surface_material(iridescence_pbr_material_kind_constant);
    material.iridescence = (opts.as_ref().map(|value| value.iridescence)).unwrap_or(0.0_f64);
    material.iridescence_ior =
        (opts.as_ref().map(|value| value.iridescence_ior)).unwrap_or(1.3_f64);
    material.iridescence_map = opts
        .as_ref()
        .and_then(|value| (value.iridescence_map).clone());
    material.iridescence_thickness_map = opts
        .as_ref()
        .and_then(|value| (value.iridescence_thickness_map).clone());
    material.iridescence_thickness_max =
        (opts.as_ref().map(|value| value.iridescence_thickness_max)).unwrap_or(400.0_f64);
    material.iridescence_thickness_min =
        (opts.as_ref().map(|value| value.iridescence_thickness_min)).unwrap_or(100.0_f64);
    material.standard = (opts.as_ref().map(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return (material).clone();
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:66 (sha256:1c8df165b9e747f2ee412eb525194739568029823de250c899e34c192f728802)
pub fn create_sheen_pbr_material(opts: Option<SheenPbrMaterial>) -> SheenPbrMaterial {
    let mut material = create_surface_material(sheen_pbr_material_kind_constant);
    material.sheen_color = (opts.as_ref().map(|value| value.sheen_color)).unwrap_or(255.0_f64);
    material.sheen_color_map = opts
        .as_ref()
        .and_then(|value| (value.sheen_color_map).clone());
    material.sheen_roughness =
        (opts.as_ref().map(|value| value.sheen_roughness)).unwrap_or(0.0_f64);
    material.sheen_roughness_map = opts
        .as_ref()
        .and_then(|value| (value.sheen_roughness_map).clone());
    material.standard = (opts.as_ref().map(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return (material).clone();
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:78 (sha256:aedf6f7bd647e67e2972e45199ef02e58aec34f5b29c9ba7c98d282ae15d6adf)
pub fn create_specular_pbr_material(opts: Option<SpecularPbrMaterial>) -> SpecularPbrMaterial {
    let mut material = create_surface_material(specular_pbr_material_kind_constant);
    material.specular = (opts.as_ref().map(|value| value.specular)).unwrap_or(1.0_f64);
    material.specular_color =
        (opts.as_ref().map(|value| value.specular_color)).unwrap_or(4294967295.0_f64);
    material.specular_color_map = opts
        .as_ref()
        .and_then(|value| (value.specular_color_map).clone());
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    material.standard = (opts.as_ref().map(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return (material).clone();
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:91 (sha256:e27ed21c62be14dd9fe930b6e42c858dbcc628ea0846a95713e7e6f3b01b7927)
pub fn create_subsurface_pbr_material(
    opts: Option<SubsurfacePbrMaterial>,
) -> SubsurfacePbrMaterial {
    let mut material = create_surface_material(subsurface_pbr_material_kind_constant);
    material.standard = (opts.as_ref().map(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    material.subsurface = (opts.as_ref().map(|value| value.subsurface)).unwrap_or(0.0_f64);
    material.subsurface_color =
        (opts.as_ref().map(|value| value.subsurface_color)).unwrap_or(4294967295.0_f64);
    material.subsurface_map = opts
        .as_ref()
        .and_then(|value| (value.subsurface_map).clone());
    material.thickness = (opts.as_ref().map(|value| value.thickness)).unwrap_or(0.0_f64);
    material.thickness_map = opts
        .as_ref()
        .and_then(|value| (value.thickness_map).clone());
    return (material).clone();
}

// Source: upstream/packages/materials/src/pbrExtensionMaterials.ts:106 (sha256:3c6e424d049a3225554ce25277982f92430910648c55bc2f1252647cf5465448)
pub fn create_transmission_volume_pbr_material(
    opts: Option<TransmissionVolumePbrMaterial>,
) -> TransmissionVolumePbrMaterial {
    let mut material = create_surface_material(transmission_volume_pbr_material_kind_constant);
    material.attenuation_color =
        (opts.as_ref().map(|value| value.attenuation_color)).unwrap_or(4294967295.0_f64);
    material.attenuation_distance =
        (opts.as_ref().map(|value| value.attenuation_distance)).unwrap_or(f64::INFINITY);
    material.ior = (opts.as_ref().map(|value| value.ior)).unwrap_or(1.5_f64);
    material.standard = (opts.as_ref().map(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    material.thickness = (opts.as_ref().map(|value| value.thickness)).unwrap_or(0.0_f64);
    material.thickness_map = opts
        .as_ref()
        .and_then(|value| (value.thickness_map).clone());
    material.transmission = (opts.as_ref().map(|value| value.transmission)).unwrap_or(0.0_f64);
    material.transmission_map = opts
        .as_ref()
        .and_then(|value| (value.transmission_map).clone());
    return (material).clone();
}
