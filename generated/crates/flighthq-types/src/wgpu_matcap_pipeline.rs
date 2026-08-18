// @generated from upstream/packages/types/src/WgpuMatcapPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuMatcapPipeline.ts:6 (sha256:f5bf1fd0107ae533cb5257481380ec77fd36bb3253350b757f466f92e76a7aef)
#[derive(Clone, Default)]
pub struct WgpuMatcapDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_matcap: bool,
}
impl PartialEq for WgpuMatcapDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuMatcapPipeline.ts:13 (sha256:75902b1447676b35b111e2189f3a23d194034440b7bca6592cc1e7b4603022a8)
#[derive(Clone, Default)]
pub struct WgpuMatcapPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
    pub skinned: bool,
}
impl PartialEq for WgpuMatcapPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
