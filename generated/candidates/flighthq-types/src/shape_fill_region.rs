// @generated from upstream/packages/types/src/ShapeFillRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Path;

// Source: upstream/packages/types/src/ShapeFillRegion.ts:7 (sha256:9583984fbeadff5cb76c66d1ce347f6b68a1e9d0ded4e7ebb19cb84d578c2d3d)
#[derive(Clone)]
pub struct ShapeFillRegion {
    pub path: Path,
    pub color: f64,
    pub alpha: f64,
}
