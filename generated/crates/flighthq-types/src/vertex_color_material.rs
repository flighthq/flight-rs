// @generated from upstream/packages/types/src/VertexColorMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, EntityRuntime, Kind, MaterialAlphaMode};
use crate::{Modifier, PbrExtension, StandardPbrMaterialProperties, Texture};

// Source: upstream/packages/types/src/VertexColorMaterial.ts:6 (sha256:02fd2910de6dcf4312b4eb943885dc121d206613c7975a6a535fdfbeaeec6fc6)
#[derive(Clone, Default)]
pub struct VertexColorMaterial {
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
impl PartialEq for VertexColorMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for VertexColorMaterial {
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

// Source: upstream/packages/types/src/VertexColorMaterial.ts:11 (sha256:663ec1ea4d52c1b7c256befac8cd831121c0c0bd0dc46b845b553ec773b8becd)
pub const VERTEX_COLOR_MATERIAL_KIND: &'static str = "VertexColorMaterial";
