// @generated from upstream/packages/path/src/tessellatePathTyped.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::tessellate_path;
use flighthq_types::{Path, PathMeshTyped};

// Source: upstream/packages/path/src/tessellatePathTyped.ts:8 (sha256:bf29a56f531b14ae31c2de2f35996aa5d964510a52e467316fc00b221942a7cc)
pub fn tessellate_path_typed(path: &Path, tolerance: Option<f64>) -> PathMeshTyped {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let mesh = tessellate_path(path, Some(tolerance));
    return PathMeshTyped {
        __flight_identity: std::sync::Arc::new(()),
        vertices: ((mesh.vertices).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect(),
        indices: ((mesh.indices).clone())
            .iter()
            .map(|value| (*value) as u32)
            .collect(),
    };
}
