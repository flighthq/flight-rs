// @generated from upstream/packages/types/src/AttractorForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ForceFalloff;

// Source: upstream/packages/types/src/AttractorForce.ts:3 (sha256:4a254f8d5da04800e4650dfb75e31888fd2646240cbba4e557003330f7d39df6)
#[derive(Clone)]
pub struct AttractorForce {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub strength: f64,
    pub radius: Option<f64>,
    pub falloff: Option<ForceFalloff>,
}
impl PartialEq for AttractorForce {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AttractorForce.ts:13 (sha256:0ee70c00f3a55d4ff60845bfa0e6ec7735ce05c2f744cb3fafdaebfefbc9e5df)
pub const ATTRACTOR_FORCE_KIND: &'static str = "AttractorForce";
