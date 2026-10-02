// @generated from upstream/packages/types/src/RenderTargetClear.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RenderTargetClear.ts:7 (sha256:369cefd87f4aa6eb78736073f4ae09fa083219d1199499fe6bf85f129ec3bd57)
#[derive(Clone, Default)]
pub struct RenderTargetClear {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Option<Vec<f64>>,
    pub colors: Option<Vec<Option<Vec<f64>>>>,
    pub depth: Option<f64>,
    pub stencil: Option<f64>,
}
impl PartialEq for RenderTargetClear {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
