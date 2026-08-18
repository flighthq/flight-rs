// @generated from upstream/packages/types/src/StandardPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PbrExtension;
use crate::{BlendMode, EntityRuntime, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/StandardPbrMaterial.ts:11 (sha256:44fad9b5706a5df98cf0027a1603a725ef02feb70f58928c41700c9d56bd5de4)
#[derive(Clone, Default)]
pub struct StandardPbrMaterialProperties {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_map: Option<Texture>,
    pub base_color: f64,
    pub base_color_map: Option<Texture>,
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
}
impl PartialEq for StandardPbrMaterialProperties {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StandardPbrMaterial.ts:29 (sha256:75623596e21f7fa8bdb96972f77d790d3fa4eaa91a9a238efce37fd2c87cff25)
#[derive(Clone, Default)]
pub struct StandardPbrMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub extensions: Vec<PbrExtension>,
    pub standard: StandardPbrMaterialProperties,
    pub shader_key: String,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub alpha_map: Option<Texture>,
    pub base_color: f64,
    pub base_color_map: Option<Texture>,
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
}
impl PartialEq for StandardPbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for StandardPbrMaterial {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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

// Source: upstream/packages/types/src/StandardPbrMaterial.ts:31 (sha256:c10ef55cb5f965b373510f0a8bce10c727e60f301669267c100433d0a01cbd00)
pub const STANDARD_PBR_MATERIAL_KIND: &'static str = "StandardPbrMaterial";
