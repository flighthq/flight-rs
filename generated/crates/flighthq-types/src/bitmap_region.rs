// @generated from upstream/packages/types/src/BitmapRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Bitmap;

// Source: upstream/packages/types/src/BitmapRegion.ts:3 (sha256:6de1c57a64f9d839dba96b69bcdd8cae0ca18580cc13f425ae6cb9ec9f68c4b8)
#[derive(Clone, Default)]
pub struct BitmapRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub bitmap: Bitmap,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for BitmapRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
