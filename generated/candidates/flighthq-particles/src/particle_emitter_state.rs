// @generated from upstream/packages/particles/src/particleEmitterState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::reserve_float32_array;
use flighthq_types::{ParticleEmitterState, RandomSource};

// Source: upstream/packages/particles/src/particleEmitterState.ts:5 (sha256:9aa4d6bcc3bf248e170935b3e80167526d5e6534f436f4a8a04087e9fd7787d9)
pub const PARTICLE_VELOCITY_STRIDE: f64 = 3.0_f64;

// Source: upstream/packages/particles/src/particleEmitterState.ts:7 (sha256:69cabc20c4767ead164458449fd85c14de145c15b27bf1e6ed5a516a28e0aa7b)
pub fn create_particle_emitter_state(random: Option<RandomSource>) -> ParticleEmitterState {
    let random = random.unwrap_or((math.random).clone());
    return ParticleEmitterState {
        __flight_identity: std::sync::Arc::new(()),
        burst_timer: 0.0_f64,
        color_birth: vec![0.0_f32; (0.0_f64) as usize],
        color_death: vec![0.0_f32; (0.0_f64) as usize],
        emitter_age: 0.0_f64,
        lifetimes: vec![0.0_f32; (0.0_f64) as usize],
        prev_x: f64::NAN,
        prev_y: f64::NAN,
        prev_z: f64::NAN,
        random: (random).clone(),
        rotation_speeds: vec![0.0_f32; (0.0_f64) as usize],
        scales: vec![0.0_f32; (0.0_f64) as usize],
        spawn_accumulator: 0.0_f64,
        velocities: vec![0.0_f32; (0.0_f64) as usize],
    };
}

// Source: upstream/packages/particles/src/particleEmitterState.ts:28 (sha256:9fa47a5955ba74db9e3f501b652af5bae05a7081292b248717743329f543df4a)
pub fn ensure_particle_emitter_state_capacity(
    state: &mut ParticleEmitterState,
    capacity: f64,
    has_color_variance: bool,
) -> () {
    if ((state.lifetimes.len() as f64) >= (capacity * 2.0_f64)) {
        if (has_color_variance) && ((state.color_birth.len() as f64) < (capacity * 3.0_f64)) {
            state.color_birth = reserve_float32_array(&state.color_birth, (capacity * 3.0_f64));
            state.color_death = reserve_float32_array(&state.color_death, (capacity * 3.0_f64));
        }
        return;
    }
    state.lifetimes = reserve_float32_array(&state.lifetimes, (capacity * 2.0_f64));
    state.velocities =
        reserve_float32_array(&state.velocities, (capacity * PARTICLE_VELOCITY_STRIDE));
    state.scales = reserve_float32_array(&state.scales, capacity);
    state.rotation_speeds = reserve_float32_array(&state.rotation_speeds, capacity);
    if has_color_variance {
        state.color_birth = reserve_float32_array(&state.color_birth, (capacity * 3.0_f64));
        state.color_death = reserve_float32_array(&state.color_death, (capacity * 3.0_f64));
    }
}
