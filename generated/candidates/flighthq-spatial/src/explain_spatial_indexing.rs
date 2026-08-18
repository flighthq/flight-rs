// @generated from upstream/packages/spatial/src/explainSpatialIndexing.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{SpatialIndex, SpatialIndexingExplanation, SpatialObjectId};

// Source: upstream/packages/spatial/src/explainSpatialIndexing.ts:13 (sha256:644d502687e6c6258255cad445913742820a1210e729ced13a079c9757586af1)
pub fn explain_spatial_indexing(
    index: &SpatialIndex,
    id: SpatialObjectId,
) -> SpatialIndexingExplanation {
    return {
        let __flight_callback = (index.runtime.backend.explain_spatial_indexing).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id);
        __flight_result
    };
}
