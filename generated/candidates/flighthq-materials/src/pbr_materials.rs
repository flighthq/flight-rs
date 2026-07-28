// @generated from upstream/packages/materials/src/pbrMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_color::unpack_color_to_linear;
use flighthq_types::{
    SPECULAR_GLOSSINESS_PBR_MATERIAL_KIND as specular_glossiness_pbr_material_kind_constant,
    STANDARD_PBR_MATERIAL_KIND as standard_pbr_material_kind_constant,
    SpecularGlossinessPbrMaterial, StandardPbrMaterial, StandardPbrMaterialProperties,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:23 (sha256:4846ac22876a1277fb9e084e5fd6d87f1254ca1b0fc2d64a65465ee5ba84be40)
pub fn convert_specular_glossiness_to_standard_pbr(
    out: &mut StandardPbrMaterialProperties,
    source: &SpecularGlossinessPbrMaterial,
) -> () {
    let diffuse = source.diffuse;
    let specular = source.specular;
    let glossiness = source.glossiness;
    let diffuse_map = (source.diffuse_map).clone();
    let specular_glossiness_map = (source.specular_glossiness_map).clone();
    let emissive = source.emissive;
    let emissive_map = (source.emissive_map).clone();
    let emissive_strength = source.emissive_strength;
    let normal_map = (source.normal_map).clone();
    let normal_scale = source.normal_scale;
    let occlusion_map = (source.occlusion_map).clone();
    let occlusion_strength = source.occlusion_strength;
    unpack_color_to_linear(&mut SCRATCH_LINEAR, specular);
    let spec_r = SCRATCH_LINEAR[0.0_f64 as usize].clone();
    let spec_g = SCRATCH_LINEAR[1.0_f64 as usize].clone();
    let spec_b = SCRATCH_LINEAR[2.0_f64 as usize].clone();
    let spec_luma = (((0.2126_f64 * spec_r) + (0.7152_f64 * spec_g)) + (0.0722_f64 * spec_b));
    let dielectric_f0 = 0.04_f64;
    let metallic =
        (1.0_f64).min((0.0_f64).max(((spec_luma - dielectric_f0) / (1.0_f64 - dielectric_f0))));
    unpack_color_to_linear(&mut SCRATCH_LINEAR2, diffuse);
    let diff_r = SCRATCH_LINEAR2[0.0_f64 as usize].clone();
    let diff_g = SCRATCH_LINEAR2[1.0_f64 as usize].clone();
    let diff_b = SCRATCH_LINEAR2[2.0_f64 as usize].clone();
    let diff_a = SCRATCH_LINEAR2[3.0_f64 as usize].clone();
    let base_r = (diff_r * (1.0_f64 - (spec_luma * (1.0_f64 - metallic))));
    let base_g = (diff_g * (1.0_f64 - (spec_luma * (1.0_f64 - metallic))));
    let base_b = (diff_b * (1.0_f64 - (spec_luma * (1.0_f64 - metallic))));
    let base_color = pack_linear(base_r, base_g, base_b, diff_a);
    out.base_color = base_color;
    out.base_color_map = (diffuse_map).clone();
    out.emissive = emissive;
    out.emissive_map = (emissive_map).clone();
    out.emissive_strength = emissive_strength;
    out.metallic = metallic;
    out.metallic_roughness_map = (specular_glossiness_map).clone();
    out.normal_map = (normal_map).clone();
    out.normal_scale = normal_scale;
    out.occlusion_map = (occlusion_map).clone();
    out.occlusion_strength = occlusion_strength;
    out.roughness = (1.0_f64 - glossiness);
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:82 (sha256:6d1e5a7dd1430e301aca3227e55149598a38dc442ceaafc42b6d1630f3fea604)
pub fn create_specular_glossiness_pbr_material(
    opts: Option<SpecularGlossinessPbrMaterial>,
) -> SpecularGlossinessPbrMaterial {
    let mut material =
        create_surface_material((specular_glossiness_pbr_material_kind_constant).to_owned());
    material.diffuse = (opts.as_ref().map(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.emissive = (opts.as_ref().map(|value| value.emissive)).unwrap_or(255.0_f64);
    material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    material.emissive_strength =
        (opts.as_ref().map(|value| value.emissive_strength)).unwrap_or(1.0_f64);
    material.glossiness = (opts.as_ref().map(|value| value.glossiness)).unwrap_or(1.0_f64);
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().map(|value| value.normal_scale)).unwrap_or(1.0_f64);
    material.occlusion_map = opts
        .as_ref()
        .and_then(|value| (value.occlusion_map).clone());
    material.occlusion_strength =
        (opts.as_ref().map(|value| value.occlusion_strength)).unwrap_or(1.0_f64);
    material.specular = (opts.as_ref().map(|value| value.specular)).unwrap_or(4294967295.0_f64);
    material.specular_glossiness_map = opts
        .as_ref()
        .and_then(|value| (value.specular_glossiness_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:104 (sha256:2aec8f6092e3534c75f4a3953fd7b612468639f95e378d26fd5cb1e5d604c3e6)
pub fn create_standard_pbr_material(opts: Option<StandardPbrMaterial>) -> StandardPbrMaterial {
    let mut material = create_surface_material((standard_pbr_material_kind_constant).to_owned());
    assign_standard_pbr_material_properties(&mut material, Some(((opts).clone().unwrap()).clone()));
    return material;
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:113 (sha256:ffc83ec68e9fe692c7c4c9719572cc5fa67771f90a9fe42f2b02caf9f0584bcb)
#[derive(Clone)]
struct CreateStandardPbrMaterialPropertiesRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateStandardPbrMaterialPropertiesRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_standard_pbr_material_properties(
    opts: Option<StandardPbrMaterialProperties>,
) -> StandardPbrMaterialProperties {
    let mut properties = StandardPbrMaterialProperties {
        __flight_identity: std::sync::Arc::new(()),
    };
    assign_standard_pbr_material_properties(
        &mut properties,
        Some(((opts).clone().unwrap()).clone()),
    );
    return properties;
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:123 (sha256:d85d2e708d1f2dc62493236719438cf15efe77c835cb7179c53e09a02fd654cc)
fn assign_standard_pbr_material_properties(
    target: &mut StandardPbrMaterialProperties,
    opts: Option<StandardPbrMaterialProperties>,
) -> () {
    target.base_color = (opts.as_ref().map(|value| value.base_color)).unwrap_or(4294967295.0_f64);
    target.base_color_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    target.emissive = (opts.as_ref().map(|value| value.emissive)).unwrap_or(255.0_f64);
    target.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    target.emissive_strength =
        (opts.as_ref().map(|value| value.emissive_strength)).unwrap_or(1.0_f64);
    target.metallic = (opts.as_ref().map(|value| value.metallic)).unwrap_or(0.0_f64);
    target.metallic_roughness_map = opts
        .as_ref()
        .and_then(|value| (value.metallic_roughness_map).clone());
    target.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    target.normal_scale = (opts.as_ref().map(|value| value.normal_scale)).unwrap_or(1.0_f64);
    target.occlusion_map = opts
        .as_ref()
        .and_then(|value| (value.occlusion_map).clone());
    target.occlusion_strength =
        (opts.as_ref().map(|value| value.occlusion_strength)).unwrap_or(1.0_f64);
    target.roughness = (opts.as_ref().map(|value| value.roughness)).unwrap_or(1.0_f64);
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:142 (sha256:d73291d2c9a128374c2e52e8713d2f8ac4c87c45e28969c9f1bba7dfb19b8913)
fn linear_channel_to_srgb8(value: f64) -> f64 {
    let srgb = if (value <= 0.0031308_f64) {
        (value * 12.92_f64)
    } else {
        ((1.055_f64 * (value).powf((1.0_f64 / 2.4_f64))) - 0.055_f64)
    };
    return ((1.0_f64).min((0.0_f64).max(srgb)) * 255.0_f64).round();
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:148 (sha256:090c9090fa5ea7ec2b41bb45dacd19e1a31b5efb978c884fc7e7da324a5d6a34)
fn pack_linear(r: f64, g: f64, b: f64, a: f64) -> f64 {
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32(linear_channel_to_srgb8(r))
                        .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31))
                        as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32(linear_channel_to_srgb8(g))
                        .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31))
                        as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32(linear_channel_to_srgb8(b))
                    .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32((a * 255.0_f64).round())) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:158 (sha256:4de967a9e344614ed886cb790ba6f585de1c2d42f051926890b3b7de517eb6b7)
static SCRATCH_LINEAR: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/materials/src/pbrMaterials.ts:159 (sha256:94db8fa211f564f31fb60b498857c9abf1208978aebee1c61cbe90a5cacc0440)
static SCRATCH_LINEAR2: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
