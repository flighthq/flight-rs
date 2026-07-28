// @generated from upstream/packages/types/src/RendererData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RendererData.ts:3 (sha256:ceee83b4526296d63fea61167e6c510adbf00521b4e4937c63435ff43233ee6a)
#[derive(Clone, Default)]
pub struct RendererData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for RendererData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
