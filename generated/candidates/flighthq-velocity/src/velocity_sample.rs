// @generated from upstream/packages/velocity/src/velocitySample.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Matrix, Velocity2D, VelocitySample};

// Source: upstream/packages/velocity/src/velocitySample.ts:9 (sha256:370ea652b60af7fa9f71db408236a5104aaacf1cff2151bb3313aa5240afc4b4)
pub fn get_velocity_sample_at(
    sample: &VelocitySample,
    current_world_transform: &Matrix,
    point_x: f64,
    point_y: f64,
    out: &mut Velocity2D,
) -> Velocity2D {
    if ((sample.previous_world_transform).clone()).is_none() {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        return out.clone();
    }
    let cx = (((current_world_transform.a * point_x) + (current_world_transform.c * point_y))
        + current_world_transform.tx);
    let cy = (((current_world_transform.b * point_x) + (current_world_transform.d * point_y))
        + current_world_transform.ty);
    let px = (((sample.previous_world_transform.as_ref().unwrap().a * point_x)
        + (sample.previous_world_transform.as_ref().unwrap().c * point_y))
        + sample.previous_world_transform.as_ref().unwrap().tx);
    let py = (((sample.previous_world_transform.as_ref().unwrap().b * point_x)
        + (sample.previous_world_transform.as_ref().unwrap().d * point_y))
        + sample.previous_world_transform.as_ref().unwrap().ty);
    out.x = (cx - px);
    out.y = (cy - py);
    return out.clone();
}
