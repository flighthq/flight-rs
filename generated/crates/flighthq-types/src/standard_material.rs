// @generated from upstream/packages/types/src/StandardMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, MaterialAlphaMode, PbrExtension, StandardPbrMaterialProperties, Texture};
use crate::{EntityRuntime, Kind};

// Source: upstream/packages/types/src/StandardMaterial.ts:6 (sha256:0858067e7a3bd9e65383d2aebaeec1e0bd188d0050a34613b912d577700c411d)
#[derive(Clone, Default)]
pub struct StandardMaterial {
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
}
impl PartialEq for StandardMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for StandardMaterial {
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

// Source: upstream/packages/types/src/StandardMaterial.ts:10 (sha256:3d90e4755aff134b66956fe35659b153e929c5017b20dec23aa55c025633b4d5)
pub const STANDARD_MATERIAL_KIND: &'static str = "StandardMaterial";

// Source: upstream/packages/types/src/StandardMaterial.ts:11 (sha256:01068b62ef49aca03763402438e943ba82f56877859f4c7850e5250569fdd08f)
pub type StandardMaterialKind = String;
