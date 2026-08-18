// @generated from upstream/packages/types/src/Skeleton2DImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationClip, Skeleton2D, Skeleton2DDrawOrderTimeline};

// Source: upstream/packages/types/src/Skeleton2DImport.ts:11 (sha256:e4fb2376cb4f099b3b36936711f32beb77e6eaf6ea1ddef49f5ccb37149b913b)
#[derive(Clone, Default)]
pub struct Skeleton2DImport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: Vec<Skeleton2DImportAnimation>,
    pub skeleton: Skeleton2D,
}
impl PartialEq for Skeleton2DImport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Skeleton2DImport.ts:26 (sha256:de8c8c6f3b177c89b2e1f173f554dd5ffe1674b74f51372a5ead5cfba1c875b4)
#[derive(Clone, Default)]
pub struct Skeleton2DImportAnimation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: AnimationClip,
    pub draw_order: Option<Skeleton2DDrawOrderTimeline>,
    pub name: String,
}
impl PartialEq for Skeleton2DImportAnimation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
