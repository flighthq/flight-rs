// @generated from upstream/packages/spatial/src/formatSpatialIndexingNotice.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::MAX_INDEXED_CELLS_PER_OBJECT as max_indexed_cells_per_object_constant;
use flighthq_types::SpatialIndexingNotice;

// Source: upstream/packages/spatial/src/formatSpatialIndexingNotice.ts:18 (sha256:e03c250dcb83d5443af1e0c8dd132edc6ff34c7c37ab433f46dbf46211d79121)
pub fn format_spatial_indexing_notice(notice: &SpatialIndexingNotice) -> String {
    if ((notice.reason).clone())
        .as_ref()
        .map(|value| value.to_string())
        == Some("invalid-cell-size".to_owned())
    {
        return format!(
            "createUniformGridSpatialBackend2D({}): cellSize must be a positive finite number. {}SpatialObject2D({}) used the bounded overflow path instead, so results remain correct but queries scan this object.",
            notice.cell_size,
            (notice.operation).clone(),
            notice.id
        );
    }
    if ((notice.reason).clone())
        .as_ref()
        .map(|value| value.to_string())
        == Some("inverted-bounds".to_owned())
    {
        return format!(
            "{}SpatialObject2D({}): minX/minY must not exceed maxX/maxY, so the object was not indexed and no query will return it. The operation returns false for this — normalize or correct the bounds upstream.",
            (notice.operation).clone(),
            notice.id
        );
    }
    if ((notice.reason).clone())
        .as_ref()
        .map(|value| value.to_string())
        == Some("missing-id".to_owned())
    {
        if ((notice.operation).clone() == "remove") {
            return format!(
                "removeSpatialObject2D({}): the id was not indexed, so removal was a no-op. Check the object's indexing lifecycle if this was unexpected.",
                notice.id
            );
        }
        return format!(
            "updateSpatialObject2D({}): the id was not indexed, so update used its documented insert behavior and left the object in '{}' mode. Use insertSpatialObject2D for a new id, or check the object's indexing lifecycle.",
            notice.id,
            (notice.mode).clone()
        );
    }
    if ((notice.mode).clone() == "declined") {
        return format!(
            "{}SpatialObject2D({}): the bounds are not finite, so the object was not indexed and no query will return it. The operation returns false for this — check the sentinel, and check what produced NaN/Infinity bounds upstream.",
            (notice.operation).clone(),
            notice.id
        );
    }
    if ((notice.mode).clone() == "overflow") {
        return format!(
            "{}SpatialObject2D({}): the bounds span {} cells, over the {} per-object budget, so the object is held in the flat overflow list instead of the grid. Results are unaffected. If this is not a one-off outlier, the grid's cellSize is too small for the objects being indexed — size it to a typical object.",
            (notice.operation).clone(),
            notice.id,
            notice.would_occupy_bucket_count,
            max_indexed_cells_per_object_constant
        );
    }
    return format!(
        "{}SpatialObject2D({}): indexed as '{}', which carries no caller-facing advice.",
        (notice.operation).clone(),
        notice.id,
        (notice.mode).clone()
    );
}
