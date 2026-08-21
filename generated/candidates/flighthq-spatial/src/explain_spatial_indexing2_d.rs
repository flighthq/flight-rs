// @generated from upstream/packages/spatial/src/explainSpatialIndexing2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{SpatialIndex2D, SpatialIndexingExplanation, SpatialObjectId};

// Source: upstream/packages/spatial/src/explainSpatialIndexing2D.ts:13 (sha256:c107e5cb76beb3a599766d67539cdd13259d4b24bb25cb1edd1a2db004dae751)
pub fn explain_spatial_indexing2_d(
    index: &SpatialIndex2D,
    id: SpatialObjectId,
) -> SpatialIndexingExplanation {
    return {
        let __flight_callback = (index.runtime.backend.explain_spatial_indexing).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id);
        __flight_result
    };
}
