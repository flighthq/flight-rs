// @generated from upstream/packages/types/src/HalftoneEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/HalftoneEffect.ts:3 (sha256:ed6d743155eae5359c44c1f9a9a07d0978b9343d16fec86c4d6c858e9100a805)
#[derive(Clone, Default)]
pub struct HalftoneEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub scale: Option<f64>,
    pub angle: Option<f64>,
}
impl PartialEq for HalftoneEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
