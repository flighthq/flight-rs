// @generated from upstream/packages/types/src/DisplayObjectRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BatchFormat, DisplayObject, RenderProxy2D, RenderState, RendererData};

// Source: upstream/packages/types/src/DisplayObjectRenderer.ts:7 (sha256:db64412470e3a77b56acccae26cf88379fd21c4c3c11f0a6724d90371334e060)
#[derive(Clone)]
pub struct DisplayObjectRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<BatchFormat>,
    pub create_data: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, DisplayObject) -> Option<RendererData> + Send + 'static>,
        >,
    >,
    pub destroy_data: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, RendererData) -> () + Send + 'static>>,
        >,
    >,
    pub submit: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(RenderState, RenderProxy2D) -> () + Send + 'static>>,
    >,
}
impl PartialEq for DisplayObjectRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DisplayObjectRenderer.ts:14 (sha256:c7383b822769baacf173c3a1b8042747b3ccbe077769ff1c32ed63a8d9811135)
#[derive(Clone)]
pub struct DisplayObjectClipHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub finalize:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(RenderState) -> () + Send + 'static>>>,
    pub pop_clip: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, RenderProxy2D, DisplayObject) -> () + Send + 'static>,
        >,
    >,
    pub push_clip: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, RenderProxy2D, DisplayObject) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for DisplayObjectClipHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
