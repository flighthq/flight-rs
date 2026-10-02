// @generated from upstream/packages/types/src/WgpuWireframePipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/WgpuWireframePipeline.ts:4 (sha256:a32c6690e3dcdc92088069a5cfce507767868fc512249bf6464bc4e23c1b2c1e)
#[derive(Clone, Default)]
pub struct WgpuWireframePipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
    pub skinned: bool,
}
impl PartialEq for WgpuWireframePipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuWireframePipeline {
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

// Source: upstream/packages/types/src/WgpuWireframePipeline.ts:11 (sha256:f47b684c999ca62a6410324b6f9fd4c19ecdfd4defb5ab69843eefc33182b1c0)
#[derive(Clone, Default)]
pub struct WgpuWireframeUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_format: crate::OpaqueHostValue,
    pub line_index_buffer: crate::OpaqueHostValue,
    pub version: f64,
    pub vertex_buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuWireframeUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
