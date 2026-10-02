// @generated from upstream/packages/types/src/RegistryTable.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/RegistryTable.ts:6 (sha256:eacc31b2ef49b5d874a12429a4bbe2554cacd17407d420c650bb7ea93bb6982d)
#[derive(Clone)]
pub struct OrdinalTable<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entries: Vec<Option<T>>,
    pub vocabulary: Vec<Kind>,
}
impl<T> Default for OrdinalTable<T> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            entries: Default::default(),
            vocabulary: Default::default(),
        }
    }
}
impl<T> PartialEq for OrdinalTable<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
