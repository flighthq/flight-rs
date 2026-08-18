// @generated from upstream/packages/types/src/GlPbrExtensionRegistration.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlPbrExtensionBindContext, GlPbrExtensionShaderContext, GlPbrExtensionShaderContribution,
    PbrExtension,
};

// Source: upstream/packages/types/src/GlPbrExtensionRegistration.ts:8 (sha256:acb0b2b98d58648c1a15b9ce6f04852d32ddcadac40f418d9612f01949977938)
#[derive(Clone)]
pub struct GlPbrExtensionRegistration {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(GlPbrExtensionBindContext, PbrExtension) -> () + Send + 'static>,
        >,
    >,
    pub create_shader_contribution: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        GlPbrExtensionShaderContext,
                        PbrExtension,
                    ) -> GlPbrExtensionShaderContribution
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub is_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PbrExtension) -> bool + Send + 'static>>>,
}
impl PartialEq for GlPbrExtensionRegistration {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
