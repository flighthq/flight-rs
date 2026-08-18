// @generated from upstream/packages/types/src/WgpuClassicPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuClassicPipeline.ts:6 (sha256:af5eb146a079d945bf08d07c1bd3a5e32598423ed6bdac412f5af9cd9031e60c)
pub type WgpuClassicLightingModel = String;

// Source: upstream/packages/types/src/WgpuClassicPipeline.ts:9 (sha256:d33a6ae6c9ca4677439cf4ca374f46b18a1913d8897d75c3ac72cbff54b288df)
#[derive(Clone, Default)]
pub struct WgpuClassicPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
    pub skinned: bool,
}
impl PartialEq for WgpuClassicPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuClassicPipeline.ts:18 (sha256:5c476243586800d985e7c78b4961b4fec554438de4f63f92fbeb4b6000f81ba8)
#[derive(Clone, Default)]
pub struct WgpuClassicDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub double_sided: bool,
    pub has_color_adjustment: Option<bool>,
    pub has_color_matrix: Option<bool>,
    pub has_alpha_map: Option<bool>,
    pub has_diffuse_map: bool,
    pub has_normal_map: bool,
    pub has_specular_map: bool,
    pub lighting_model: WgpuClassicLightingModel,
}
impl PartialEq for WgpuClassicDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
