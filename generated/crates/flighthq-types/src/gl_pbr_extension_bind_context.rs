// @generated from upstream/packages/types/src/GlPbrExtensionBindContext.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PbrUvSet, Texture};

// Source: upstream/packages/types/src/GlPbrExtensionBindContext.ts:6 (sha256:0616fb298712eeb5b28283d1975a4a1ea1250b853c94fecbf70c7192a5995d8f)
#[derive(Clone)]
pub struct GlPbrExtensionBindContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_transmission_scene_color:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, String) -> bool + Send + 'static>>>,
    pub bind_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, String, String, Option<Texture>, PbrUvSet) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_linear_color:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, f64) -> () + Send + 'static>>>,
    pub set_float:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, f64) -> () + Send + 'static>>>,
}
impl PartialEq for GlPbrExtensionBindContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
