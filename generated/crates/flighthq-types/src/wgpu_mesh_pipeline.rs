// @generated from upstream/packages/types/src/WgpuMeshPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/WgpuMeshPipeline.ts:7 (sha256:b7fd093aaba2e92afc0df308e8f97d44fa6f1d242170336e48f948d624265016)
#[derive(Clone, Default)]
pub struct WgpuMeshPipeline {
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
impl PartialEq for WgpuMeshPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuMeshPipeline {
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

// Source: upstream/packages/types/src/WgpuMeshPipeline.ts:18 (sha256:985e6758b6896c8e482b48ff1d6fa180a9bff8c9500fe4d23b12289b2cd40097)
#[derive(Clone, Default)]
pub struct WgpuScene3DLayouts {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_bind_group_layout: crate::OpaqueHostValue,
    pub frame_bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for WgpuScene3DLayouts {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
