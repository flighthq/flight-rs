// @generated from upstream/packages/types/src/Attachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Attachment2D.ts:7 (sha256:fc469b928f6a26542e450d9876de7f8989497309253813eea980c36e457406fc)
#[derive(Clone, Default)]
pub struct Attachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
}
impl PartialEq for Attachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
