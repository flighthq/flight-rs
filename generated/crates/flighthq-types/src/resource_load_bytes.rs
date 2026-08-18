// @generated from upstream/packages/types/src/ResourceLoadBytes.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceLoadBytes.ts:8 (sha256:5b1d7cec9efcad88319d5bce57ed94ebb4854279a015b8f7de28907584b76449)
#[derive(Clone, Default)]
pub struct ResourceLoadBytes {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes_loaded: f64,
    pub bytes_total_known: f64,
    pub items_with_known_bytes: f64,
}
impl PartialEq for ResourceLoadBytes {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
