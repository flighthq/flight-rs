// @generated from upstream/packages/materials/src/classicMaterials.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_types::{
    BLINN_PHONG_MATERIAL_KIND as blinn_phong_material_kind_constant, BlinnPhongMaterial,
    LAMBERT_MATERIAL_KIND as lambert_material_kind_constant, LambertMaterial,
    PHONG_MATERIAL_KIND as phong_material_kind_constant, PhongMaterial,
};

// Source: upstream/packages/materials/src/classicMaterials.ts:8 (sha256:465125f0f65bd8d60cdbb44359798c0be38471fd06378e26593f367339c9219d)
pub fn create_blinn_phong_material(opts: Option<BlinnPhongMaterial>) -> BlinnPhongMaterial {
    let mut material = create_surface_material(blinn_phong_material_kind_constant);
    material.diffuse = (opts.as_ref().map(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().map(|value| value.normal_scale)).unwrap_or(1.0_f64);
    material.shininess = (opts.as_ref().map(|value| value.shininess)).unwrap_or(32.0_f64);
    material.specular = (opts.as_ref().map(|value| value.specular)).unwrap_or(4294967295.0_f64);
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    return (material).clone();
}

// Source: upstream/packages/materials/src/classicMaterials.ts:22 (sha256:1d01534542a98e1b110f231cd83b84ee5e8058efb4b123aa4ad89f769eb40a18)
pub fn create_lambert_material(opts: Option<LambertMaterial>) -> LambertMaterial {
    let mut material = create_surface_material(lambert_material_kind_constant);
    material.diffuse = (opts.as_ref().map(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.emissive = (opts.as_ref().map(|value| value.emissive)).unwrap_or(255.0_f64);
    material.emissive_map = opts.as_ref().and_then(|value| (value.emissive_map).clone());
    return (material).clone();
}

// Source: upstream/packages/materials/src/classicMaterials.ts:33 (sha256:6ed4897276572afa6ed6a4061d8280e35bfb8d1a1a44f6fa8990ce076296a40b)
pub fn create_phong_material(opts: Option<PhongMaterial>) -> PhongMaterial {
    let mut material = create_surface_material(phong_material_kind_constant);
    material.diffuse = (opts.as_ref().map(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = opts.as_ref().and_then(|value| (value.diffuse_map).clone());
    material.normal_map = opts.as_ref().and_then(|value| (value.normal_map).clone());
    material.normal_scale = (opts.as_ref().map(|value| value.normal_scale)).unwrap_or(1.0_f64);
    material.shininess = (opts.as_ref().map(|value| value.shininess)).unwrap_or(32.0_f64);
    material.specular = (opts.as_ref().map(|value| value.specular)).unwrap_or(4294967295.0_f64);
    material.specular_map = opts.as_ref().and_then(|value| (value.specular_map).clone());
    return (material).clone();
}
