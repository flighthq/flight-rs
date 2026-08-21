// @generated from upstream/packages/types/src/WrappedDiffusePbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, PbrUvSet, Texture};

// Source: upstream/packages/types/src/WrappedDiffusePbrExtension.ts:6 (sha256:58c73e60264700f5dcd433febea3ead0758d3e062b8cc68c0b35586324582a31)
#[derive(Clone, Default)]
pub struct WrappedDiffusePbrExtension {
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
impl PartialEq for WrappedDiffusePbrExtension {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WrappedDiffusePbrExtension {
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

// Source: upstream/packages/types/src/WrappedDiffusePbrExtension.ts:18 (sha256:0fff2775831eacaeb3c45f0d60026bd56e03d1cbb0b60cb604f03ee73d4de6cc)
pub const WRAPPED_DIFFUSE_PBR_EXTENSION_KIND: &'static str = "WrappedDiffusePbrExtension";
