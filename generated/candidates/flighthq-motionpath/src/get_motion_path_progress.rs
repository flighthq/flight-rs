// @generated from upstream/packages/motionpath/src/getMotionPathProgress.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::MotionPath;

// Source: upstream/packages/motionpath/src/getMotionPathProgress.ts:5 (sha256:429b7e3fefcaef8591c12c85dd06323ca598ca8586cb82eb6350bc02df43d1fb)
pub fn get_motion_path_progress(mp: &MotionPath) -> f64 {
    return if (mp.length > 0.0_f64) {
        (mp.distance / mp.length)
    } else {
        0.0_f64
    };
}
