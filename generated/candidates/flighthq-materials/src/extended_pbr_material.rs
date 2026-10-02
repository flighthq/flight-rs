// @generated from upstream/packages/materials/src/extendedPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_material3_d, create_standard_pbr_material_properties};
use flighthq_types::{
    BlendMode, EXTENDED_PBR_MATERIAL_KIND as extended_pbr_material_kind_constant,
    ExtendedPbrMaterial, Kind, Material3DOptions, MaterialAlphaMode, Modifier, PbrExtension,
    StandardPbrMaterialProperties, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1774687694 {
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
impl PartialEq for FlightPartialRecord1774687694 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/extendedPbrMaterial.ts:9 (sha256:3b96895a235a23e00cf43b7bef90397b2bdd2d30fe2c118c7301630ac6f9a21a)
pub fn create_extended_pbr_material(
    opts: Option<FlightPartialRecord1774687694>,
) -> ExtendedPbrMaterial {
    let mut material = {
        let __flight_source = &(create_material3_d(
            (extended_pbr_material_kind_constant).to_owned(),
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
        ExtendedPbrMaterial {
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
    material.extensions =
        (opts.as_ref().and_then(|value| (value.extensions).clone())).unwrap_or(vec![]);
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .unwrap_or(create_standard_pbr_material_properties(None));
    return material;
}
