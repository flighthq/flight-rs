// @generated from upstream/packages/types/src/MatcapMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, EntityRuntime, Kind, MaterialAlphaMode, Texture};
use crate::{PbrExtension, StandardPbrMaterialProperties};

// Source: upstream/packages/types/src/MatcapMaterial.ts:7 (sha256:8b4467696325ff69cdbe538077a2259a9bd9f7007df08e3a49a165bf267de4e8)
#[derive(Clone, Default)]
pub struct MatcapMaterial {
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
    pub matcap: Option<Texture>,
    pub tint: f64,
}
impl PartialEq for MatcapMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MatcapMaterial {
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

// Source: upstream/packages/types/src/MatcapMaterial.ts:12 (sha256:69bf12a40048345093a5b1586225cc07af7a37b16ec5fa67fd999291d501bfaa)
pub const MATCAP_MATERIAL_KIND: &'static str = "MatcapMaterial";
