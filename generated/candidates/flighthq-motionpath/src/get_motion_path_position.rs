// @generated from upstream/packages/motionpath/src/getMotionPathPosition.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_path::get_path_position_at_distance;
use flighthq_types::{MotionPath, Vector2Like};

// Source: upstream/packages/motionpath/src/getMotionPathPosition.ts:8 (sha256:7686723f3f68bf4690e34510395f148d8a4590c68cecf3279539dbcb436484fc)
pub fn get_motion_path_position(
    mp: &mut MotionPath,
    point_out: &mut Vector2Like,
    tangent_out: &mut Vector2Like,
) -> bool {
    return {
        let __flight_argument_0 = (mp.path).clone();
        get_path_position_at_distance(
            &__flight_argument_0,
            mp.distance,
            point_out,
            tangent_out,
            None,
        )
    };
}
