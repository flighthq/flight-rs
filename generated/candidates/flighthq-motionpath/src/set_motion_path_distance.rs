// @generated from upstream/packages/motionpath/src/setMotionPathDistance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::MotionPath;

// Source: upstream/packages/motionpath/src/setMotionPathDistance.ts:5 (sha256:dafdaf31f8e675c97650e82b611cde0c53574cfa713dbd344246b20d15864b78)
pub fn set_motion_path_distance(mp: &mut MotionPath, distance: f64) -> () {
    let length = mp.length;
    let mut clamped = distance;
    if (clamped < 0.0_f64) {
        clamped = 0.0_f64;
    } else {
        if (clamped > length) {
            clamped = length;
        }
    }
    mp.distance = clamped;
}
