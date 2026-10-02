// @generated from upstream/packages/types/src/DepthMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, EntityRuntime, Kind, MaterialAlphaMode};
use crate::{Modifier, PbrExtension, StandardPbrMaterialProperties, Texture};

// Source: upstream/packages/types/src/DepthMaterial.ts:6 (sha256:4721bae6ff66d0d16214d5ef4bfac522e5eb27d40190de244e696b6c64e28a76)
#[derive(Clone, Default)]
pub struct DepthMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub color: f64,
    pub thickness: f64,
    pub tint: f64,
    pub base_color: f64,
    pub base_color_map: Option<Texture>,
    pub ramp: Option<Texture>,
    pub steps: f64,
    pub alpha_map: Option<Texture>,
    pub emissive: f64,
    pub emissive_map: Option<Texture>,
    pub emissive_strength: f64,
    pub metallic: f64,
    pub metallic_roughness_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub occlusion_map: Option<Texture>,
    pub occlusion_strength: f64,
    pub roughness: f64,
    pub diffuse: f64,
    pub diffuse_map: Option<Texture>,
    pub glossiness: f64,
    pub specular: f64,
    pub specular_glossiness_map: Option<Texture>,
    pub modifiers: Vec<Modifier>,
    pub shininess: f64,
    pub specular_map: Option<Texture>,
    pub matcap: Option<Texture>,
    pub extensions: Vec<PbrExtension>,
    pub standard: StandardPbrMaterialProperties,
    pub far: f64,
    pub near: f64,
    pub shader_key: String,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for DepthMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for DepthMaterial {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/DepthMaterial.ts:12 (sha256:c1194c7b3c562cb4f9279d00f877fb6fc7a9b4a810295bad4a58f1ef5fe562ce)
pub const DEPTH_MATERIAL_KIND: &'static str = "DepthMaterial";
