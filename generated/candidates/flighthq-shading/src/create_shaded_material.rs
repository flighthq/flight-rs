// @generated from upstream/packages/shading/src/createShadedMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    BLEND_MODE as blend_mode_constant, SHADED_MATERIAL_KIND as shaded_material_kind_constant,
    ShadedMaterial, ShadedMaterialOptions,
};

// Source: upstream/packages/shading/src/createShadedMaterial.ts:19 (sha256:9cb1bf87a0c01e95e7f6ab9c37a803f52f33b5e44937f631cf89c9c22015f166)
pub fn create_shaded_material(options: Option<ShadedMaterialOptions>) -> ShadedMaterial {
    let mut material = create_entity(Some(ShadedMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        kind: (shaded_material_kind_constant).to_owned(),
        diffuse: Default::default(),
        diffuse_map: Default::default(),
        modifiers: Default::default(),
        normal_map: Default::default(),
        normal_scale: Default::default(),
        shininess: Default::default(),
        specular: Default::default(),
        specular_map: Default::default(),
    }));
    material.alpha_cutoff =
        (options.as_ref().and_then(|value| value.alpha_cutoff)).unwrap_or(0.5_f64);
    material.alpha_mode = (options
        .as_ref()
        .and_then(|value| (value.alpha_mode).clone()))
    .unwrap_or("opaque".to_owned());
    material.blend_mode = (options
        .as_ref()
        .and_then(|value| (value.blend_mode).clone()))
    .unwrap_or((blend_mode_constant.normal).clone());
    material.diffuse =
        (options.as_ref().and_then(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = options
        .as_ref()
        .and_then(|value| (value.diffuse_map).clone());
    material.double_sided =
        (options.as_ref().and_then(|value| value.double_sided)).unwrap_or(false);
    material.modifiers =
        (options.as_ref().and_then(|value| (value.modifiers).clone())).unwrap_or(vec![]);
    material.normal_map = options
        .as_ref()
        .and_then(|value| (value.normal_map).clone());
    material.normal_scale =
        (options.as_ref().and_then(|value| value.normal_scale)).unwrap_or(1.0_f64);
    material.shininess = (options.as_ref().and_then(|value| value.shininess)).unwrap_or(32.0_f64);
    material.specular =
        (options.as_ref().and_then(|value| value.specular)).unwrap_or(4294967295.0_f64);
    material.specular_map = options
        .as_ref()
        .and_then(|value| (value.specular_map).clone());
    return material;
}
