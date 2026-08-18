// @generated from upstream/packages/types/src/SpatialIndexing.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpatialObjectId;

// Source: upstream/packages/types/src/SpatialIndexing.ts:7 (sha256:dfb909d0faa1b172e1fa57d3eec1c386f96713eb42afff62bccd3e698a66364d)
#[derive(Clone, Default)]
pub struct SpatialIndexingExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: SpatialObjectId,
    pub mode: SpatialIndexingMode,
    pub bucket_count: f64,
    pub reason: Option<SpatialDeclineReason>,
}
impl PartialEq for SpatialIndexingExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpatialIndexing.ts:25 (sha256:9769bf792364fc595f28801fa0779fbec2de222d56f382c88ff265d4e981c785)
pub type SpatialIndexingMode = String;

// Source: upstream/packages/types/src/SpatialIndexing.ts:29 (sha256:ab080934d853ec207d9ebfd0dd0d21542c2ef46c469fdb24511b33eedc7a1d12)
pub type SpatialDeclineReason = String;

// Source: upstream/packages/types/src/SpatialIndexing.ts:32 (sha256:8a257f64bcbb0301b9fb2120b562e95f1e993bb397d1e305a9ee4c5d11d49c0f)
pub type SpatialIndexingOperation = String;

// Source: upstream/packages/types/src/SpatialIndexing.ts:37 (sha256:9f5b46dc9e93b8e77cdb56563de1be0a6fd727c8bb183ea470ea4b09dcf2a795)
pub type SpatialIndexingReason = crate::FlightUnion2<SpatialDeclineReason, String>;

// Source: upstream/packages/types/src/SpatialIndexing.ts:42 (sha256:f864004b87b82bcca917b1ed1e00b1b83f330263d4f5c0fce6e3b8e5bc6dafa8)
#[derive(Clone, Default)]
pub struct SpatialIndexingNotice {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cell_size: f64,
    pub id: SpatialObjectId,
    pub mode: SpatialIndexingMode,
    pub operation: SpatialIndexingOperation,
    pub would_occupy_bucket_count: f64,
    pub reason: Option<SpatialIndexingReason>,
}
impl PartialEq for SpatialIndexingNotice {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpatialIndexing.ts:56 (sha256:a4e52625be94fdc6783d9f290ab75ae93e18e48d9ff505560ddb63fd32d8ad56)
pub type SpatialIndexingGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SpatialIndexingNotice) -> () + Send + 'static>>>;
