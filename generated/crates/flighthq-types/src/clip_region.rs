// @generated from upstream/packages/types/src/ClipRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PathWinding, Rectangle};

// Source: upstream/packages/types/src/ClipRegion.ts:19 (sha256:f73b90fe6168b429bc413bda84ebe794b96c7345e5da4ab65264c4241d9995b2)
#[derive(Clone, Default)]
pub struct ClipRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rect: Rectangle,
    pub contours: Option<Vec<Vec<f64>>>,
    pub winding: PathWinding,
    pub version: f64,
}
impl PartialEq for ClipRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ClipRegion.ts:31 (sha256:6c511de0c14602672dd1c83729f1a59405f3ebbae160c52232c3273ce0ab59c6)
pub type ClipRegionReleaseGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ClipRegion) -> () + Send + 'static>>>;
