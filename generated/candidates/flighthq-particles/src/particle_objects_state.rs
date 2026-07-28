// @generated from upstream/packages/particles/src/particleObjectsState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::reserve_float32_array;
use flighthq_types::{ParticleObjectsState, RandomSource};

// Source: upstream/packages/particles/src/particleObjectsState.ts:4 (sha256:38efe1561666d49a8a3703c679ed1a1bb2d014f51a0fd5ba46477ad55ebc4d48)
pub fn create_particle_objects_state(
    capacity: f64,
    random: Option<RandomSource>,
) -> ParticleObjectsState {
    let random = random.unwrap_or((math.random).clone());
    return ParticleObjectsState {
        __flight_identity: std::sync::Arc::new(()),
        burst_timer: 0.0_f64,
        emitter_age: 0.0_f64,
        lifetimes: vec![0.0_f32; (capacity * 2.0_f64) as usize],
        prev_x: f64::NAN,
        prev_y: f64::NAN,
        random: (random).clone(),
        rotation_speeds: vec![0.0_f32; (capacity) as usize],
        scales: vec![0.0_f32; (capacity) as usize],
        spawn_accumulator: 0.0_f64,
        velocities: vec![0.0_f32; (capacity * 2.0_f64) as usize],
    };
}

// Source: upstream/packages/particles/src/particleObjectsState.ts:19 (sha256:128414a5499294c20d9d3357e075e314351f8a394a2880be55e34ff232e3d541)
pub fn ensure_particle_objects_state_capacity(
    state: &mut ParticleObjectsState,
    capacity: f64,
) -> () {
    if ((state.lifetimes.len() as f64) >= (capacity * 2.0_f64)) {
        return;
    }
    state.lifetimes = reserve_float32_array(&state.lifetimes, (capacity * 2.0_f64));
    state.velocities = reserve_float32_array(&state.velocities, (capacity * 2.0_f64));
    state.scales = reserve_float32_array(&state.scales, capacity);
    state.rotation_speeds = reserve_float32_array(&state.rotation_speeds, capacity);
}
