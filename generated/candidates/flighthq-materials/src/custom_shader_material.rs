// @generated from upstream/packages/materials/src/customShaderMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_surface_material;
use flighthq_types::{
    CUSTOM_SHADER_MATERIAL_KIND as custom_shader_material_kind_constant, CustomShaderMaterial,
};

// Source: upstream/packages/materials/src/customShaderMaterial.ts:11 (sha256:a643450772789ace081e49e752691665df01a1db56b29a85c016fed23fb5a3ad)
pub fn create_custom_shader_material(opts: Option<CustomShaderMaterial>) -> CustomShaderMaterial {
    let mut material = create_surface_material(custom_shader_material_kind_constant);
    material.shader_key =
        (opts.as_ref().map(|value| (value.shader_key).clone())).unwrap_or("".to_owned());
    material.textures = opts.as_ref().and_then(|value| (value.textures).clone());
    material.uniforms = opts.as_ref().and_then(|value| (value.uniforms).clone());
    return (material).clone();
}
