// @generated from upstream/packages/types/src/CffTable.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CffTable.ts:7 (sha256:5ce6595f77e48ec51736d037cd6c477e81146a82bc72dfb055963b7abde227bd)
#[derive(Clone, Default)]
pub struct CffIndex {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub end_offset: f64,
    pub entries: Vec<CffIndexEntry>,
}
impl PartialEq for CffIndex {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CffTable.ts:13 (sha256:76b9afe6665890aed271a4117b276a29ffdca32fe5e93201234e8c0ecd6a53e6)
#[derive(Clone, Default)]
pub struct CffIndexEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub end: f64,
    pub start: f64,
}
impl PartialEq for CffIndexEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CffTable.ts:21 (sha256:ba30a3ead067283a234c4645d4eb5fdc36f889df5e303408016e9c6acf87bb72)
#[derive(Clone, Default)]
pub struct CffTable {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub charstrings: Vec<CffIndexEntry>,
    pub global_subrs: Vec<CffIndexEntry>,
    pub local_subrs: Vec<CffIndexEntry>,
    pub local_subrs_by_glyph: Option<Vec<Vec<CffIndexEntry>>>,
}
impl PartialEq for CffTable {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
