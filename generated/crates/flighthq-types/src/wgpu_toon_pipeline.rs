// @generated from upstream/packages/types/src/WgpuToonPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuToonPipeline.ts:9 (sha256:f398168e5cf1af145fc5549817da699d6921686ffdae42534676bf3609d37599)
#[derive(Clone, Default)]
pub struct WgpuToonDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_base_color_map: bool,
    pub has_ramp: bool,
}
impl PartialEq for WgpuToonDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuToonPipeline.ts:17 (sha256:f181dc70570bd7dbff5c2288f5993da0a58149a85f37b24bbf1fd56b7c80910b)
#[derive(Clone, Default)]
pub struct WgpuToonPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
    pub skinned: bool,
}
impl PartialEq for WgpuToonPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
