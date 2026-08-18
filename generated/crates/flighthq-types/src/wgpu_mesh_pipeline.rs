// @generated from upstream/packages/types/src/WgpuMeshPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuMeshPipeline.ts:5 (sha256:5457497ed09b0fa23e64aa05dce5b60933f498139db0fc6470ba4f0e29804d8d)
#[derive(Clone, Default)]
pub struct WgpuMeshPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/WgpuMeshPipeline.ts:16 (sha256:985e6758b6896c8e482b48ff1d6fa180a9bff8c9500fe4d23b12289b2cd40097)
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
