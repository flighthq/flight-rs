// @generated from upstream/packages/types/src/BitmapDisplacementMapOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BitmapDisplacementMapMode, BitmapEdgeMode, BitmapRegion};

// Source: upstream/packages/types/src/BitmapDisplacementMapOptions.ts:5 (sha256:c2fd0040fbc7b51bf0203d08bdd9632541b02207997542283e86c19175cd69c8)
#[derive(Clone, Default)]
pub struct BitmapDisplacementMapOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub map: BitmapRegion,
    pub component_x: Option<f64>,
    pub component_y: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub mode: Option<BitmapDisplacementMapMode>,
    pub edge_mode: Option<BitmapEdgeMode>,
    pub fill_color: Option<f64>,
}
impl PartialEq for BitmapDisplacementMapOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
