// @generated from upstream/packages/types/src/RenderTargetSizeOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RenderTargetSizeOptions.ts:1 (sha256:d096576df44fb3f2e8c6e8407060ad62e0000a87855d01af617a915673f40ae7)
#[derive(Clone, Default)]
pub struct RenderTargetSizeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
}
impl PartialEq for RenderTargetSizeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
