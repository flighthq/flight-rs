// @generated from upstream/packages/camera2d/src/follow.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_camera2_d_visible_bounds;
use flighthq_geometry::create_rectangle;
use flighthq_math::{clamp, damp};
use flighthq_types::{Camera2D, Camera2DFollowOptions, Rectangle};

// Source: upstream/packages/camera2d/src/follow.ts:20 (sha256:c6f7fa8a1c27676957bd661e9838d216cf034d6c650e1c8d5df2a75c01cb00a8)
pub fn update_camera2_d_follow(
    camera: &mut Camera2D,
    target_x: f64,
    target_y: f64,
    delta_time: f64,
    options: Option<Camera2DFollowOptions>,
) -> () {
    let mut cam_x = camera.x;
    let mut cam_y = camera.y;
    let dead_half_w =
        (options.as_ref().and_then(|value| value.deadzone_half_width)).unwrap_or(0.0_f64);
    let dead_half_h = (options
        .as_ref()
        .and_then(|value| value.deadzone_half_height))
    .unwrap_or(0.0_f64);
    let smooth_time = (options.as_ref().and_then(|value| value.smooth_time)).unwrap_or(0.0_f64);
    let world_bounds = options
        .as_ref()
        .and_then(|value| (value.world_bounds).clone());
    let dx = (target_x - cam_x);
    let mut goal_x = cam_x;
    if (dx > dead_half_w) {
        goal_x = (target_x - dead_half_w);
    } else {
        if (dx < (-dead_half_w)) {
            goal_x = (target_x + dead_half_w);
        }
    }
    let dy = (target_y - cam_y);
    let mut goal_y = cam_y;
    if (dy > dead_half_h) {
        goal_y = (target_y - dead_half_h);
    } else {
        if (dy < (-dead_half_h)) {
            goal_y = (target_y + dead_half_h);
        }
    }
    let mut next_x: f64;
    let mut next_y: f64;
    if ((smooth_time > 0.0_f64) && (delta_time > 0.0_f64)) {
        let lambda = (1.0_f64 / smooth_time);
        next_x = damp(cam_x, goal_x, lambda, delta_time);
        next_y = damp(cam_y, goal_y, lambda, delta_time);
    } else {
        next_x = goal_x;
        next_y = goal_y;
    }
    if (world_bounds).is_some() {
        get_camera2_d_visible_bounds(camera, &mut (*SCRATCH_BOUNDS.lock().unwrap()));
        let half_vis_w = ((*SCRATCH_BOUNDS.lock().unwrap()).width * 0.5_f64);
        let half_vis_h = ((*SCRATCH_BOUNDS.lock().unwrap()).height * 0.5_f64);
        if (world_bounds.as_ref().unwrap().width <= (*SCRATCH_BOUNDS.lock().unwrap()).width) {
            next_x = (world_bounds.as_ref().unwrap().x
                + (world_bounds.as_ref().unwrap().width * 0.5_f64));
        } else {
            next_x = clamp(
                next_x,
                (world_bounds.as_ref().unwrap().x + half_vis_w),
                ((world_bounds.as_ref().unwrap().x + world_bounds.as_ref().unwrap().width)
                    - half_vis_w),
            );
        }
        if (world_bounds.as_ref().unwrap().height <= (*SCRATCH_BOUNDS.lock().unwrap()).height) {
            next_y = (world_bounds.as_ref().unwrap().y
                + (world_bounds.as_ref().unwrap().height * 0.5_f64));
        } else {
            next_y = clamp(
                next_y,
                (world_bounds.as_ref().unwrap().y + half_vis_h),
                ((world_bounds.as_ref().unwrap().y + world_bounds.as_ref().unwrap().height)
                    - half_vis_h),
            );
        }
    }
    camera.x = next_x;
    camera.y = next_y;
}

// Source: upstream/packages/camera2d/src/follow.ts:78 (sha256:f2a34741d72cdb59386dbaf622828c85ae87742c11938a27f293d5a18441f16b)
static SCRATCH_BOUNDS: std::sync::LazyLock<std::sync::Mutex<Rectangle>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_rectangle(None, None, None, None)));
