// @generated from upstream/packages/types/src/WgpuPbrPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuPbrPipeline.ts:9 (sha256:a7eca7b8b4c5bebcde7fcacf5c5ca74544887c32edab5eaa871e975e1e966283)
#[derive(Clone, Default)]
pub struct WgpuPbrPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_ibl_group: bool,
    pub has_pbr_sample_group: bool,
    pub has_shadow_group: bool,
    pub material_bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
    pub skinned: bool,
}
impl PartialEq for WgpuPbrPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuPbrPipeline.ts:19 (sha256:5b1c782aab4ed949c1a9113b814e6de6be86517559e31b7c6c4f171b2b5cf61d)
#[derive(Clone, Default)]
pub struct WgpuPbrDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub anisotropy_enabled: bool,
    pub clearcoat_enabled: bool,
    pub double_sided: bool,
    pub has_alpha_map: bool,
    pub has_base_color_map: bool,
    pub has_color_adjustment: Option<bool>,
    pub has_color_matrix: Option<bool>,
    pub has_emissive_map: bool,
    pub has_metallic_roughness_map: bool,
    pub has_normal_map: bool,
    pub has_occlusion_map: bool,
    pub iridescence_enabled: bool,
    pub sheen_enabled: bool,
    pub specular_enabled: bool,
    pub subsurface_enabled: bool,
    pub transmission_enabled: bool,
}
impl PartialEq for WgpuPbrDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
