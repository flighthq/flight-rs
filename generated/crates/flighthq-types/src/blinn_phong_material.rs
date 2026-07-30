// @generated from upstream/packages/types/src/BlinnPhongMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, EntityRuntime, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/BlinnPhongMaterial.ts:8 (sha256:398af6a2a8e8ac3c8ebb3cb210ab657da96659a27250218c3a4773a5ed0e4c88)
#[derive(Clone, Default)]
pub struct BlinnPhongMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub shader_key: String,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
    pub diffuse: f64,
    pub diffuse_map: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
    pub shininess: f64,
    pub specular: f64,
    pub specular_map: Option<Texture>,
}
impl PartialEq for BlinnPhongMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for BlinnPhongMaterial {
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

// Source: upstream/packages/types/src/BlinnPhongMaterial.ts:18 (sha256:5c93195075cdac2ebe9f0994ed3bd2eed4cfdf7c368df0a78d8edb6e7bc30b22)
pub const BLINN_PHONG_MATERIAL_KIND: &'static str = "BlinnPhongMaterial";
