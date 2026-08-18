// @generated from upstream/packages/types/src/GlRenderTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, GlRenderTarget, GlRenderTargetPool, RenderTexture};

// Source: upstream/packages/types/src/GlRenderTexture.ts:5 (sha256:7fe14e2761d838a139225d2488a7b28f0769eae7ae0959ffe91dd62e81a3f401)
pub type GlRenderTextureStatus = String;

// Source: upstream/packages/types/src/GlRenderTexture.ts:7 (sha256:39dac1b1bb916941fd370d477ba6730f1eded94d2396ef556e3c5b9ed02afbce)
#[derive(Clone, Default)]
pub struct GlRenderTextureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub status: GlRenderTextureStatus,
    pub width: f64,
}
impl PartialEq for GlRenderTextureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderTexture.ts:13 (sha256:a7fa638af0c9e6325e55fe5a74a9b9a6af64eb5bcba6dd5adbd45c089c1d8836)
#[derive(Clone, Default)]
pub struct GlRenderTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: GlRenderTextureStatus,
    pub target: GlRenderTarget,
}
impl PartialEq for GlRenderTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderTexture.ts:20 (sha256:c9cc45d02ce4d7e948c85172e4a590ef8bacca7dc268920a688dea44c772f41c)
#[derive(Clone, Default)]
pub struct GlRenderTexturePool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub context: Option<crate::OpaqueHostValue>,
    pub destroyed: bool,
    pub effect_targets: GlRenderTargetPool,
    pub free: Vec<RenderTexture>,
    pub leased: Vec<RenderTexture>,
}
impl PartialEq for GlRenderTexturePool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderTexture.ts:28 (sha256:2f3a2dad10d2d1210708e1690cfa69d093b7d7f66bbe1aaa2ef0193b4b12bdc2)
pub type GlRenderTextureGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(GlRenderState, RenderTexture, GlRenderTextureExplanation) -> ()
                + Send
                + 'static,
        >,
    >,
>;
