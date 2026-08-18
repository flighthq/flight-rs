// @generated from upstream/packages/types/src/WgpuUnlitPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuUnlitPipeline.ts:6 (sha256:b024ecf20addb9cf1fc01e3868192cfd3f3ff20ecc07fafba9b6fb269a1c6c38)
#[derive(Clone, Default)]
pub struct WgpuUnlitDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_color_map: bool,
}
impl PartialEq for WgpuUnlitDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuUnlitPipeline.ts:13 (sha256:13962e75a52f8bd0c83ad837857bb9e1a799458ef290fcc38635a558fc13b699)
#[derive(Clone, Default)]
pub struct WgpuUnlitPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
    pub skinned: bool,
}
impl PartialEq for WgpuUnlitPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
