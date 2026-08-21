// @generated from upstream/packages/materials/src/surfaceMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_material;
use flighthq_types::{
    BLEND_MODE as blend_mode_constant, Kind, MaterialAlphaMode, SurfaceMaterial,
    SurfaceMaterialOptions,
};

// Source: upstream/packages/materials/src/surfaceMaterial.ts:12 (sha256:8a8835ae1dc2d48b74c9d12063eb57902d8f884b1a6f7a939f3fea6f1b32ed33)
pub fn create_surface_material(
    kind: Kind,
    opts: Option<SurfaceMaterialOptions>,
) -> SurfaceMaterial {
    let mut material = {
        let __flight_source = &(create_material((kind).clone()));
        SurfaceMaterial {
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
            ..Default::default()
        }
    };
    material.alpha_cutoff = (opts.as_ref().and_then(|value| value.alpha_cutoff))
        .clone()
        .unwrap_or(DEFAULT_ALPHA_CUTOFF);
    material.alpha_mode = (opts.as_ref().and_then(|value| (value.alpha_mode).clone()))
        .clone()
        .unwrap_or(((DEFAULT_ALPHA_MODE).clone()).to_owned());
    material.blend_mode = (opts.as_ref().and_then(|value| (value.blend_mode).clone()))
        .clone()
        .unwrap_or((blend_mode_constant.normal).clone());
    material.double_sided = (opts.as_ref().and_then(|value| value.double_sided))
        .clone()
        .unwrap_or(DEFAULT_DOUBLE_SIDED);
    return material;
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:24 (sha256:252591eb40d6c2c44df83b9adb139587d884def792ffde9c4701d6fe83bccdb8)
pub fn get_surface_material_alpha_mode(source: &SurfaceMaterial) -> MaterialAlphaMode {
    return (source.alpha_mode).clone();
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:30 (sha256:0ca31e8419a91c96dc7a3cdbdfa32351fb7674daf232167039eb4a6df8d62d41)
pub fn is_surface_material_blended(source: &SurfaceMaterial) -> bool {
    return ((source.alpha_mode).clone() == "blend");
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:36 (sha256:eecd176a1371fe2deae850ee3bd786c415b4b40acde835e12b08b6e2d75b25d0)
pub fn is_surface_material_masked(source: &SurfaceMaterial) -> bool {
    return ((source.alpha_mode).clone() == "mask");
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:42 (sha256:87f5428ef0672ff57424c64b79545f15fbae2eb80ccc44baf5d9d74ea738cf5b)
pub fn is_surface_material_opaque(source: &SurfaceMaterial) -> bool {
    return ((source.alpha_mode).clone() == "opaque");
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:46 (sha256:34e90c18bc19a9bc5f2510d91751864751ac7980e8301885b96e60f58aef72a0)
const DEFAULT_ALPHA_CUTOFF: f64 = 0.5_f64;

// Source: upstream/packages/materials/src/surfaceMaterial.ts:47 (sha256:401716d5b49444f6ce555a73a1459772cc9cab019086d5c70ea11ac13e42b06e)
const DEFAULT_ALPHA_MODE: &'static str = "opaque";

// Source: upstream/packages/materials/src/surfaceMaterial.ts:48 (sha256:59abc25abd5a8cbe5fa6822845633843c57e374dba44d086d0760fb0d39cd176)
const DEFAULT_DOUBLE_SIDED: bool = false;
