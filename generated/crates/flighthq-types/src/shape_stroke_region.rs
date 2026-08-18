// @generated from upstream/packages/types/src/ShapeStrokeRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Path, StrokeStyle};

// Source: upstream/packages/types/src/ShapeStrokeRegion.ts:7 (sha256:f1dc61ebc379fe1a424bda629a0fb4dc0e8bb625330235733b29ac0aba54be03)
#[derive(Clone, Default)]
pub struct ShapeStrokeRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub path: Path,
    pub color: f64,
    pub alpha: f64,
    pub style: StrokeStyle,
}
impl PartialEq for ShapeStrokeRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
