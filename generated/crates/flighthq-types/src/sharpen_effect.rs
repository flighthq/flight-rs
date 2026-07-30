// @generated from upstream/packages/types/src/SharpenEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/SharpenEffect.ts:3 (sha256:95e10a108935dd00270d48e51a58096a1710da97b05a4f1e05849c2c0a42b7b6)
#[derive(Clone, Default)]
pub struct SharpenEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub amount: Option<f64>,
}
impl PartialEq for SharpenEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
