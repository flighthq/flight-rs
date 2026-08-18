// @generated from upstream/packages/types/src/SwfMorphShapePaths.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Path;

// Source: upstream/packages/types/src/SwfMorphShapePaths.ts:6 (sha256:7eb385b918d55147dd586f7df3f0131e718f5dbed8ec4151fa4d72c8d6270c70)
#[derive(Clone, Default)]
pub struct SwfMorphShapePathsRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub end: Path,
    pub start: Path,
}
impl PartialEq for SwfMorphShapePathsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfMorphShapePaths {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fills: Vec<(f64, SwfMorphShapePathsRecord1)>,
    pub lines: Vec<(f64, SwfMorphShapePathsRecord1)>,
}
impl PartialEq for SwfMorphShapePaths {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
