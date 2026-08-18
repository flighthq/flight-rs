// @generated from upstream/packages/types/src/SkinInfluence.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SkinInfluence.ts:3 (sha256:04511e93f2917757793dd33b99201edfe88bdaf2903e996761b937b22a3365ea)
#[derive(Clone, Default)]
pub struct SkinInfluence {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joint_index: f64,
    pub weight: f64,
}
impl PartialEq for SkinInfluence {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
