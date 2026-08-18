// @generated from upstream/packages/types/src/LambertMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, EntityRuntime, Kind, MaterialAlphaMode, Texture};
use crate::{PbrExtension, StandardPbrMaterialProperties};

// Source: upstream/packages/types/src/LambertMaterial.ts:6 (sha256:2d0f9e3abe6c598f4700dd1dcfca1aa30bc6c7bff1a53bd74e2cbd646349b722)
#[derive(Clone, Default)]
pub struct LambertMaterial {
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
    pub diffuse: f64,
    pub diffuse_map: Option<Texture>,
    pub emissive: f64,
    pub emissive_map: Option<Texture>,
}
impl PartialEq for LambertMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for LambertMaterial {
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

// Source: upstream/packages/types/src/LambertMaterial.ts:13 (sha256:ca40c337cba89ac196ea3e987c08e86ffd294bfabf9d8540583a479d14216478)
pub const LAMBERT_MATERIAL_KIND: &'static str = "LambertMaterial";
