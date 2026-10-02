// @generated from upstream/packages/materials/src/classicMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_material3_d;
use flighthq_types::{
    BLINN_PHONG_MATERIAL_KIND as blinn_phong_material_kind_constant, BlendMode, BlinnPhongMaterial,
    Kind, LAMBERT_MATERIAL_KIND as lambert_material_kind_constant, LambertMaterial,
    Material3DOptions, MaterialAlphaMode, Modifier,
    PHONG_MATERIAL_KIND as phong_material_kind_constant, PbrExtension, PhongMaterial,
    StandardPbrMaterialProperties, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord2973490600 {
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
impl PartialEq for FlightPartialRecord2973490600 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord94679780 {
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
impl PartialEq for FlightPartialRecord94679780 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord508692455 {
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
impl PartialEq for FlightPartialRecord508692455 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/classicMaterials.ts:8 (sha256:b6cd10f7e6913ab85075a7562ba8dca68df404c65264af89179dae6e64929162)
pub fn create_blinn_phong_material(
    opts: Option<FlightPartialRecord2973490600>,
) -> BlinnPhongMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (blinn_phong_material_kind_constant).to_owned(),
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
        BlinnPhongMaterial {
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
    material.alpha_map = opts.as_ref().and_then(|value| (value.alpha_map).clone());
    material.diffuse = (opts.as_ref().and_then(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
    material.shininess = (opts.as_ref().and_then(|value| value.shininess)).unwrap_or(32.0_f64);
    material.specular =
        (opts.as_ref().and_then(|value| value.specular)).unwrap_or(4294967295.0_f64);
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/classicMaterials.ts:23 (sha256:a6c7e268d2191456783251df8d6baee2ba2bd1839bf8aab38e5154a130c869bd)
pub fn create_lambert_material(opts: Option<FlightPartialRecord94679780>) -> LambertMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (lambert_material_kind_constant).to_owned(),
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
        LambertMaterial {
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
    return material;
}

// Source: upstream/packages/materials/src/classicMaterials.ts:34 (sha256:9b46bfbd9f732d57861d26dfcd9a0d2ffdbcca39d35d68e7de7eb8b955bf06bd)
pub fn create_phong_material(opts: Option<FlightPartialRecord508692455>) -> PhongMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (phong_material_kind_constant).to_owned(),
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
        PhongMaterial {
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
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
    material.shininess = (opts.as_ref().and_then(|value| value.shininess)).unwrap_or(32.0_f64);
    material.specular =
        (opts.as_ref().and_then(|value| value.specular)).unwrap_or(4294967295.0_f64);
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    return material;
}
