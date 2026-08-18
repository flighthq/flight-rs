// @generated from upstream/packages/types/src/GlPbrExtensionShaderContribution.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlPbrExtensionShaderContribution.ts:3 (sha256:d43e1b3feba6608a728c2a6353cda7f1775d32d16ac233abc23c9ee6f9dcf68a)
#[derive(Clone, Default)]
pub struct GlPbrExtensionShaderContribution {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply_surface: String,
    pub contribute_ibl: String,
    pub contribute_punctual: String,
    pub finalize: String,
    pub fragment_declarations: String,
    pub fragment_functions: String,
    pub key: String,
    pub samples_transmission_scene_color: Option<bool>,
    pub texture_count: f64,
}
impl PartialEq for GlPbrExtensionShaderContribution {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
