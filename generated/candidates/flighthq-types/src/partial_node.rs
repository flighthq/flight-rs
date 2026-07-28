// @generated from upstream/packages/types/src/PartialNode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PartialNode.ts:1 (sha256:3583a86c0772720f19035939203975228f4f80c285b178711678669c933b6859)
#[derive(Clone)]
pub struct PartialNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<crate::OpaqueHostValue>,
}
impl PartialEq for PartialNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
