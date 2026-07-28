// @generated from upstream/packages/motionpath/src/updateMotionPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::MotionPath;

// Source: upstream/packages/motionpath/src/updateMotionPath.ts:7 (sha256:f6da18ac4d3158bf216568d9f9d211ef0f3ca3f2e72b1013d93a73a410206cc8)
pub fn update_motion_path(mp: &mut MotionPath, delta_time: f64) -> () {
    if (delta_time <= 0.0_f64) {
        return;
    }
    let length = mp.length;
    if (length <= 0.0_f64) {
        return;
    }
    let move_ = (mp.speed * delta_time);
    apply_motion_path_loop_mode(mp, move_, length);
}

// Source: upstream/packages/motionpath/src/updateMotionPath.ts:26 (sha256:861da4dbeebc47e75da16d02e565793fb705d9a89c951e779f19486e268ebc2d)
fn apply_motion_path_loop_mode(mp: &mut MotionPath, move_: f64, length: f64) -> () {
    let loop_mode = (mp.loop_mode).clone();
    let distance = mp.distance;
    let direction = mp.direction;
    if (loop_mode == "loop") {
        let mut wrapped = ((distance + (direction * move_)) % length);
        if (wrapped < 0.0_f64) {
            wrapped += length;
        }
        mp.distance = wrapped;
        return;
    }
    if (loop_mode == "pingpong") {
        let period = (2.0_f64 * length);
        let phase = if (direction < 0.0_f64) {
            (period - distance)
        } else {
            distance
        };
        let mut advanced = ((phase + move_) % period);
        if (advanced < 0.0_f64) {
            advanced += period;
        }
        if (advanced <= length) {
            mp.distance = advanced;
            mp.direction = 1.0_f64;
        } else {
            mp.distance = (period - advanced);
            mp.direction = (-1.0_f64);
        }
        return;
    }
    let mut clamped = (distance + (direction * move_));
    if (clamped < 0.0_f64) {
        clamped = 0.0_f64;
    } else {
        if (clamped > length) {
            clamped = length;
        }
    }
    mp.distance = clamped;
}
