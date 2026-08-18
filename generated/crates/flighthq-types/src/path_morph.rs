// @generated from upstream/packages/types/src/PathMorph.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PathWinding;

// Source: upstream/packages/types/src/PathMorph.ts:7 (sha256:c25a51028062eec11ee4ab9e57bb4ceb38827380efadcb801856aade5c3c20d6)
#[derive(Clone, Default)]
pub struct PathMorph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub commands: Vec<f64>,
    pub end_data: Vec<f64>,
    pub start_data: Vec<f64>,
    pub winding: PathWinding,
}
impl PartialEq for PathMorph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
