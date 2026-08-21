// @generated from upstream/packages/materials/src/extendedPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_standard_pbr_material_properties, create_surface_material};
use flighthq_types::{
    BlendMode, EXTENDED_PBR_MATERIAL_KIND as extended_pbr_material_kind_constant,
    ExtendedPbrMaterial, Kind, MaterialAlphaMode, PbrExtension, StandardPbrMaterialProperties,
    SurfaceMaterialOptions, Texture,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord3120887473 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
    pub extensions: Option<Vec<PbrExtension>>,
    pub standard: Option<StandardPbrMaterialProperties>,
    pub shader_key: Option<String>,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for FlightPartialRecord3120887473 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/extendedPbrMaterial.ts:9 (sha256:83c50b1ac04d5b2764fe294c5958cc196749ab50dcbe357666f10f906f82b480)
pub fn create_extended_pbr_material(
    opts: Option<FlightPartialRecord3120887473>,
) -> ExtendedPbrMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (extended_pbr_material_kind_constant).to_owned(),
            ((opts).clone()).as_ref().map(|__flight_value| {
                let __flight_source = &(__flight_value);
                SurfaceMaterialOptions {
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
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            alpha_cutoff: __flight_source.alpha_cutoff,
            alpha_mode: (__flight_source.alpha_mode).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            ..Default::default()
        }
    };
    material.extensions = (opts.as_ref().and_then(|value| (value.extensions).clone()))
        .clone()
        .unwrap_or(vec![]);
    material.standard = (opts.as_ref().and_then(|value| (value.standard).clone()))
        .clone()
        .unwrap_or(create_standard_pbr_material_properties(None));
    return material;
}
