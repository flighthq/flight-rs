// @generated from upstream/packages/types/src/ScanlinesEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ScanlinesEffect.ts:3 (sha256:099eec547d8f28354cd5695de10f1c00d056594cc7376ee30e3aa6c0c21e553a)
#[derive(Clone)]
pub struct ScanlinesEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub count: Option<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for ScanlinesEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
