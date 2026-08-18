// @generated from upstream/packages/types/src/Scene2DRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BatchFormat, Node2D, RenderProxy2D, RenderState, Renderable, RendererData};

// Source: upstream/packages/types/src/Scene2DRenderer.ts:7 (sha256:756ce2ce73290d6c0026edadad28ad3a8ecef6042387a664243db86408467e03)
#[derive(Clone)]
pub struct Scene2DRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<BatchFormat>,
    pub create_data: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, Node2D) -> Option<RendererData> + Send + 'static>,
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
        std::sync::Mutex<Box<dyn FnMut(RenderState, RenderProxy2D) -> () + Send + 'static>>,
    >,
}
impl PartialEq for Scene2DRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DRenderer.ts:14 (sha256:9298e505b99d5557fb82e5d04f9e448e1ef1e434f1bde6d7380a2e77f5eb443d)
#[derive(Clone)]
pub struct Scene2DClipHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub finalize:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(RenderState) -> () + Send + 'static>>>,
    pub pop_clip: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(RenderState, RenderProxy2D, Node2D) -> () + Send + 'static>>,
    >,
    pub push_clip: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(RenderState, RenderProxy2D, Node2D) -> () + Send + 'static>>,
    >,
}
impl PartialEq for Scene2DClipHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
