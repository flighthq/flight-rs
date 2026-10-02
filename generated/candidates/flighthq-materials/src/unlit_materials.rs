// @generated from upstream/packages/materials/src/unlitMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_material3_d;
use flighthq_types::{
    BlendMode, DEPTH_MATERIAL_KIND as depth_material_kind_constant, DepthMaterial,
    EMISSIVE_MATERIAL_KIND as emissive_material_kind_constant, EmissiveMaterial, Kind,
    MATCAP_MATERIAL_KIND as matcap_material_kind_constant, MatcapMaterial, Material3DOptions,
    MaterialAlphaMode, Modifier, NORMAL_MATERIAL_KIND as normal_material_kind_constant,
    NormalMaterial, PbrExtension, StandardPbrMaterialProperties,
    TOON_MATERIAL_KIND as toon_material_kind_constant, Texture, ToonMaterial,
    UNLIT_MATERIAL_KIND as unlit_material_kind_constant, UnlitMaterial,
    VERTEX_COLOR_MATERIAL_KIND as vertex_color_material_kind_constant, VertexColorMaterial,
    WIREFRAME_MATERIAL_KIND as wireframe_material_kind_constant, WireframeMaterial,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord2204233292 {
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
impl PartialEq for FlightPartialRecord2204233292 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1548468902 {
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
impl PartialEq for FlightPartialRecord1548468902 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1816662963 {
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
impl PartialEq for FlightPartialRecord1816662963 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1184324530 {
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
impl PartialEq for FlightPartialRecord1184324530 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3388180413 {
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
impl PartialEq for FlightPartialRecord3388180413 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2491192661 {
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
impl PartialEq for FlightPartialRecord2491192661 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord755745112 {
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
impl PartialEq for FlightPartialRecord755745112 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2134117391 {
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
impl PartialEq for FlightPartialRecord2134117391 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:26 (sha256:47d5f58d078853392bfee9d4f90372e0e49f2d8387c7dfcab254ee87db303ebd)
pub fn create_depth_material(opts: Option<FlightPartialRecord2204233292>) -> DepthMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (depth_material_kind_constant).to_owned(),
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
        DepthMaterial {
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
    material.far = (opts.as_ref().and_then(|value| value.far)).unwrap_or(1.0_f64);
    material.near = (opts.as_ref().and_then(|value| value.near)).unwrap_or(0.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:35 (sha256:8b954cc9a2b77d66c56efed4a4fae6b5e6eeeca22833dc5670259a9a1b83139b)
pub fn create_emissive_material(opts: Option<FlightPartialRecord1548468902>) -> EmissiveMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (emissive_material_kind_constant).to_owned(),
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
        EmissiveMaterial {
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
    material.emissive =
        (opts.as_ref().and_then(|value| value.emissive)).unwrap_or(4294967295.0_f64);
    material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    material.emissive_strength =
        (opts.as_ref().and_then(|value| value.emissive_strength)).unwrap_or(1.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:45 (sha256:3faddae5ac4a54962510fc53a3112bd956bc320cb77a979781c12552365ba8f4)
pub fn create_matcap_material(opts: Option<FlightPartialRecord1816662963>) -> MatcapMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (matcap_material_kind_constant).to_owned(),
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
        MatcapMaterial {
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
    material.matcap = opts.as_ref().and_then(|value| (value.matcap).clone());
    material.tint = (opts.as_ref().and_then(|value| value.tint)).unwrap_or(4294967295.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:53 (sha256:747d71b8183dba110b6158e360ff9c3d0273a0a6f894e8214e8fef0b8992ea83)
pub fn create_normal_material(opts: Option<FlightPartialRecord1184324530>) -> NormalMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (normal_material_kind_constant).to_owned(),
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
        NormalMaterial {
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
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:62 (sha256:02351e44d3a1be562ba62d5cdfe8d4a0fdff296769e231202bf82124448d1ab5)
pub fn create_toon_material(opts: Option<FlightPartialRecord3388180413>) -> ToonMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (toon_material_kind_constant).to_owned(),
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
        ToonMaterial {
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
    material.base_color =
        (opts.as_ref().and_then(|value| value.base_color)).unwrap_or(4294967295.0_f64);
    material.base_color_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    material.ramp = opts.as_ref().and_then(|value| (value.ramp).clone());
    material.steps = (opts.as_ref().and_then(|value| value.steps)).unwrap_or(3.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:73 (sha256:09d63e169909b9513c4a2d38ca3629a8a4871849b336192f6b9b88dda893a8b8)
pub fn create_unlit_material(opts: Option<FlightPartialRecord2491192661>) -> UnlitMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (unlit_material_kind_constant).to_owned(),
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
        UnlitMaterial {
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
    material.base_color =
        (opts.as_ref().and_then(|value| value.base_color)).unwrap_or(4294967295.0_f64);
    material.base_color_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:81 (sha256:67e887f65f16e146bb020dd8d4ee62aa13b8cd2aa2c3ca823646525697cdc5ac)
pub fn create_vertex_color_material(
    opts: Option<FlightPartialRecord755745112>,
) -> VertexColorMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (vertex_color_material_kind_constant).to_owned(),
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
        VertexColorMaterial {
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
    material.tint = (opts.as_ref().and_then(|value| value.tint)).unwrap_or(4294967295.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:88 (sha256:0079edd4d88bca28bfcbe6c88b05d75b93c8d3dc24080cbc0cd285d9d17c8c42)
pub fn create_wireframe_material(opts: Option<FlightPartialRecord2134117391>) -> WireframeMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (wireframe_material_kind_constant).to_owned(),
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
        WireframeMaterial {
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
    material.color = (opts.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64);
    material.thickness = (opts.as_ref().and_then(|value| value.thickness)).unwrap_or(1.0_f64);
    return material;
}
