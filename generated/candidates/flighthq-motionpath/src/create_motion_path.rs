// @generated from upstream/packages/motionpath/src/createMotionPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_path::get_path_length;
use flighthq_types::{MotionPath, MotionPathLoopMode, Path};

// Source: upstream/packages/motionpath/src/createMotionPath.ts:11 (sha256:8094d2232ee15414bacdfd1e50a3b65ed92ca74e3d6eff8a98f0ed28642f5f95)
pub fn create_motion_path(
    path: &Path,
    speed: Option<f64>,
    loop_mode: Option<MotionPathLoopMode>,
    tolerance: Option<f64>,
) -> MotionPath {
    let speed = speed.unwrap_or(0.0_f64);
    let loop_mode = loop_mode.unwrap_or("clamp".to_owned());
    return MotionPath {
        __flight_identity: std::sync::Arc::new(()),
        direction: 1.0_f64,
        distance: 0.0_f64,
        length: get_path_length(path, Some((tolerance).clone().unwrap())),
        loop_mode: (loop_mode).clone(),
        path: (*path).clone(),
        speed: speed,
    };
}
