// @generated from upstream/packages/types/src/PointGravityWellForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ForceFalloff;

// Source: upstream/packages/types/src/PointGravityWellForce.ts:11 (sha256:c053bdacfe96edb8c4feb840ad48c21fcfae6a2a4a93eca6b49039e2fc0739f2)
#[derive(Clone)]
pub struct PointGravityWellForce {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub strength: f64,
    pub radius: Option<f64>,
    pub min_radius: Option<f64>,
    pub falloff: Option<ForceFalloff>,
    pub repulse: Option<bool>,
}
impl PartialEq for PointGravityWellForce {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PointGravityWellForce.ts:25 (sha256:e3278244bb61c822167efcb800aa2106ea6b3618adc946d0c1fd28dbeaffc9fd)
pub const POINT_GRAVITY_WELL_FORCE_KIND: &'static str = "PointGravityWellForce";
