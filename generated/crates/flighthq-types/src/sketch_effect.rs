// @generated from upstream/packages/types/src/SketchEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/SketchEffect.ts:3 (sha256:9df395f8338664e9eeb891f5192981dfacb653c16c2f76ecb15e083c78e567ec)
#[derive(Clone, Default)]
pub struct SketchEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub strength: Option<f64>,
}
impl PartialEq for SketchEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
