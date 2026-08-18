// @generated from upstream/packages/types/src/TransformInherit2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TransformInherit2D.ts:17 (sha256:58e62708f57df42b10b3295377b9c994df1e99c342cdf02e0df5ddac41df98f5)
#[derive(Clone, Default)]
pub struct TransformInherit2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reflection: bool,
    pub rotation: bool,
    pub scale: bool,
    pub translation: bool,
}
impl PartialEq for TransformInherit2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
