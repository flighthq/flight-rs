// @generated from upstream/packages/types/src/GlCapabilities.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlCapabilities.ts:1 (sha256:76cc879028708f5d0eed8db4f3b31fa4d064e3e284516d8ef2e3e0ea38f64f9d)
#[derive(Clone)]
pub struct GlCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_texture_size: f64,
    pub max_texture_units: f64,
    pub max_samples: f64,
    pub max_draw_buffers: f64,
    pub max_color_attachments: f64,
    pub max_renderbuffer_size: f64,
    pub supports_color_buffer_float: bool,
    pub supports_float_linear: bool,
    pub supports_srgb: bool,
    pub max_anisotropy: f64,
}
impl PartialEq for GlCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
