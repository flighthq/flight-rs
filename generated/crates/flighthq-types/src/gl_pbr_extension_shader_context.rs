// @generated from upstream/packages/types/src/GlPbrExtensionShaderContext.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;

// Source: upstream/packages/types/src/GlPbrExtensionShaderContext.ts:4 (sha256:ae4c2dcf8c55e8ce6741981a522cad7816e4147172424f54b6b21a9c476a264e)
#[derive(Clone)]
pub struct GlPbrExtensionShaderContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_transmission_scene_color:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_texture_ready:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<Texture>) -> bool + Send + 'static>>>,
}
impl PartialEq for GlPbrExtensionShaderContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
