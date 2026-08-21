// @generated from upstream/packages/path/src/explainPathMorphCreation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    PATH_MORPH_ISSUE_CONTOUR_CLOSEDNESS_MISMATCH as path_morph_issue_contour_closedness_mismatch_constant,
    PATH_MORPH_ISSUE_CONTOUR_COUNT_MISMATCH as path_morph_issue_contour_count_mismatch_constant,
    PATH_MORPH_ISSUE_CONTOUR_ORIENTATION_MISMATCH as path_morph_issue_contour_orientation_mismatch_constant,
    PATH_MORPH_ISSUE_NONE as path_morph_issue_none_constant,
    PATH_MORPH_ISSUE_WINDING_MISMATCH as path_morph_issue_winding_mismatch_constant,
    build_path_morph,
};
use flighthq_types::Path;
pub use flighthq_types::{PathMorphCreationExplanation, PathMorphCreationReason};

// Source: upstream/packages/path/src/explainPathMorphCreation.ts:17 (sha256:834254d450d27feae9ee0726d8d929f31c708d02272fb64be41f70ed038611ed)
pub fn explain_path_morph_creation(start: &Path, end: &Path) -> PathMorphCreationExplanation {
    let result = build_path_morph(start, end);
    return PathMorphCreationExplanation {
        __flight_identity: std::sync::Arc::new(()),
        contour: result.contour,
        reason: get_reason(result.issue),
        supported: (result.issue == path_morph_issue_none_constant),
    };
}

// Source: upstream/packages/path/src/explainPathMorphCreation.ts:26 (sha256:71771b78cba2aaa91133d282e0a75d83b09381ef9868f2b787c964459e073f87)
fn get_reason(issue: f64) -> PathMorphCreationReason {
    if (issue == path_morph_issue_winding_mismatch_constant) {
        return "winding-mismatch".to_owned();
    }
    if (issue == path_morph_issue_contour_count_mismatch_constant) {
        return "contour-count-mismatch".to_owned();
    }
    if (issue == path_morph_issue_contour_closedness_mismatch_constant) {
        return "contour-closedness-mismatch".to_owned();
    }
    if (issue == path_morph_issue_contour_orientation_mismatch_constant) {
        return "contour-orientation-mismatch".to_owned();
    }
    return "ok".to_owned();
}
