// @generated from upstream/packages/types/src/BitmapResizeOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BitmapEdgeMode, BitmapResizeMode};

// Source: upstream/packages/types/src/BitmapResizeOptions.ts:4 (sha256:87b09c6d5ea978c064e25bb367e74b89d3b538b6291581755e2f1cfa80075768)
#[derive(Clone, Default)]
pub struct BitmapResizeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: Option<BitmapResizeMode>,
    pub edge_mode: Option<BitmapEdgeMode>,
    pub premultiplied: Option<bool>,
}
impl PartialEq for BitmapResizeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
