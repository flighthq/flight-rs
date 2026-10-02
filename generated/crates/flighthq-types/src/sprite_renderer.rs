// @generated from upstream/packages/types/src/SpriteRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BatchFormat, NodeAny, RenderProxy2D, RenderState, RendererData, Sprite};

// Source: upstream/packages/types/src/SpriteRenderer.ts:7 (sha256:f9ded3dce9086cf5157bb990118d458ceec4d000dc6d5c1a416920cc603fe55a)
#[derive(Clone)]
pub struct SpriteRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<BatchFormat>,
    pub create_data: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, Sprite) -> Option<RendererData> + Send + 'static>,
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
                Box<dyn FnMut(RenderState, NodeAny, Option<RendererData>) -> bool + Send + 'static>,
            >,
        >,
    >,
    pub submit: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(RenderState, RenderProxy2D) -> () + Send + 'static>>,
    >,
}
impl PartialEq for SpriteRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
