// @generated from upstream/packages/types/src/ToneMapEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ToneMapEffect.ts:3 (sha256:18c47bb6429d38fa8969cddceb75fbd9f3ed125456bb5048b53a7ae029c6e08b)
pub type ToneMapOperator = String;

// Source: upstream/packages/types/src/ToneMapEffect.ts:5 (sha256:3ed46015146588b439615b5f39aec2e63064c530db2f30842559d441ef41f01a)
#[derive(Clone)]
pub struct ToneMapEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub operator: Option<ToneMapOperator>,
    pub exposure: Option<f64>,
    pub white: Option<f64>,
}
impl PartialEq for ToneMapEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
