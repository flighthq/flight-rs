// @generated from upstream/packages/types/src/PathOffsetOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PathOffsetEnd, PathOffsetJoin};

// Source: upstream/packages/types/src/PathOffsetOptions.ts:13 (sha256:58496674ac809c68befeef91aea257e966fec422df3b0a676931ea4736ed37e0)
#[derive(Clone)]
pub struct PathOffsetOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub join: Option<PathOffsetJoin>,
    pub end: Option<PathOffsetEnd>,
    pub miter_limit: Option<f64>,
    pub tolerance: Option<f64>,
    pub arc_tolerance: Option<f64>,
}
impl PartialEq for PathOffsetOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
