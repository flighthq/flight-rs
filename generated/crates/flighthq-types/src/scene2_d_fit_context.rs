// @generated from upstream/packages/types/src/Scene2DFitContext.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node, ViewportAlign, ViewportScaleMode};

// Source: upstream/packages/types/src/Scene2DFitContext.ts:5 (sha256:568351256a17384a8c1a6368a42a933babe4ce092e32e51376a4d74057cd04e5)
#[derive(Clone, Default)]
pub struct Scene2DFitContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: ViewportAlign,
    pub root: Option<Node>,
    pub scale_mode: ViewportScaleMode,
}
impl PartialEq for Scene2DFitContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
