// @generated from upstream/packages/motionpath/src/motionPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::create_vector2;
use flighthq_path::{get_path_length, get_path_position_at_distance, get_path_tangent_at_distance};
use flighthq_types::{MotionPath, MotionPathLoopMode, Path, Vector2, Vector2Like};

// Source: upstream/packages/motionpath/src/motionPath.ts:12 (sha256:8094d2232ee15414bacdfd1e50a3b65ed92ca74e3d6eff8a98f0ed28642f5f95)
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

// Source: upstream/packages/motionpath/src/motionPath.ts:32 (sha256:11588e1a4d311775614e3907dfdf779161fc08dc219ea8325a5e6168776d7d13)
pub fn get_motion_path_heading(mp: &mut MotionPath) -> f64 {
    {
        let __flight_argument_0 = (mp.path).clone();
        let __flight_result = get_path_tangent_at_distance(
            &__flight_argument_0,
            mp.distance,
            &mut (*SCRATCH_TANGENT.lock().unwrap()),
            None,
        );
        __flight_result
    };
    return ((*SCRATCH_TANGENT.lock().unwrap()).y).atan2((*SCRATCH_TANGENT.lock().unwrap()).x);
}

// Source: upstream/packages/motionpath/src/motionPath.ts:37 (sha256:2e10d36ed25d4ebcc04d55068ab9c532efe1d4fc3f34793fd89445b65802a4be)
static SCRATCH_TANGENT: std::sync::LazyLock<std::sync::Mutex<Vector2>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector2(None, None)));

// Source: upstream/packages/motionpath/src/motionPath.ts:43 (sha256:7686723f3f68bf4690e34510395f148d8a4590c68cecf3279539dbcb436484fc)
pub fn get_motion_path_position(
    mp: &mut MotionPath,
    point_out: &mut Vector2Like,
    tangent_out: &mut Vector2Like,
) -> bool {
    return {
        let __flight_argument_0 = (mp.path).clone();
        let __flight_result = get_path_position_at_distance(
            &__flight_argument_0,
            mp.distance,
            point_out,
            tangent_out,
            None,
        );
        __flight_result
    };
}

// Source: upstream/packages/motionpath/src/motionPath.ts:53 (sha256:429b7e3fefcaef8591c12c85dd06323ca598ca8586cb82eb6350bc02df43d1fb)
pub fn get_motion_path_progress(mp: &MotionPath) -> f64 {
    return if (mp.length > 0.0_f64) {
        (mp.distance / mp.length)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/motionpath/src/motionPath.ts:59 (sha256:dafdaf31f8e675c97650e82b611cde0c53574cfa713dbd344246b20d15864b78)
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

// Source: upstream/packages/motionpath/src/motionPath.ts:69 (sha256:87233ac5b2a6211f0807526f2b5c1a09d30e0fb607f37372c8c504d1f3d81e0c)
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

// Source: upstream/packages/motionpath/src/motionPath.ts:80 (sha256:f6da18ac4d3158bf216568d9f9d211ef0f3ca3f2e72b1013d93a73a410206cc8)
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

// Source: upstream/packages/motionpath/src/motionPath.ts:99 (sha256:861da4dbeebc47e75da16d02e565793fb705d9a89c951e779f19486e268ebc2d)
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
