// @generated from upstream/packages/types/src/BackendExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BackendExplanation.ts:1 (sha256:92cfa6569cb5ae5e4bfcf63c57f4b54339a43c30a40e966016cc3c605e850eb4)
#[derive(Clone, Default)]
pub struct BackendExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub conflict: bool,
    pub layer: String,
    pub operation: Option<String>,
    pub viability: String,
}
impl PartialEq for BackendExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
