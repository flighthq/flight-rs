// @generated from upstream/packages/types/src/Skeleton2DSlotDeform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Attachment2D;

// Source: upstream/packages/types/src/Skeleton2DSlotDeform.ts:21 (sha256:5d3e66f2f82fdde048fa46dc2a59d8cfc2ea0326b005684a745e6ff6f60000ca)
#[derive(Clone, Default)]
pub struct Skeleton2DSlotDeform {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attachment: Option<Attachment2D>,
    pub offsets: Vec<f32>,
}
impl PartialEq for Skeleton2DSlotDeform {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
