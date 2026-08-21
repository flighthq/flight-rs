// @generated from upstream/packages/path/src/explainStrokePathTessellation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    STROKE_PATH_TESSELLATION_ISSUE_INVALID_PATH as stroke_path_tessellation_issue_invalid_path_constant,
    STROKE_PATH_TESSELLATION_ISSUE_INVALID_STYLE as stroke_path_tessellation_issue_invalid_style_constant,
    STROKE_PATH_TESSELLATION_ISSUE_NONE as stroke_path_tessellation_issue_none_constant,
    STROKE_PATH_TESSELLATION_ISSUE_REVERSING_JOIN as stroke_path_tessellation_issue_reversing_join_constant,
    STROKE_PATH_TESSELLATION_ISSUE_SELF_INTERSECTING_CENTERLINE as stroke_path_tessellation_issue_self_intersecting_centerline_constant,
    STROKE_PATH_TESSELLATION_ISSUE_SELF_INTERSECTING_OUTLINE as stroke_path_tessellation_issue_self_intersecting_outline_constant,
    build_stroke_path_geometry,
};
use flighthq_types::{Path, StrokeStyle};
pub use flighthq_types::{StrokePathTessellationExplanation, StrokePathTessellationReason};

// Source: upstream/packages/path/src/explainStrokePathTessellation.ts:23 (sha256:789a5fea8371cb14f185f4b1d6dc32a9cf02a065e8b33527372d2e88324a6a84)
pub fn explain_stroke_path_tessellation(
    path: &Path,
    style: &StrokeStyle,
    tolerance: Option<f64>,
) -> StrokePathTessellationExplanation {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let geometry = build_stroke_path_geometry(path, style, tolerance);
    return StrokePathTessellationExplanation {
        __flight_identity: std::sync::Arc::new(()),
        reason: get_reason(geometry.issue),
        subpath: geometry.issue_subpath,
        supported: (geometry.issue == stroke_path_tessellation_issue_none_constant),
    };
}

// Source: upstream/packages/path/src/explainStrokePathTessellation.ts:36 (sha256:4ea4e375e4218a9e285b6ae7773a126ebf603eb51ded717b9e6c48aea304fbea)
fn get_reason(issue: f64) -> StrokePathTessellationReason {
    if (issue == stroke_path_tessellation_issue_invalid_style_constant) {
        return "invalid-style".to_owned();
    }
    if (issue == stroke_path_tessellation_issue_invalid_path_constant) {
        return "invalid-path".to_owned();
    }
    if (issue == stroke_path_tessellation_issue_self_intersecting_centerline_constant) {
        return "self-intersecting-centerline".to_owned();
    }
    if (issue == stroke_path_tessellation_issue_reversing_join_constant) {
        return "reversing-join".to_owned();
    }
    if (issue == stroke_path_tessellation_issue_self_intersecting_outline_constant) {
        return "self-intersecting-outline".to_owned();
    }
    return "ok".to_owned();
}
