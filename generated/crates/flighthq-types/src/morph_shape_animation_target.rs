// @generated from upstream/packages/types/src/MorphShapeAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::MorphShape;

// Source: upstream/packages/types/src/MorphShapeAnimationTarget.ts:6 (sha256:fe1394427e35042ec6cc108bd2924960765e205372f08ad78f79be2edf26d287)
#[derive(Clone, Default)]
pub struct MorphShapeAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub shape: MorphShape,
}
impl PartialEq for MorphShapeAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
