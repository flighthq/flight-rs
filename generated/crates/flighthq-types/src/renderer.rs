// @generated from upstream/packages/types/src/Renderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BatchFormat, RenderProxy, RenderState, Renderable, RendererData};

// Source: upstream/packages/types/src/Renderer.ts:7 (sha256:2c2aa988b1ba8b33e663de85bf001da0a7769676b02a6000baed218622687043)
#[derive(Clone)]
pub struct Renderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<BatchFormat>,
    pub create_data: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, Renderable) -> Option<RendererData> + Send + 'static>,
        >,
    >,
    pub destroy_data: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, RendererData) -> () + Send + 'static>>,
        >,
    >,
    pub is_dirty: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(RenderState, Renderable, Option<RendererData>) -> bool
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub submit: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(RenderState, RenderProxy) -> () + Send + 'static>>,
    >,
}
impl PartialEq for Renderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
