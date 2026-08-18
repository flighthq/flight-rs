// @generated from upstream/packages/types/src/BitmapBevelOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BitmapBevelType;

// Source: upstream/packages/types/src/BitmapBevelOptions.ts:3 (sha256:0a011bdbca5a41569ee8a81d5ebca8c94dc47e164f8c24aef36dbdab9b78f282)
#[derive(Clone, Default)]
pub struct BitmapBevelOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub angle: Option<f64>,
    pub distance: Option<f64>,
    pub radius_x: Option<f64>,
    pub radius_y: Option<f64>,
    pub passes: Option<f64>,
    pub highlight_color: Option<f64>,
    pub shadow_color: Option<f64>,
    pub intensity: Option<f64>,
    pub type_: Option<BitmapBevelType>,
}
impl PartialEq for BitmapBevelOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
