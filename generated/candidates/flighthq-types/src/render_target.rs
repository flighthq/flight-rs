// @generated from upstream/packages/types/src/RenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RenderTarget.ts:6 (sha256:70e56e4c1b52244a919e2cc3bfa0d0ac1ad7f7bec423bf04d8594940a73d302b)
pub type RenderTargetFormat = String;

// Source: upstream/packages/types/src/RenderTarget.ts:8 (sha256:b7e550ff29ab52a8095a9dc34717bc1c7bf9cf5a8e4b6f043e18baa25252f9b4)
pub type RenderTargetDepth = String;

// Source: upstream/packages/types/src/RenderTarget.ts:15 (sha256:935540e940b052c519a1ecf1aabe414ee3507f7c9da1ca6c4ac9721aef250751)
pub type RenderTargetColorSpace = String;

// Source: upstream/packages/types/src/RenderTarget.ts:17 (sha256:f976a3e923d48395ab6e3ab23594c3979ad742550499012816e1aa6fada959dc)
#[derive(Clone, Default)]
pub struct RenderTargetDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
    pub format: Option<RenderTargetFormat>,
    pub color_attachments: Option<f64>,
    pub color_formats: Option<Vec<RenderTargetFormat>>,
    pub sample_count: Option<f64>,
    pub depth: Option<RenderTargetDepth>,
    pub color_space: Option<RenderTargetColorSpace>,
    pub clear_colors: Option<Vec<f64>>,
    pub clear_depth: Option<f64>,
}
impl PartialEq for RenderTargetDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
