// @generated from upstream/packages/types/src/WgpuQuadBatchResources.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuQuadBatchResources.ts:1 (sha256:e7401ae5f771894539b7a791f292abc1b53210f1dff1ac22aab2843df5382509)
#[derive(Clone, Default)]
pub struct WgpuQuadBatchResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instance_bind_group_layout: crate::OpaqueHostValue,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub base_pipeline_layout: crate::OpaqueHostValue,
    pub material_pipeline_layout: crate::OpaqueHostValue,
    pub pipelines: Vec<(
        crate::OpaqueHostValue,
        Vec<(String, crate::OpaqueHostValue)>,
    )>,
}
impl PartialEq for WgpuQuadBatchResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
