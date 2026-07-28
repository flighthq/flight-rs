// @generated from upstream/packages/materials/src/materialValidation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::StandardPbrMaterialProperties;

// Source: upstream/packages/materials/src/materialValidation.ts:13 (sha256:a629417fa9f6cce6af936c84991395ca43049bdde636347b7cf5fd3e63a43fab)
pub fn clamp_standard_pbr_material_properties(
    out: &mut StandardPbrMaterialProperties,
) -> StandardPbrMaterialProperties {
    out.metallic = (1.0_f64).min((0.0_f64).max(out.metallic));
    out.roughness = (1.0_f64).min((0.0_f64).max(out.roughness));
    out.occlusion_strength = (1.0_f64).min((0.0_f64).max(out.occlusion_strength));
    out.emissive_strength = (0.0_f64).max(out.emissive_strength);
    out.normal_scale = (0.0_f64).max(out.normal_scale);
    return out.clone();
}

// Source: upstream/packages/materials/src/materialValidation.ts:25 (sha256:370ac0c946cca6df2c2809ff27858af471ca7b7e86ff1b8cb292962324d09c57)
pub fn is_valid_material_clearcoat(value: f64) -> bool {
    return (((value).is_finite() && (value >= 0.0_f64)) && (value <= 1.0_f64));
}

// Source: upstream/packages/materials/src/materialValidation.ts:33 (sha256:65ab4da3d937d34fc86d980d1f3e8e070835b8728a78832968707dbfd1a7f3da)
pub fn is_valid_material_ior(value: f64) -> bool {
    return (((value).is_finite() && (value >= MIN_MATERIAL_IOR)) && (value <= MAX_MATERIAL_IOR));
}

// Source: upstream/packages/materials/src/materialValidation.ts:41 (sha256:539a8c3a88f6ce0c9dba571144cbb1ce7f48e39089a43b9827579bdf6f3c2d60)
pub fn is_valid_material_iridescence_thickness(value: f64) -> bool {
    return ((value).is_finite() && (value >= 0.0_f64));
}

// Source: upstream/packages/materials/src/materialValidation.ts:48 (sha256:52740263409b505dadd4bb2492c68838a36f9df9e15f3f6582c72b1e551bf709)
pub fn is_valid_material_weight(value: f64) -> bool {
    return (((value).is_finite() && (value >= 0.0_f64)) && (value <= 1.0_f64));
}

// Source: upstream/packages/materials/src/materialValidation.ts:55 (sha256:7d1c1d11d68505fd1b55e53866b0b039012aad2cb49b58f092501eb533c0798a)
const MIN_MATERIAL_IOR: f64 = 1.0_f64;

// Source: upstream/packages/materials/src/materialValidation.ts:58 (sha256:3531603a103535b856c15d96342320e2dd0b2b9d2fd4bddad1074bebd800977a)
const MAX_MATERIAL_IOR: f64 = 5.0_f64;
