// @generated from upstream/packages/materials/src/pbrMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_material3_d, explain_specular_glossiness_conversion};
use flighthq_color::unpack_color_to_linear;
use flighthq_types::{
    BlendMode, Kind, Material3DOptions, MaterialAlphaMode, MaterialConversionExplanation,
    MaterialConversionGuard, Modifier, NonEntityCreateResult, PbrExtension,
    SPECULAR_GLOSSINESS_PBR_MATERIAL_KIND as specular_glossiness_pbr_material_kind_constant,
    STANDARD_PBR_MATERIAL_KIND as standard_pbr_material_kind_constant,
    SpecularGlossinessPbrMaterial, StandardPbrMaterial, StandardPbrMaterialProperties, Texture,
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord1996927468 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub color: Option<f64>,
    pub thickness: Option<f64>,
    pub tint: Option<f64>,
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
    pub ramp: Option<Texture>,
    pub steps: Option<f64>,
    pub alpha_map: Option<Texture>,
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
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub glossiness: Option<f64>,
    pub specular: Option<f64>,
    pub specular_glossiness_map: Option<Texture>,
    pub modifiers: Option<Vec<Modifier>>,
    pub shininess: Option<f64>,
    pub specular_map: Option<Texture>,
    pub matcap: Option<Texture>,
    pub extensions: Option<Vec<PbrExtension>>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub far: Option<f64>,
    pub near: Option<f64>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for FlightPartialRecord1996927468 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3953157034 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub color: Option<f64>,
    pub thickness: Option<f64>,
    pub tint: Option<f64>,
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
    pub ramp: Option<Texture>,
    pub steps: Option<f64>,
    pub alpha_map: Option<Texture>,
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
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub glossiness: Option<f64>,
    pub specular: Option<f64>,
    pub specular_glossiness_map: Option<Texture>,
    pub modifiers: Option<Vec<Modifier>>,
    pub shininess: Option<f64>,
    pub specular_map: Option<Texture>,
    pub matcap: Option<Texture>,
    pub extensions: Option<Vec<PbrExtension>>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub far: Option<f64>,
    pub near: Option<f64>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for FlightPartialRecord3953157034 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3905749610 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_map: Option<Texture>,
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
impl PartialEq for FlightPartialRecord3905749610 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:29 (sha256:2b7b2f80faeb8a6151c6d080e3f5f8911985b06db3c18bf8f46d1022e1bc1bb5)
pub fn convert_specular_glossiness_to_standard_pbr(
    out: &mut StandardPbrMaterialProperties,
    source: &SpecularGlossinessPbrMaterial,
) -> () {
    let diffuse = source.diffuse;
    let specular = source.specular;
    let glossiness = source.glossiness;
    let diffuse_map = (source.diffuse_map).clone();
    let emissive = source.emissive;
    let emissive_map = (source.emissive_map).clone();
    let emissive_strength = source.emissive_strength;
    let normal_map = (source.normal_map).clone();
    let normal_scale = source.normal_scale;
    let occlusion_map = (source.occlusion_map).clone();
    let occlusion_strength = source.occlusion_strength;
    unpack_color_to_linear(&mut (*SCRATCH_LINEAR.lock().unwrap()), specular);
    let spec_r = (*SCRATCH_LINEAR.lock().unwrap())[0.0_f64 as usize].clone();
    let spec_g = (*SCRATCH_LINEAR.lock().unwrap())[1.0_f64 as usize].clone();
    let spec_b = (*SCRATCH_LINEAR.lock().unwrap())[2.0_f64 as usize].clone();
    let spec_luma = (((0.2126_f64 * spec_r) + (0.7152_f64 * spec_g)) + (0.0722_f64 * spec_b));
    let dielectric_f0 = 0.04_f64;
    let metallic =
        (1.0_f64).min((0.0_f64).max(((spec_luma - dielectric_f0) / (1.0_f64 - dielectric_f0))));
    unpack_color_to_linear(&mut (*SCRATCH_LINEAR2.lock().unwrap()), diffuse);
    let diff_r = (*SCRATCH_LINEAR2.lock().unwrap())[0.0_f64 as usize].clone();
    let diff_g = (*SCRATCH_LINEAR2.lock().unwrap())[1.0_f64 as usize].clone();
    let diff_b = (*SCRATCH_LINEAR2.lock().unwrap())[2.0_f64 as usize].clone();
    let diff_a = (*SCRATCH_LINEAR2.lock().unwrap())[3.0_f64 as usize].clone();
    let base_r = (diff_r * (1.0_f64 - (spec_luma * (1.0_f64 - metallic))));
    let base_g = (diff_g * (1.0_f64 - (spec_luma * (1.0_f64 - metallic))));
    let base_b = (diff_b * (1.0_f64 - (spec_luma * (1.0_f64 - metallic))));
    let base_color = pack_linear(base_r, base_g, base_b, diff_a);
    out.alpha_map = None;
    out.base_color = base_color;
    out.base_color_map = (diffuse_map).clone();
    out.emissive = emissive;
    out.emissive_map = (emissive_map).clone();
    out.emissive_strength = emissive_strength;
    out.metallic = metallic;
    if ((source.specular_glossiness_map).clone()).is_some() {
        report_material_conversion_drop(
            &explain_specular_glossiness_conversion(source),
            "convertSpecularGlossinessToStandardPbr".to_owned(),
        );
    }
    out.metallic_roughness_map = None;
    out.normal_map = (normal_map).clone();
    out.normal_scale = normal_scale;
    out.occlusion_map = (occlusion_map).clone();
    out.occlusion_strength = occlusion_strength;
    out.roughness = (1.0_f64 - glossiness);
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:95 (sha256:5e042c55287eb7fd2bf26319592a0a096f501f03d7851a949a5079a9d14db70e)
pub fn create_specular_glossiness_pbr_material(
    opts: Option<FlightPartialRecord1996927468>,
) -> SpecularGlossinessPbrMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (specular_glossiness_pbr_material_kind_constant).to_owned(),
            ((opts).clone()).as_ref().map(|__flight_value| {
                let __flight_source = &(__flight_value);
                Material3DOptions {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    alpha_cutoff: __flight_source.alpha_cutoff,
                    alpha_mode: (__flight_source.alpha_mode).clone(),
                    blend_mode: (__flight_source.blend_mode).clone(),
                    double_sided: __flight_source.double_sided,
                }
            }),
        ));
        SpecularGlossinessPbrMaterial {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            alpha_cutoff: __flight_source.alpha_cutoff,
            alpha_mode: (__flight_source.alpha_mode).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
            color: __flight_source.color,
            thickness: __flight_source.thickness,
            tint: __flight_source.tint,
            base_color: __flight_source.base_color,
            base_color_map: (__flight_source.base_color_map).clone(),
            ramp: (__flight_source.ramp).clone(),
            steps: __flight_source.steps,
            alpha_map: (__flight_source.alpha_map).clone(),
            emissive: __flight_source.emissive,
            emissive_map: (__flight_source.emissive_map).clone(),
            emissive_strength: __flight_source.emissive_strength,
            metallic: __flight_source.metallic,
            metallic_roughness_map: (__flight_source.metallic_roughness_map).clone(),
            normal_map: (__flight_source.normal_map).clone(),
            normal_scale: __flight_source.normal_scale,
            occlusion_map: (__flight_source.occlusion_map).clone(),
            occlusion_strength: __flight_source.occlusion_strength,
            roughness: __flight_source.roughness,
            diffuse: __flight_source.diffuse,
            diffuse_map: (__flight_source.diffuse_map).clone(),
            glossiness: __flight_source.glossiness,
            specular: __flight_source.specular,
            specular_glossiness_map: (__flight_source.specular_glossiness_map).clone(),
            modifiers: (__flight_source.modifiers).clone(),
            shininess: __flight_source.shininess,
            specular_map: (__flight_source.specular_map).clone(),
            matcap: (__flight_source.matcap).clone(),
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            far: __flight_source.far,
            near: __flight_source.near,
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            ..Default::default()
        }
    };
    material.diffuse = (opts.as_ref().and_then(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.emissive = (opts.as_ref().and_then(|value| value.emissive)).unwrap_or(255.0_f64);
    material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    material.emissive_strength =
        (opts.as_ref().and_then(|value| value.emissive_strength)).unwrap_or(1.0_f64);
    material.glossiness = (opts.as_ref().and_then(|value| value.glossiness)).unwrap_or(1.0_f64);
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
    material.occlusion_map = opts
        .as_ref()
        .and_then(|value| (value.occlusion_map).clone());
    material.occlusion_strength =
        (opts.as_ref().and_then(|value| value.occlusion_strength)).unwrap_or(1.0_f64);
    material.specular =
        (opts.as_ref().and_then(|value| value.specular)).unwrap_or(4294967295.0_f64);
    material.specular_glossiness_map = opts
        .as_ref()
        .and_then(|value| (value.specular_glossiness_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:117 (sha256:6c2c4b3c4296b7a5c4e378322b8854bfd9ff12f73e80f4792ed10b068cd5dd3f)
pub fn create_standard_pbr_material(
    opts: Option<FlightPartialRecord3953157034>,
) -> StandardPbrMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (standard_pbr_material_kind_constant).to_owned(),
            ((opts).clone()).as_ref().map(|__flight_value| {
                let __flight_source = &(__flight_value);
                Material3DOptions {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    alpha_cutoff: __flight_source.alpha_cutoff,
                    alpha_mode: (__flight_source.alpha_mode).clone(),
                    blend_mode: (__flight_source.blend_mode).clone(),
                    double_sided: __flight_source.double_sided,
                }
            }),
        ));
        StandardPbrMaterial {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            alpha_cutoff: __flight_source.alpha_cutoff,
            alpha_mode: (__flight_source.alpha_mode).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
            color: __flight_source.color,
            thickness: __flight_source.thickness,
            tint: __flight_source.tint,
            base_color: __flight_source.base_color,
            base_color_map: (__flight_source.base_color_map).clone(),
            ramp: (__flight_source.ramp).clone(),
            steps: __flight_source.steps,
            alpha_map: (__flight_source.alpha_map).clone(),
            emissive: __flight_source.emissive,
            emissive_map: (__flight_source.emissive_map).clone(),
            emissive_strength: __flight_source.emissive_strength,
            metallic: __flight_source.metallic,
            metallic_roughness_map: (__flight_source.metallic_roughness_map).clone(),
            normal_map: (__flight_source.normal_map).clone(),
            normal_scale: __flight_source.normal_scale,
            occlusion_map: (__flight_source.occlusion_map).clone(),
            occlusion_strength: __flight_source.occlusion_strength,
            roughness: __flight_source.roughness,
            diffuse: __flight_source.diffuse,
            diffuse_map: (__flight_source.diffuse_map).clone(),
            glossiness: __flight_source.glossiness,
            specular: __flight_source.specular,
            specular_glossiness_map: (__flight_source.specular_glossiness_map).clone(),
            modifiers: (__flight_source.modifiers).clone(),
            shininess: __flight_source.shininess,
            specular_map: (__flight_source.specular_map).clone(),
            matcap: (__flight_source.matcap).clone(),
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            far: __flight_source.far,
            near: __flight_source.near,
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            ..Default::default()
        }
    };
    {
        material.alpha_map = opts.as_ref().and_then(|value| (value.alpha_map).clone());
        material.base_color =
            (opts.as_ref().and_then(|value| value.base_color)).unwrap_or(4294967295.0_f64);
        material.base_color_map = opts
            .as_ref()
            .and_then(|value| (value.base_color_map).clone());
        material.emissive = (opts.as_ref().and_then(|value| value.emissive)).unwrap_or(255.0_f64);
        material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
        material.emissive_strength =
            (opts.as_ref().and_then(|value| value.emissive_strength)).unwrap_or(1.0_f64);
        material.metallic = (opts.as_ref().and_then(|value| value.metallic)).unwrap_or(0.0_f64);
        material.metallic_roughness_map = opts
            .as_ref()
            .and_then(|value| (value.metallic_roughness_map).clone());
        material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
        material.normal_scale =
            (opts.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
        material.occlusion_map = opts
            .as_ref()
            .and_then(|value| (value.occlusion_map).clone());
        material.occlusion_strength =
            (opts.as_ref().and_then(|value| value.occlusion_strength)).unwrap_or(1.0_f64);
        material.roughness = (opts.as_ref().and_then(|value| value.roughness)).unwrap_or(1.0_f64);
    };
    return material;
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:126 (sha256:886c3e6b850bae52c74d7327ad7dddab32b2907241acb5566e1debd36c2b53d7)
#[derive(Clone, Default)]
struct CreateStandardPbrMaterialPropertiesRecord4 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateStandardPbrMaterialPropertiesRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_standard_pbr_material_properties(
    opts: Option<FlightPartialRecord3905749610>,
) -> NonEntityCreateResult<StandardPbrMaterialProperties, String> {
    let mut properties = StandardPbrMaterialProperties {
        __flight_identity: std::sync::Arc::new(()),
        alpha_map: Default::default(),
        base_color: Default::default(),
        base_color_map: Default::default(),
        emissive: Default::default(),
        emissive_map: Default::default(),
        emissive_strength: Default::default(),
        metallic: Default::default(),
        metallic_roughness_map: Default::default(),
        normal_map: Default::default(),
        normal_scale: Default::default(),
        occlusion_map: Default::default(),
        occlusion_strength: Default::default(),
        roughness: Default::default(),
    };
    assign_standard_pbr_material_properties(
        &mut properties,
        ((opts).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord3905749610 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                alpha_map: (__flight_source.alpha_map).clone(),
                base_color: __flight_source.base_color,
                base_color_map: (__flight_source.base_color_map).clone(),
                emissive: __flight_source.emissive,
                emissive_map: (__flight_source.emissive_map).clone(),
                emissive_strength: __flight_source.emissive_strength,
                metallic: __flight_source.metallic,
                metallic_roughness_map: (__flight_source.metallic_roughness_map).clone(),
                normal_map: (__flight_source.normal_map).clone(),
                normal_scale: __flight_source.normal_scale,
                occlusion_map: (__flight_source.occlusion_map).clone(),
                occlusion_strength: __flight_source.occlusion_strength,
                roughness: __flight_source.roughness,
            }
        }),
    );
    return {
        let __flight_portable_source = (properties).clone();
        crate::FlightValue::Record({
            let mut __flight_record = Vec::new();
            __flight_record
        })
    };
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:136 (sha256:3a38d2ff6d36f421b9fcdb0d3aaae37ca3c90711b4935ce56c59307263a2e20f)
fn assign_standard_pbr_material_properties(
    target: &mut StandardPbrMaterialProperties,
    opts: Option<FlightPartialRecord3905749610>,
) -> () {
    target.alpha_map = opts.as_ref().and_then(|value| (value.alpha_map).clone());
    target.base_color =
        (opts.as_ref().and_then(|value| value.base_color)).unwrap_or(4294967295.0_f64);
    target.base_color_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    target.emissive = (opts.as_ref().and_then(|value| value.emissive)).unwrap_or(255.0_f64);
    target.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    target.emissive_strength =
        (opts.as_ref().and_then(|value| value.emissive_strength)).unwrap_or(1.0_f64);
    target.metallic = (opts.as_ref().and_then(|value| value.metallic)).unwrap_or(0.0_f64);
    target.metallic_roughness_map = opts
        .as_ref()
        .and_then(|value| (value.metallic_roughness_map).clone());
    target.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    target.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
    target.occlusion_map = opts
        .as_ref()
        .and_then(|value| (value.occlusion_map).clone());
    target.occlusion_strength =
        (opts.as_ref().and_then(|value| value.occlusion_strength)).unwrap_or(1.0_f64);
    target.roughness = (opts.as_ref().and_then(|value| value.roughness)).unwrap_or(1.0_f64);
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:156 (sha256:d73291d2c9a128374c2e52e8713d2f8ac4c87c45e28969c9f1bba7dfb19b8913)
fn linear_channel_to_srgb8(value: f64) -> f64 {
    let srgb = if (value <= 0.0031308_f64) {
        (value * 12.92_f64)
    } else {
        ((1.055_f64 * (value).powf((1.0_f64 / 2.4_f64))) - 0.055_f64)
    };
    return ((1.0_f64).min((0.0_f64).max(srgb)) * 255.0_f64).round();
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:162 (sha256:090c9090fa5ea7ec2b41bb45dacd19e1a31b5efb978c884fc7e7da324a5d6a34)
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

// Source: upstream/packages/materials/src/pbrMaterials.ts:172 (sha256:4de967a9e344614ed886cb790ba6f585de1c2d42f051926890b3b7de517eb6b7)
static SCRATCH_LINEAR: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/materials/src/pbrMaterials.ts:173 (sha256:94db8fa211f564f31fb60b498857c9abf1208978aebee1c61cbe90a5cacc0440)
static SCRATCH_LINEAR2: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/materials/src/pbrMaterials.ts:178 (sha256:cd2b81e85530e0aa669c63d6b02baac68229302b0c8bdbcec273366d62ebbd0c)
pub fn report_material_conversion_drop(
    explanation: &MaterialConversionExplanation,
    conversion: String,
) -> () {
    {
        let __flight_callback = (*_CONVERSION_GUARD.lock().unwrap()).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()((*explanation).clone(), (conversion).clone()))
    };
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:188 (sha256:ebda4b9d23ad5a59195b6cbb219bbeb05346b68852352e44ccea060e745ed7fe)
pub fn set_material_conversion_guard(guard: &Option<MaterialConversionGuard>) -> () {
    (*_CONVERSION_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/materials/src/pbrMaterials.ts:192 (sha256:59de46673a923246e8ce3d61d2d93015e3c56e2a9f237a48386f4b9921cbeeb5)
static _CONVERSION_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<MaterialConversionGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
