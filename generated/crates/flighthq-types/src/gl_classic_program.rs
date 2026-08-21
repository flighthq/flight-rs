// @generated from upstream/packages/types/src/GlClassicProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlClassicProgram.ts:6 (sha256:da8c86a67466de62e68689a19c47b1ae7903c06c2335c6065e75d25da995a406)
pub type GlClassicLightingModel = String;

// Source: upstream/packages/types/src/GlClassicProgram.ts:13 (sha256:7b94d617f1c9a7a03d46263f6678d58640d4118d76692005ef73db1a217582f4)
#[derive(Clone, Default)]
pub struct GlClassicDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub has_color_adjustment: Option<bool>,
    pub has_color_matrix: Option<bool>,
    pub has_alpha_map: bool,
    pub has_diffuse_map: bool,
    pub has_normal_map: bool,
    pub has_skin: Option<bool>,
    pub has_specular_map: bool,
    pub has_uv_transform: bool,
    pub lighting_model: GlClassicLightingModel,
}
impl PartialEq for GlClassicDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlClassicProgram.ts:38 (sha256:b20947fd9184317c7f029c89d578561437626fa7ec03965c083784006319e1ec)
#[derive(Clone, Default)]
pub struct GlClassicProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_color_scale: Option<crate::OpaqueHostValue>,
    pub loc_color_bias: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix0: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix1: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix2: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix3: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix_offset: Option<crate::OpaqueHostValue>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_alpha_is_coverage: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_joint_normal_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_ambient_count: Option<crate::OpaqueHostValue>,
    pub loc_ambient_radiance: Option<crate::OpaqueHostValue>,
    pub loc_camera_position: Option<crate::OpaqueHostValue>,
    pub loc_directional: Option<crate::OpaqueHostValue>,
    pub loc_directional_count: Option<crate::OpaqueHostValue>,
    pub loc_directional_radiance: Option<crate::OpaqueHostValue>,
    pub loc_hemisphere_count: Option<crate::OpaqueHostValue>,
    pub loc_hemisphere_lights: Option<crate::OpaqueHostValue>,
    pub loc_ibl_brdf: Option<crate::OpaqueHostValue>,
    pub loc_ibl_enabled: Option<crate::OpaqueHostValue>,
    pub loc_ibl_intensity: Option<crate::OpaqueHostValue>,
    pub loc_ibl_irradiance: Option<crate::OpaqueHostValue>,
    pub loc_ibl_max_mip: Option<crate::OpaqueHostValue>,
    pub loc_ibl_prefiltered: Option<crate::OpaqueHostValue>,
    pub loc_point_count: Option<crate::OpaqueHostValue>,
    pub loc_point_lights: Option<crate::OpaqueHostValue>,
    pub loc_shadow_bias: Option<crate::OpaqueHostValue>,
    pub loc_shadow_enabled: Option<crate::OpaqueHostValue>,
    pub loc_shadow_map: Option<crate::OpaqueHostValue>,
    pub loc_shadow_matrix: Option<crate::OpaqueHostValue>,
    pub loc_shadow_normal_bias_world: Option<crate::OpaqueHostValue>,
    pub loc_shadow_pcf_radius: Option<crate::OpaqueHostValue>,
    pub loc_spot_count: Option<crate::OpaqueHostValue>,
    pub loc_spot_lights: Option<crate::OpaqueHostValue>,
    pub loc_alpha_cutoff: Option<crate::OpaqueHostValue>,
    pub loc_alpha_map: Option<crate::OpaqueHostValue>,
    pub loc_diffuse: Option<crate::OpaqueHostValue>,
    pub loc_diffuse_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_scale: Option<crate::OpaqueHostValue>,
    pub loc_shininess: Option<crate::OpaqueHostValue>,
    pub loc_specular: Option<crate::OpaqueHostValue>,
    pub loc_specular_map: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlClassicProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
