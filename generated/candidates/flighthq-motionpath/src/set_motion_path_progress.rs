// @generated from upstream/packages/motionpath/src/setMotionPathProgress.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::MotionPath;

// Source: upstream/packages/motionpath/src/setMotionPathProgress.ts:5 (sha256:87233ac5b2a6211f0807526f2b5c1a09d30e0fb607f37372c8c504d1f3d81e0c)
pub fn set_motion_path_progress(mp: &mut MotionPath, t: f64) -> () {
    let mut clamped = t;
    if (clamped < 0.0_f64) {
        clamped = 0.0_f64;
    } else {
        if (clamped > 1.0_f64) {
            clamped = 1.0_f64;
        }
    }
    mp.distance = (clamped * mp.length);
}
