// @generated from upstream/packages/materials/src/unlitMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_types::{
    DEPTH_MATERIAL_KIND as depth_material_kind_constant, DepthMaterial,
    EMISSIVE_MATERIAL_KIND as emissive_material_kind_constant, EmissiveMaterial,
    MATCAP_MATERIAL_KIND as matcap_material_kind_constant, MatcapMaterial,
    NORMAL_MATERIAL_KIND as normal_material_kind_constant, NormalMaterial,
    TOON_MATERIAL_KIND as toon_material_kind_constant, ToonMaterial,
    UNLIT_MATERIAL_KIND as unlit_material_kind_constant, UnlitMaterial,
    VERTEX_COLOR_MATERIAL_KIND as vertex_color_material_kind_constant, VertexColorMaterial,
    WIREFRAME_MATERIAL_KIND as wireframe_material_kind_constant, WireframeMaterial,
};

// Source: upstream/packages/materials/src/unlitMaterials.ts:26 (sha256:34e40d15213e01b97927051a51e96da02959c327449e350224f2521f69bdce49)
pub fn create_depth_material(opts: Option<DepthMaterial>) -> DepthMaterial {
    let mut material = create_surface_material(depth_material_kind_constant);
    material.far = (opts.as_ref().map(|value| value.far)).unwrap_or(1.0_f64);
    material.near = (opts.as_ref().map(|value| value.near)).unwrap_or(0.0_f64);
    return (material).clone();
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:35 (sha256:18241b4bef5b483d55375d88d0162a95f19ec7bcc6ab24ba11e7a7468e4b5bdf)
pub fn create_emissive_material(opts: Option<EmissiveMaterial>) -> EmissiveMaterial {
    let mut material = create_surface_material(emissive_material_kind_constant);
    material.emissive = (opts.as_ref().map(|value| value.emissive)).unwrap_or(4294967295.0_f64);
    material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    material.emissive_strength =
        (opts.as_ref().map(|value| value.emissive_strength)).unwrap_or(1.0_f64);
    return (material).clone();
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:45 (sha256:34d3547a83d42be7997ef6799f6720ac9f3ff69a23e048835e11c8f9ae91b156)
pub fn create_matcap_material(opts: Option<MatcapMaterial>) -> MatcapMaterial {
    let mut material = create_surface_material(matcap_material_kind_constant);
    material.matcap = opts.as_ref().and_then(|value| (value.matcap).clone());
    material.tint = (opts.as_ref().map(|value| value.tint)).unwrap_or(4294967295.0_f64);
    return (material).clone();
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:53 (sha256:62f7f16d7142038697df95138c373dd1065ad35f45e402f2e76b8b1de5c632b4)
pub fn create_normal_material(opts: Option<NormalMaterial>) -> NormalMaterial {
    let mut material = create_surface_material(normal_material_kind_constant);
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().map(|value| value.normal_scale)).unwrap_or(1.0_f64);
    return (material).clone();
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:62 (sha256:c2fc522f43aad8fb353ec6ee4f6c78fb1b636aabfa745124b1231541da9ebb94)
pub fn create_toon_material(opts: Option<ToonMaterial>) -> ToonMaterial {
    let mut material = create_surface_material(toon_material_kind_constant);
    material.base_color = (opts.as_ref().map(|value| value.base_color)).unwrap_or(4294967295.0_f64);
    material.base_color_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    material.ramp = opts.as_ref().and_then(|value| (value.ramp).clone());
    material.steps = (opts.as_ref().map(|value| value.steps)).unwrap_or(3.0_f64);
    return (material).clone();
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:74 (sha256:4b2356314f49fa6c7437772c60cf86ca43b9b16573be1509ef57bd34a2bfaaaf)
pub fn create_unlit_material(opts: Option<UnlitMaterial>) -> UnlitMaterial {
    let mut material = create_surface_material(unlit_material_kind_constant);
    material.base_color = (opts.as_ref().map(|value| value.base_color)).unwrap_or(4294967295.0_f64);
    material.base_color_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    material.base_color_video_map = opts
        .as_ref()
        .and_then(|value| (value.base_color_video_map).clone());
    return (material).clone();
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:83 (sha256:dd3c306bc0e5a221a397f34997e427a1f3e5d5f4554e8d40fc84396a7e430019)
pub fn create_vertex_color_material(opts: Option<VertexColorMaterial>) -> VertexColorMaterial {
    let mut material = create_surface_material(vertex_color_material_kind_constant);
    material.tint = (opts.as_ref().map(|value| value.tint)).unwrap_or(4294967295.0_f64);
    return (material).clone();
}

// Source: upstream/packages/materials/src/unlitMaterials.ts:90 (sha256:c0b1052149897b973d4907f19ebec29b0b53afa271e841e081eb6c2fda04ec82)
pub fn create_wireframe_material(opts: Option<WireframeMaterial>) -> WireframeMaterial {
    let mut material = create_surface_material(wireframe_material_kind_constant);
    material.color = (opts.as_ref().map(|value| value.color)).unwrap_or(4294967295.0_f64);
    material.thickness = (opts.as_ref().map(|value| value.thickness)).unwrap_or(1.0_f64);
    return (material).clone();
}
