// @generated from upstream/packages/particles/src/applyParticleForces.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PARTICLE_VELOCITY_STRIDE as particle_velocity_stride_constant;
use flighthq_types::{
    AttractorForce, DragForce, ForceFalloff, ParticleEmitter2D, ParticleEmitterState,
    ParticleForce, ParticleObject, ParticleObjectsState, TurbulenceForce, VortexForce, WindForce,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/particles/src/applyParticleForces.ts:13 (sha256:304b720129965b0734ea48c99769b9cb85145765bb3409e378ff6dcd9f36f047)
static ACCEL: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/particles/src/applyParticleForces.ts:22 (sha256:2c099f93103fc399fd5a56a88f1d2ee137d4a16968e3cb9881961c153bfb5aa3)
pub fn apply_particle_forces(
    emitter: &ParticleEmitter2D,
    state: &mut ParticleEmitterState,
    forces: &Vec<ParticleForce>,
    delta_time: f64,
) -> () {
    if (delta_time <= 0.0_f64) || ((forces.len() as f64) == 0.0_f64) {
        return;
    }
    let count = emitter.data.particle_count;
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let tt = (i * 4.0_f64);
            let vt = (i * particle_velocity_stride_constant);
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == ACCEL.lock().unwrap().len() {
                    ACCEL.lock().unwrap().push(__flight_value);
                } else {
                    ACCEL.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == ACCEL.lock().unwrap().len() {
                    ACCEL.lock().unwrap().push(__flight_value);
                } else {
                    ACCEL.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == ACCEL.lock().unwrap().len() {
                    ACCEL.lock().unwrap().push(__flight_value);
                } else {
                    ACCEL.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            let pz = if ((emitter.data.positions_z.len() as f64) > i) {
                (emitter.data.positions_z[i as usize] as f64) as f32
            } else {
                (0.0_f64) as f32
            };
            accumulate_forces(
                forces,
                (emitter.data.transforms[tt as usize] as f64),
                (emitter.data.transforms[(tt + 1.0_f64) as usize] as f64),
                (pz).clone(),
                (state.velocities[vt as usize] as f64),
                (state.velocities[(vt + 1.0_f64) as usize] as f64),
                (state.velocities[(vt + 2.0_f64) as usize] as f64),
                &mut ACCEL,
            );
            state.velocities[vt as usize] += (ACCEL[0.0_f64 as usize].clone() * delta_time) as f32;
            state.velocities[(vt + 1.0_f64) as usize] +=
                (ACCEL[1.0_f64 as usize].clone() * delta_time) as f32;
            state.velocities[(vt + 2.0_f64) as usize] +=
                (ACCEL[2.0_f64 as usize].clone() * delta_time) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/particles/src/applyParticleForces.ts:59 (sha256:ff1bad45e2a6ebe0fb5a360290b4702f6bbb8465d53782cd9d0b19be31b4bc6d)
pub fn apply_particle_object_forces(
    objects: &Vec<ParticleObject>,
    state: &mut ParticleObjectsState,
    forces: &Vec<ParticleForce>,
    delta_time: f64,
) -> () {
    if (delta_time <= 0.0_f64) || ((forces.len() as f64) == 0.0_f64) {
        return;
    }
    {
        let mut i = 0.0_f64;
        while (i < (objects.len() as f64)) {
            if ((state.lifetimes[((i * 2.0_f64) + 1.0_f64) as usize] as f64) <= 0.0_f64) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let vt = (i * 2.0_f64);
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == ACCEL.lock().unwrap().len() {
                    ACCEL.lock().unwrap().push(__flight_value);
                } else {
                    ACCEL.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == ACCEL.lock().unwrap().len() {
                    ACCEL.lock().unwrap().push(__flight_value);
                } else {
                    ACCEL.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == ACCEL.lock().unwrap().len() {
                    ACCEL.lock().unwrap().push(__flight_value);
                } else {
                    ACCEL.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            accumulate_forces(
                forces,
                objects[i as usize].x,
                objects[i as usize].y,
                0.0_f64,
                (state.velocities[vt as usize] as f64),
                (state.velocities[(vt + 1.0_f64) as usize] as f64),
                0.0_f64,
                &mut ACCEL,
            );
            state.velocities[vt as usize] += (ACCEL[0.0_f64 as usize].clone() * delta_time) as f32;
            state.velocities[(vt + 1.0_f64) as usize] +=
                (ACCEL[1.0_f64 as usize].clone() * delta_time) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/particles/src/applyParticleForces.ts:80 (sha256:ae5eab2a2e8a425781585c2145303028d440bdfcb6ad0df00e325396cc2e815e)
fn accumulate_forces(
    forces: &Vec<ParticleForce>,
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    out: &mut Vec<f64>,
) -> () {
    {
        let mut f = 0.0_f64;
        while (f < (forces.len() as f64)) {
            let force = forces[f as usize].clone();
            {
                let __switch_value = match &((force).clone()) {
                    crate::FlightUnion2::A(value) => (value).kind.clone(),
                    crate::FlightUnion2::B(value) => match value {
                        crate::FlightUnion2::A(value) => (value).kind.clone(),
                        crate::FlightUnion2::B(value) => match value {
                            crate::FlightUnion2::A(value) => (value).kind.clone(),
                            crate::FlightUnion2::B(value) => match value {
                                crate::FlightUnion2::A(value) => (value).kind.clone(),
                                crate::FlightUnion2::B(value) => (value).kind.clone(),
                            },
                        },
                    },
                };
                let __flight_case = if __switch_value == "WindForce" {
                    0_usize
                } else if __switch_value == "DragForce" {
                    1_usize
                } else if __switch_value == "AttractorForce" {
                    2_usize
                } else if __switch_value == "VortexForce" {
                    3_usize
                } else if __switch_value == "TurbulenceForce" {
                    4_usize
                } else {
                    5_usize
                };
                '__flight_switch: {
                    if __flight_case <= 0_usize {
                        out[0.0_f64 as usize] += force.x;
                        out[1.0_f64 as usize] += force.y;
                        out[2.0_f64 as usize] += force.z;
                        break '__flight_switch;
                    }
                    if __flight_case <= 1_usize {
                        out[0.0_f64 as usize] -= (force.strength * vx);
                        out[1.0_f64 as usize] -= (force.strength * vy);
                        out[2.0_f64 as usize] -= (force.strength * vz);
                        break '__flight_switch;
                    }
                    if __flight_case <= 2_usize {
                        {
                            let fz = force.z;
                            let dx = (force.x - px);
                            let dy = (force.y - py);
                            let dz = (fz - pz);
                            let dist = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
                            if (dist <= 0.000001_f64) {
                                break '__flight_switch;
                            }
                            let mag = (force.strength
                                * falloff_factor(
                                    Some((force.falloff).clone()),
                                    dist,
                                    Some(force.radius),
                                ));
                            if (mag == 0.0_f64) {
                                break '__flight_switch;
                            }
                            out[0.0_f64 as usize] += ((dx / dist) * mag);
                            out[1.0_f64 as usize] += ((dy / dist) * mag);
                            out[2.0_f64 as usize] += ((dz / dist) * mag);
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 3_usize {
                        {
                            let fz = force.z;
                            let dx = (px - force.x);
                            let dy = (py - force.y);
                            let dz = (pz - fz);
                            let dist = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
                            if (dist <= 0.000001_f64) {
                                break '__flight_switch;
                            }
                            let mag = (force.strength
                                * falloff_factor(
                                    Some((force.falloff).clone()),
                                    dist,
                                    Some(force.radius),
                                ));
                            if (mag == 0.0_f64) {
                                break '__flight_switch;
                            }
                            let ax = force.axis_x;
                            let ay = force.axis_y;
                            let az = force.axis_z;
                            let inv_dist = (1.0_f64 / dist);
                            let rx = (dx * inv_dist);
                            let ry = (dy * inv_dist);
                            let rz = (dz * inv_dist);
                            out[0.0_f64 as usize] += (((ay * rz) - (az * ry)) * mag);
                            out[1.0_f64 as usize] += (((az * rx) - (ax * rz)) * mag);
                            out[2.0_f64 as usize] += (((ax * ry) - (ay * rx)) * mag);
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 4_usize {
                        {
                            let s = force.scale;
                            out[0.0_f64 as usize] +=
                                (((value_noise((px * s), (py * s), 0.0_f64) * 2.0_f64) - 1.0_f64)
                                    * force.strength);
                            out[1.0_f64 as usize] +=
                                (((value_noise((px * s), (py * s), 1.0_f64) * 2.0_f64) - 1.0_f64)
                                    * force.strength);
                            out[2.0_f64 as usize] +=
                                (((value_noise((px * s), (pz * s), 2.0_f64) * 2.0_f64) - 1.0_f64)
                                    * force.strength);
                            break '__flight_switch;
                        }
                    }
                }
            }
            {
                f += 1.0;
                f
            };
        }
    }
}

// Source: upstream/packages/particles/src/applyParticleForces.ts:151 (sha256:f4308d680e0d146e014c6de02dd91d8a4050b61a0d31a5899f98fe18c31c36a8)
fn falloff_factor(falloff: Option<ForceFalloff>, dist: f64, radius: Option<f64>) -> f64 {
    if (((radius).is_some()) && ((radius).as_ref().is_some_and(|value| *value > 0.0_f64)))
        && ((radius).as_ref().is_some_and(|value| dist > *value))
    {
        return 0.0_f64;
    }
    {
        let __switch_value = falloff;
        let __flight_case = if __switch_value == "linear" {
            0_usize
        } else if __switch_value == "inverseSquare" {
            1_usize
        } else {
            2_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return if ((radius).is_some())
                    && ((radius).as_ref().is_some_and(|value| *value > 0.0_f64))
                {
                    (0.0_f64).max((1.0_f64 - (dist / *(radius.as_ref().unwrap()))))
                } else {
                    1.0_f64
                };
            }
            if __flight_case <= 1_usize {
                {
                    let d = if (dist < 1.0_f64) { 1.0_f64 } else { dist };
                    return (1.0_f64 / (d * d));
                }
            }
            if __flight_case <= 2_usize {
                return 1.0_f64;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}

// Source: upstream/packages/particles/src/applyParticleForces.ts:167 (sha256:cef6aab2d9a0aba6ba2edc2833904b55659a315f2abd7b8c4ccef85560f10246)
fn value_noise(x: f64, y: f64, seed: f64) -> f64 {
    let x0 = (x).floor();
    let y0 = (y).floor();
    let fx = (x - x0);
    let fy = (y - y0);
    let ux = ((fx * fx) * (3.0_f64 - (2.0_f64 * fx)));
    let uy = ((fy * fy) * (3.0_f64 - (2.0_f64 * fy)));
    let n00 = hash2(x0, y0, seed);
    let n10 = hash2((x0 + 1.0_f64), y0, seed);
    let n01 = hash2(x0, (y0 + 1.0_f64), seed);
    let n11 = hash2((x0 + 1.0_f64), (y0 + 1.0_f64), seed);
    let nx0 = (n00 + ((n10 - n00) * ux));
    let nx1 = (n01 + ((n11 - n01) * ux));
    return (nx0 + ((nx1 - nx0) * uy));
}

// Source: upstream/packages/particles/src/applyParticleForces.ts:183 (sha256:3e7699a23edb19be608018a48f20a81f2ee0988f1e145efda10a0defb7ef4d41)
fn hash2(x: f64, y: f64, seed: f64) -> f64 {
    let mut h = (__flight_js_to_i32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                __flight_js_to_i32(x).wrapping_mul(__flight_js_to_i32(668265261.0_f64)) as f64,
            ) ^ __flight_js_to_i32(
                __flight_js_to_i32(y).wrapping_mul(__flight_js_to_i32(374761393.0_f64)) as f64,
            )) as f64,
        ) ^ __flight_js_to_i32(
            __flight_js_to_i32((seed + 1.0_f64)).wrapping_mul(__flight_js_to_i32(2654435761.0_f64))
                as f64,
        )) as f64,
    ) | __flight_js_to_i32(0.0_f64)) as f64;
    h = __flight_js_to_i32(
        (__flight_js_to_i32(h)
            ^ __flight_js_to_i32(
                (__flight_js_to_u32(h) >> (__flight_js_to_u32(15.0_f64) & 31)) as f64,
            )) as f64,
    )
    .wrapping_mul(__flight_js_to_i32(2246822507.0_f64)) as f64;
    h = __flight_js_to_i32(
        (__flight_js_to_i32(h)
            ^ __flight_js_to_i32(
                (__flight_js_to_u32(h) >> (__flight_js_to_u32(13.0_f64) & 31)) as f64,
            )) as f64,
    )
    .wrapping_mul(__flight_js_to_i32(3266489909.0_f64)) as f64;
    return ((__flight_js_to_u32(
        (__flight_js_to_i32(h)
            ^ __flight_js_to_i32(
                (__flight_js_to_u32(h) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
            )) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64
        / 4294967296.0_f64);
}
