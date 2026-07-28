// @generated from upstream/packages/types/src/DomStageRectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DomStageRectangle.ts:1 (sha256:fc8013d047c9528642b95db2f22da6d31c527b58936e073300f57cc07e3e6ff3)
#[derive(Clone, Default)]
pub struct DomStageRectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub top: f64,
}
impl PartialEq for DomStageRectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
