// @generated from upstream/packages/materials/src/unlitMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_types::{
    BlendMode, DEPTH_MATERIAL_KIND as depth_material_kind_constant, DepthMaterial,
    EMISSIVE_MATERIAL_KIND as emissive_material_kind_constant, EmissiveMaterial, Kind,
    MATCAP_MATERIAL_KIND as matcap_material_kind_constant, MatcapMaterial, MaterialAlphaMode,
    NORMAL_MATERIAL_KIND as normal_material_kind_constant, NormalMaterial, PbrExtension,
    StandardPbrMaterialProperties, SurfaceMaterialOptions,
    TOON_MATERIAL_KIND as toon_material_kind_constant, Texture, ToonMaterial,
    UNLIT_MATERIAL_KIND as unlit_material_kind_constant, UnlitMaterial,
    VERTEX_COLOR_MATERIAL_KIND as vertex_color_material_kind_constant, VertexColorMaterial,
    WIREFRAME_MATERIAL_KIND as wireframe_material_kind_constant, WireframeMaterial,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord2049819932 {
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
    pub far: Option<f64>,
    pub near: Option<f64>,
}
impl PartialEq for FlightPartialRecord2049819932 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord567701514 {
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
    pub emissive: Option<f64>,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: Option<f64>,
}
impl PartialEq for FlightPartialRecord567701514 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2770153774 {
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
    pub matcap: Option<Texture>,
    pub tint: Option<f64>,
}
impl PartialEq for FlightPartialRecord2770153774 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3106176057 {
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
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
}
impl PartialEq for FlightPartialRecord3106176057 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord400840814 {
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
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
    pub ramp: Option<Texture>,
    pub steps: Option<f64>,
}
impl PartialEq for FlightPartialRecord400840814 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3145127941 {
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
    pub base_color: Option<f64>,
    pub base_color_map: Option<Texture>,
}
impl PartialEq for FlightPartialRecord3145127941 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1918310071 {
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
    pub tint: Option<f64>,
}
impl PartialEq for FlightPartialRecord1918310071 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1752344006 {
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
    pub color: Option<f64>,
    pub thickness: Option<f64>,
}
impl PartialEq for FlightPartialRecord1752344006 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:26 (sha256:c38339db46f51b317860c7a504b688e90dccf4cc61b108bb8b6bfa9a21f4f3bb)
pub fn create_depth_material(opts: Option<FlightPartialRecord2049819932>) -> DepthMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (depth_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            far: Default::default(),
            near: Default::default(),
        }
    };
    material.far = (opts.as_ref().and_then(|value| value.far)).unwrap_or(1.0_f64);
    material.near = (opts.as_ref().and_then(|value| value.near)).unwrap_or(0.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:35 (sha256:ed0aec227af5036bf364a28aa80dc7433ff939d1001082a2af7cc0bfaed350e1)
pub fn create_emissive_material(opts: Option<FlightPartialRecord567701514>) -> EmissiveMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (emissive_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            emissive: Default::default(),
            emissive_map: Default::default(),
            emissive_strength: Default::default(),
        }
    };
    material.emissive =
        (opts.as_ref().and_then(|value| value.emissive)).unwrap_or(4294967295.0_f64);
    material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    material.emissive_strength =
        (opts.as_ref().and_then(|value| value.emissive_strength)).unwrap_or(1.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:45 (sha256:0531de2489a7a2eca1014db69d5e003d8df05d0417d5025d69819482685f2491)
pub fn create_matcap_material(opts: Option<FlightPartialRecord2770153774>) -> MatcapMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (matcap_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            matcap: Default::default(),
            tint: Default::default(),
        }
    };
    material.matcap = opts.as_ref().and_then(|value| (value.matcap).clone());
    material.tint = (opts.as_ref().and_then(|value| value.tint)).unwrap_or(4294967295.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:53 (sha256:e7211f146429ec1d90d90ce6ecdb97186b0b140251e6749c306332d817907417)
pub fn create_normal_material(opts: Option<FlightPartialRecord3106176057>) -> NormalMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (normal_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            normal_map: Default::default(),
            normal_scale: Default::default(),
        }
    };
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:62 (sha256:dd6d654b600a2e6ae93e95bd5902c3e49bef4368a5e0834c1e1f0d695369e81f)
pub fn create_toon_material(opts: Option<FlightPartialRecord400840814>) -> ToonMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (toon_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            base_color: Default::default(),
            base_color_map: Default::default(),
            ramp: Default::default(),
            steps: Default::default(),
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

// Source: upstream/packages/materials/src/unlitMaterials.ts:73 (sha256:2c24e49644cdab0ff4918182f9e893fb01bf8639e7aaa179e362bb28f77d9856)
pub fn create_unlit_material(opts: Option<FlightPartialRecord3145127941>) -> UnlitMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (unlit_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            base_color: Default::default(),
            base_color_map: Default::default(),
        }
    };
    material.base_color =
        (opts.as_ref().and_then(|value| value.base_color)).unwrap_or(4294967295.0_f64);
    material.base_color_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:81 (sha256:6f4b3bc7670e0b8d3f422e04a9abae8d0f99481675bfa4cd8ee717b5274ae373)
pub fn create_vertex_color_material(
    opts: Option<FlightPartialRecord1918310071>,
) -> VertexColorMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (vertex_color_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            tint: Default::default(),
        }
    };
    material.tint = (opts.as_ref().and_then(|value| value.tint)).unwrap_or(4294967295.0_f64);
    return material;
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:88 (sha256:586a98808dbc865c2a6cb5a17a2684a54bb72b61168c7555cde2beae81a59b61)
pub fn create_wireframe_material(opts: Option<FlightPartialRecord1752344006>) -> WireframeMaterial {
    let mut material = {
        let __flight_source = &(create_surface_material(
            (wireframe_material_kind_constant).to_owned(),
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
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            color: Default::default(),
            thickness: Default::default(),
        }
    };
    material.color = (opts.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64);
    material.thickness = (opts.as_ref().and_then(|value| value.thickness)).unwrap_or(1.0_f64);
    return material;
}
