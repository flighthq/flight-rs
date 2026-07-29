// @generated from upstream/packages/shading/src/createShadedMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    BLEND_MODE as blend_mode_constant, Modifier,
    SHADED_MATERIAL_KIND as shaded_material_kind_constant, ShadedMaterial, Texture,
};

// Source: upstream/packages/shading/src/createShadedMaterial.ts:8 (sha256:f71dbd0a2b4cfce366e992aeb42e4ce4599613e926c1659024b07c9d68c34828)
#[derive(Clone, Default)]
pub struct ShadedMaterialOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub diffuse: Option<f64>,
    pub diffuse_map: Option<Texture>,
    pub modifiers: Option<Vec<Modifier>>,
    pub normal_map: Option<Texture>,
    pub normal_scale: Option<f64>,
    pub shininess: Option<f64>,
    pub specular: Option<f64>,
    pub specular_map: Option<Texture>,
}
impl PartialEq for ShadedMaterialOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/createShadedMaterial.ts:27 (sha256:d00c6253743561df4dce9ba92a6ecb2776dc307c059320d31aad08eef294a5b3)
pub fn create_shaded_material(options: Option<ShadedMaterialOptions>) -> ShadedMaterial {
    let mut material = create_entity(Some(ShadedMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        kind: (shaded_material_kind_constant).to_owned(),
    }));
    material.alpha_cutoff = 0.5_f64;
    material.alpha_mode = "opaque".to_owned();
    material.alpha_type = "straight".to_owned();
    material.blend_mode = (blend_mode_constant.normal).clone();
    material.diffuse =
        (options.as_ref().and_then(|value| value.diffuse)).unwrap_or(4294967295.0_f64);
    material.diffuse_map = options
        .as_ref()
        .and_then(|value| (value.diffuse_map).clone());
    material.double_sided = false;
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
