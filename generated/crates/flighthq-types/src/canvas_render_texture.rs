// @generated from upstream/packages/types/src/CanvasRenderTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasRenderState, CanvasRenderTarget, CanvasRenderTargetPool, RenderTexture};

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:6 (sha256:fd069efe688deec5cc0c65e6a976dc2148345bf96474876535b91afd4f8b8943)
pub type CanvasRenderTextureStatus = String;

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:8 (sha256:5110cc247aaba889f23dcefc02330f1204c91fbc99863105fe96425fa255bda2)
#[derive(Clone, Default)]
pub struct CanvasRenderTextureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub status: CanvasRenderTextureStatus,
    pub width: f64,
}
impl PartialEq for CanvasRenderTextureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:14 (sha256:347be02a5d0ddbe8c51171c42f0c6fbb5fd7c9a9ce57332156da8e12fbaf5722)
#[derive(Clone, Default)]
pub struct CanvasRenderTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: CanvasRenderTextureStatus,
    pub target: CanvasRenderTarget,
}
impl PartialEq for CanvasRenderTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderTexture.ts:21 (sha256:bccbdac026b10fed057b1fa96baa3cd24ad093dbb672ca4714c9f87985872894)
#[derive(Clone, Default)]
pub struct CanvasRenderTexturePool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroyed: bool,
    pub effect_targets: CanvasRenderTargetPool,
    pub free: Vec<RenderTexture>,
    pub leased: Vec<RenderTexture>,
    pub owner: Option<CanvasRenderState>,
}
impl PartialEq for CanvasRenderTexturePool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
