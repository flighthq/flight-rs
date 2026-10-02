// @generated from upstream/packages/types/src/Skeleton2DJsonAnalysis.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skeleton2DJsonAnalysis.ts:1 (sha256:573ca9fb274e35e60d469469030a2441d0e2539e46b33133d20f595df9b4386c)
#[derive(Clone, Default)]
pub struct Skeleton2DJsonAnalysis {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub counts: Vec<(String, f64)>,
    pub format: String,
}
impl PartialEq for Skeleton2DJsonAnalysis {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
