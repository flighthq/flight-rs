// @generated from upstream/packages/types/src/ShapeFillRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Path;

// Source: upstream/packages/types/src/ShapeFillRegion.ts:6 (sha256:9583984fbeadff5cb76c66d1ce347f6b68a1e9d0ded4e7ebb19cb84d578c2d3d)
#[derive(Clone, Default)]
pub struct ShapeFillRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub path: Path,
    pub color: f64,
    pub alpha: f64,
}
impl PartialEq for ShapeFillRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
