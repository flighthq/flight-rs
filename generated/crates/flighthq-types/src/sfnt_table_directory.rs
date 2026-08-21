// @generated from upstream/packages/types/src/SfntTableDirectory.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SfntTableDirectory.ts:8 (sha256:9c0a9c97e23fa85d9ef93e4bb92eafc52268728257918105ad7b2bef22c8528c)
#[derive(Clone, Default)]
pub struct SfntTableDirectory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub declared_table_count: f64,
    pub sfnt_version: f64,
    pub tables: Vec<(String, SfntTableRange)>,
}
impl PartialEq for SfntTableDirectory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SfntTableDirectory.ts:15 (sha256:1bdecf5ebe419641dbe4f01ba3a31d542cbefad13bccc3e91e726fc63246e2af)
#[derive(Clone, Default)]
pub struct SfntTableRange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub length: f64,
    pub offset: f64,
}
impl PartialEq for SfntTableRange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
