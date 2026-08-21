// @generated from upstream/packages/types/src/SheenPbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, PbrUvSet, Texture};

// Source: upstream/packages/types/src/SheenPbrExtension.ts:5 (sha256:035a1014631528e9aa9210a89a65d69e398026d9db46131285a1c87aeb2fda16)
#[derive(Clone, Default)]
pub struct SheenPbrExtension {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub kind: Kind,
    pub thickness: f64,
    pub thickness_map: Option<Texture>,
    pub thickness_map_uv_set: PbrUvSet,
    pub wrapped_diffuse_color: f64,
    pub wrapped_diffuse_map: Option<Texture>,
    pub wrapped_diffuse_map_uv_set: PbrUvSet,
    pub wrapped_diffuse_strength: f64,
    pub attenuation_color: f64,
    pub attenuation_distance: f64,
    pub ior: f64,
    pub transmission: f64,
    pub transmission_map: Option<Texture>,
    pub transmission_map_uv_set: PbrUvSet,
    pub specular: f64,
    pub specular_color: f64,
    pub specular_color_map: Option<Texture>,
    pub specular_color_map_uv_set: PbrUvSet,
    pub specular_map: Option<Texture>,
    pub specular_map_uv_set: PbrUvSet,
    pub sheen_color: f64,
    pub sheen_color_map: Option<Texture>,
    pub sheen_color_map_uv_set: PbrUvSet,
    pub sheen_roughness: f64,
    pub sheen_roughness_map: Option<Texture>,
    pub sheen_roughness_map_uv_set: PbrUvSet,
    pub iridescence: f64,
    pub iridescence_ior: f64,
    pub iridescence_map: Option<Texture>,
    pub iridescence_map_uv_set: PbrUvSet,
    pub iridescence_thickness_map: Option<Texture>,
    pub iridescence_thickness_map_uv_set: PbrUvSet,
    pub iridescence_thickness_max: f64,
    pub iridescence_thickness_min: f64,
    pub clearcoat: f64,
    pub clearcoat_map: Option<Texture>,
    pub clearcoat_map_uv_set: PbrUvSet,
    pub clearcoat_normal_map: Option<Texture>,
    pub clearcoat_normal_map_uv_set: PbrUvSet,
    pub clearcoat_normal_scale: f64,
    pub clearcoat_roughness: f64,
    pub clearcoat_roughness_map: Option<Texture>,
    pub clearcoat_roughness_map_uv_set: PbrUvSet,
    pub anisotropy_map: Option<Texture>,
    pub anisotropy_map_uv_set: PbrUvSet,
    pub anisotropy_rotation: f64,
    pub anisotropy_strength: f64,
}
impl PartialEq for SheenPbrExtension {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SheenPbrExtension {
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

// Source: upstream/packages/types/src/SheenPbrExtension.ts:16 (sha256:696370e90ed3c1333a6c81794436f1414c16f4c9944f638742ee77edf6a626c1)
pub const SHEEN_PBR_EXTENSION_KIND: &'static str = "SheenPbrExtension";
