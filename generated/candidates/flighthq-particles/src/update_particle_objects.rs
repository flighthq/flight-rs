// @generated from upstream/packages/particles/src/updateParticleObjects.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ensure_particle_objects_state_capacity, sample_particle_curve};
use flighthq_types::{ParticleEmitterConfig, ParticleObjectsState};
pub use flighthq_types::{ParticleObject, ParticleObjectsUpdateOptions};

// Source: upstream/packages/particles/src/updateParticleObjects.ts:13 (sha256:eed66b9413dd1f3589bacb1e13051ec02a5deec60c911cd17bad429c276ac74b)
const TWO_PI: f64 = 6.283185307179586_f64;

// Source: upstream/packages/particles/src/updateParticleObjects.ts:18 (sha256:ca6a6c8a65cde89ff473264c19ea104382fe800c00a5a26bd9aad4799c05dcb8)
pub fn is_particle_objects_complete(
    objects: &Vec<ParticleObject>,
    state: &ParticleObjectsState,
    config: &ParticleEmitterConfig,
) -> bool {
    if (config.duration <= 0.0_f64) || (config.loop_) {
        return false;
    }
    if (state.emitter_age < config.duration) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (objects.len() as f64)) {
            if objects[i as usize].visible {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/particles/src/updateParticleObjects.ts:31 (sha256:334a4673e88fac352b19d03343995e3fdd2fd749b32027f53e562109a986c853)
pub fn update_particle_objects(
    objects: &mut Vec<ParticleObject>,
    state: &mut ParticleObjectsState,
    config: &ParticleEmitterConfig,
    delta_time: f64,
    options: Option<ParticleObjectsUpdateOptions>,
) -> () {
    let n = (objects.len() as f64);
    if (n == 0.0_f64) {
        return;
    }
    if (delta_time <= 0.0_f64) {
        return;
    }
    ensure_particle_objects_state_capacity(state, n);
    let gx = (config.gravity_x * delta_time);
    let gy = (config.gravity_y * delta_time);
    let alpha_curve = (config.alpha_curve).clone();
    let scale_curve = (config.scale_curve).clone();
    let has_alpha_curve =
        ((alpha_curve).is_some()) && (alpha_curve.as_ref().unwrap().length > 0.0_f64);
    let has_scale_curve =
        ((scale_curve).is_some()) && (scale_curve.as_ref().unwrap().length > 0.0_f64);
    let has_scale_anim = (config.scale_end != 1.0_f64) || (has_scale_curve);
    let has_rot_speed =
        (config.rotation_speed_min != 0.0_f64) || (config.rotation_speed_max != 0.0_f64);
    let emitter_x = (options.as_ref().and_then(|value| value.emitter_x)).unwrap_or(f64::NAN);
    let emitter_y = (options.as_ref().and_then(|value| value.emitter_y)).unwrap_or(f64::NAN);
    let mut emitter_vel_x = 0.0_f64;
    let mut emitter_vel_y = 0.0_f64;
    if ((config.velocity_inheritance != 0.0_f64) && (!(emitter_x).is_nan()))
        && (!(state.prev_x).is_nan())
    {
        emitter_vel_x = ((emitter_x - state.prev_x) / delta_time);
        emitter_vel_y = ((emitter_y - state.prev_y) / delta_time);
    }
    let on_death = options
        .as_ref()
        .unwrap()
        .callbacks
        .as_ref()
        .and_then(|value| (value.on_death).clone());
    {
        let mut i = 0.0_f64;
        while (i < n) {
            let lt = (i * 2.0_f64);
            if ((state.lifetimes[(lt + 1.0_f64) as usize] as f64) <= 0.0_f64) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            state.lifetimes[lt as usize] += (delta_time) as f32;
            if ((state.lifetimes[lt as usize] as f64)
                >= (state.lifetimes[(lt + 1.0_f64) as usize] as f64))
            {
                state.lifetimes[(lt + 1.0_f64) as usize] = (0.0_f64) as f32;
                objects[i as usize].visible = false;
                {
                    let __flight_callback = on_death;
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()())
                };
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let vt = (i * 2.0_f64);
            state.velocities[vt as usize] += (gx) as f32;
            state.velocities[(vt + 1.0_f64) as usize] += (gy) as f32;
            objects[i as usize].x += ((state.velocities[vt as usize] as f64) * delta_time);
            objects[i as usize].y +=
                ((state.velocities[(vt + 1.0_f64) as usize] as f64) * delta_time);
            let life_fraction = ((state.lifetimes[lt as usize] as f64)
                / (state.lifetimes[(lt + 1.0_f64) as usize] as f64));
            objects[i as usize].alpha = if has_alpha_curve {
                sample_particle_curve(alpha_curve.as_ref().unwrap(), life_fraction)
            } else {
                (config.alpha_start + ((config.alpha_end - config.alpha_start) * life_fraction))
            };
            if has_scale_anim {
                let factor = if has_scale_curve {
                    sample_particle_curve(scale_curve.as_ref().unwrap(), life_fraction)
                } else {
                    (1.0_f64 + ((config.scale_end - 1.0_f64) * life_fraction))
                };
                let s = ((state.scales[i as usize] as f64) * factor);
                objects[i as usize].scale_x = s;
                objects[i as usize].scale_y = s;
            }
            if has_rot_speed {
                objects[i as usize].rotation +=
                    ((state.rotation_speeds[i as usize] as f64) * delta_time);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let emitting =
        ((config.duration <= 0.0_f64) || (config.loop_)) || (state.emitter_age < config.duration);
    if (config.duration > 0.0_f64) && (!config.loop_) {
        state.emitter_age += delta_time;
    }
    state.spawn_accumulator += if emitting {
        (config.spawn_rate * delta_time)
    } else {
        0.0_f64
    };
    let mut to_spawn = (state.spawn_accumulator).floor();
    state.spawn_accumulator -= to_spawn;
    if (emitting) && (config.burst_count > 0.0_f64) {
        state.burst_timer -= delta_time;
        if (state.burst_timer <= 0.0_f64) {
            to_spawn += config.burst_count;
            state.burst_timer = if (config.burst_interval > 0.0_f64) {
                config.burst_interval
            } else {
                f64::INFINITY
            };
        }
    }
    if (to_spawn > 0.0_f64) {
        let base_angle = (config.direction_y).atan2(config.direction_x);
        let rot_speed_range = (config.rotation_speed_max - config.rotation_speed_min);
        let on_spawn = options
            .as_ref()
            .unwrap()
            .callbacks
            .as_ref()
            .and_then(|value| (value.on_spawn).clone());
        {
            let mut i = 0.0_f64;
            while (i < n) && (to_spawn > 0.0_f64) {
                let lt = (i * 2.0_f64);
                if ((state.lifetimes[(lt + 1.0_f64) as usize] as f64) > 0.0_f64) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let lifetime = (config.lifetime_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.lifetime_max - config.lifetime_min)));
                state.lifetimes[lt as usize] = (0.0_f64) as f32;
                state.lifetimes[(lt + 1.0_f64) as usize] = (lifetime) as f32;
                let angle = (base_angle
                    + ((({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } - 0.5_f64)
                        * 2.0_f64)
                        * config.spread));
                let speed = (config.speed_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.speed_max - config.speed_min)));
                let vt = (i * 2.0_f64);
                state.velocities[vt as usize] = (((angle).cos() * speed)
                    + if (config.velocity_inheritance != 0.0_f64) {
                        (emitter_vel_x * config.velocity_inheritance)
                    } else {
                        0.0_f64
                    }) as f32;
                state.velocities[(vt + 1.0_f64) as usize] = (((angle).sin() * speed)
                    + if (config.velocity_inheritance != 0.0_f64) {
                        (emitter_vel_y * config.velocity_inheritance)
                    } else {
                        0.0_f64
                    }) as f32;
                let mut spawn_x = 0.0_f64;
                let mut spawn_y = 0.0_f64;
                if ((config.emitter_shape).clone() == "circle") && (config.emitter_radius > 0.0_f64)
                {
                    let r = (({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    })
                    .sqrt()
                        * config.emitter_radius);
                    let a = ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * TWO_PI);
                    spawn_x = ((a).cos() * r);
                    spawn_y = ((a).sin() * r);
                } else {
                    if ((config.emitter_shape).clone() == "rect")
                        && ((config.emitter_width > 0.0_f64) || (config.emitter_height > 0.0_f64))
                    {
                        spawn_x = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_width);
                        spawn_y = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_height);
                    }
                }
                let spawn_scale = (config.scale_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.scale_max - config.scale_min)));
                state.scales[i as usize] = (spawn_scale) as f32;
                state.rotation_speeds[i as usize] = if has_rot_speed {
                    (config.rotation_speed_min
                        + ({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } * rot_speed_range)) as f32
                } else {
                    (0.0_f64) as f32
                };
                let mut obj = objects[i as usize].clone();
                obj.x = spawn_x;
                obj.y = spawn_y;
                obj.rotation = angle;
                let spawn_factor = if has_scale_curve {
                    (spawn_scale * sample_particle_curve(scale_curve.as_ref().unwrap(), 0.0_f64))
                } else {
                    spawn_scale
                };
                obj.scale_x = spawn_factor;
                obj.scale_y = spawn_factor;
                obj.alpha = if has_alpha_curve {
                    sample_particle_curve(alpha_curve.as_ref().unwrap(), 0.0_f64)
                } else {
                    config.alpha_start
                };
                obj.visible = true;
                {
                    to_spawn -= 1.0;
                    to_spawn
                };
                {
                    let __flight_callback = on_spawn;
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()(spawn_x, spawn_y))
                };
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    if (!(emitter_x).is_nan()) {
        state.prev_x = emitter_x;
        state.prev_y = emitter_y;
    }
}
