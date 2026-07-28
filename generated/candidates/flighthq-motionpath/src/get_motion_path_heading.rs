// @generated from upstream/packages/motionpath/src/getMotionPathHeading.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::create_vector2;
use flighthq_path::get_path_tangent_at_distance;
use flighthq_types::{MotionPath, Vector2};

// Source: upstream/packages/motionpath/src/getMotionPathHeading.ts:9 (sha256:11588e1a4d311775614e3907dfdf779161fc08dc219ea8325a5e6168776d7d13)
pub fn get_motion_path_heading(mp: &mut MotionPath) -> f64 {
    {
        let __flight_argument_0 = (mp.path).clone();
        get_path_tangent_at_distance(
            &__flight_argument_0,
            mp.distance,
            &mut (*SCRATCH_TANGENT.lock().unwrap()),
            None,
        )
    };
    return ((*SCRATCH_TANGENT.lock().unwrap()).y).atan2((*SCRATCH_TANGENT.lock().unwrap()).x);
}

// Source: upstream/packages/motionpath/src/getMotionPathHeading.ts:14 (sha256:2e10d36ed25d4ebcc04d55068ab9c532efe1d4fc3f34793fd89445b65802a4be)
static SCRATCH_TANGENT: std::sync::LazyLock<std::sync::Mutex<Vector2>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector2(None, None)));
