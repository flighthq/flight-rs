// @generated from upstream/packages/types/src/SheenPbrMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AlphaType, BlendMode, EntityRuntime, Kind, MaterialAlphaMode, StandardPbrMaterialProperties,
    Texture,
};

// Source: upstream/packages/types/src/SheenPbrMaterial.ts:8 (sha256:bf8bbb2a4eae05ef9b0a9f4af3008d76ff6d0dd95b91491391b39256a66f9f5f)
#[derive(Clone, Default)]
pub struct SheenPbrMaterial {
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
    pub sheen_color: f64,
    pub sheen_color_map: Option<Texture>,
    pub sheen_roughness: f64,
    pub sheen_roughness_map: Option<Texture>,
    pub standard: StandardPbrMaterialProperties,
}
impl PartialEq for SheenPbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SheenPbrMaterial {
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

// Source: upstream/packages/types/src/SheenPbrMaterial.ts:16 (sha256:f86a4ea6c9bc13fd4076726140efea67e23c54b89baf5b97da03ab1931e8dcb7)
pub const SHEEN_PBR_MATERIAL_KIND: &'static str = "SheenPbrMaterial";
