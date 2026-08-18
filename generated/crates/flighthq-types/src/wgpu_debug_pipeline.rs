// @generated from upstream/packages/types/src/WgpuDebugPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuDebugPipeline.ts:7 (sha256:b1a085782a24ad6c51ca8ed571649c0ca66678fd2b8c7269042a832ee8f19386)
#[derive(Clone, Default)]
pub struct WgpuDebugDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_normal_map: bool,
    pub mode: String,
}
impl PartialEq for WgpuDebugDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuDebugPipeline.ts:13 (sha256:9f44488e7bfb03ef89ea229771bbef597d62294f069ffcab6bacc1edff7e2c11)
#[derive(Clone, Default)]
pub struct WgpuDebugPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
    pub skinned: bool,
}
impl PartialEq for WgpuDebugPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
