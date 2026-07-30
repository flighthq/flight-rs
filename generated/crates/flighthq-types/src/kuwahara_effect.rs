// @generated from upstream/packages/types/src/KuwaharaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/KuwaharaEffect.ts:3 (sha256:3a166322e5591889b0e89cde634fce0a0351a317831539d20dd905a46ac01aec)
#[derive(Clone, Default)]
pub struct KuwaharaEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub radius: Option<f64>,
}
impl PartialEq for KuwaharaEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
