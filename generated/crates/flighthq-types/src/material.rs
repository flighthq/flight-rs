// @generated from upstream/packages/types/src/Material.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, MaterialAlphaMode, PbrExtension, StandardPbrMaterialProperties, Texture};
use crate::{EntityRuntime, Kind};

// Source: upstream/packages/types/src/Material.ts:17 (sha256:a4c315d8130136ab5ad5c38e12d1cebca5388d52af388fbef9c000e2da59083d)
#[derive(Clone, Default)]
pub struct Material {
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
impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Material {
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

// Source: upstream/packages/types/src/Material.ts:28 (sha256:2d9bc41a7f5dd0cf2749211f70bf48ac24a704328b0b4dcdf6886be10129a32a)
pub type MaterialLike = Material;

// Source: upstream/packages/types/src/Material.ts:33 (sha256:b9465a69e946e60e8c2ea2640de9071ff5e1c9af91cf72bebbadcf2518aeb9d3)
pub type MaterialData = crate::OpaqueHostValue;
