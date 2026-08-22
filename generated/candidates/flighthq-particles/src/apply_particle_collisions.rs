// @generated from upstream/packages/particles/src/applyParticleCollisions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PARTICLE_VELOCITY_STRIDE as particle_velocity_stride_constant;
use flighthq_types::{
    CircleCollider, ParticleCollider, ParticleEmitter2D, ParticleEmitterState, ParticleObject,
    ParticleObjectsState, PlaneCollider, RectangleCollider, SphereCollider,
};

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:16 (sha256:b0504d79fada24e50ed4f4fbdbeda39c8489d8e4e07759c08a78a6ed97085477)
static S: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64])
});

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:24 (sha256:c521242097ffb6d6f97a08e41ab8d4f4a88e91895efd1babefd351a553193d5e)
pub fn apply_particle_collisions(
    emitter: &mut ParticleEmitter2D,
    state: &mut ParticleEmitterState,
    colliders: &Vec<ParticleCollider>,
) -> () {
    if ((colliders.len() as f64) == 0.0_f64) {
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
                let __flight_value = (emitter.data.transforms[tt as usize] as f64);
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = (emitter.data.transforms[(tt + 1.0_f64) as usize] as f64);
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = if ((emitter.data.positions_z.len() as f64) > i) {
                    (emitter.data.positions_z[i as usize] as f64)
                } else {
                    0.0_f64
                };
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (3.0_f64) as usize;
                let __flight_value = (state.velocities[vt as usize] as f64);
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (4.0_f64) as usize;
                let __flight_value = (state.velocities[(vt + 1.0_f64) as usize] as f64);
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (5.0_f64) as usize;
                let __flight_value = (state.velocities[(vt + 2.0_f64) as usize] as f64);
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            if resolve_colliders(colliders, &mut (*S.lock().unwrap())) {
                emitter.data.transforms[tt as usize] =
                    ((*S.lock().unwrap())[0.0_f64 as usize].clone()) as f32;
                emitter.data.transforms[(tt + 1.0_f64) as usize] =
                    ((*S.lock().unwrap())[1.0_f64 as usize].clone()) as f32;
                if ((emitter.data.positions_z.len() as f64) > i) {
                    emitter.data.positions_z[i as usize] =
                        ((*S.lock().unwrap())[2.0_f64 as usize].clone()) as f32;
                }
                state.velocities[vt as usize] =
                    ((*S.lock().unwrap())[3.0_f64 as usize].clone()) as f32;
                state.velocities[(vt + 1.0_f64) as usize] =
                    ((*S.lock().unwrap())[4.0_f64 as usize].clone()) as f32;
                state.velocities[(vt + 2.0_f64) as usize] =
                    ((*S.lock().unwrap())[5.0_f64 as usize].clone()) as f32;
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:56 (sha256:b8d637498e7e6315c456d72a3ebc5c48de4c1ccd03e31d269bf45f0b40d97f45)
pub fn apply_particle_object_collisions(
    objects: &mut Vec<ParticleObject>,
    state: &mut ParticleObjectsState,
    colliders: &Vec<ParticleCollider>,
) -> () {
    if ((colliders.len() as f64) == 0.0_f64) {
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
                let __flight_value = objects[i as usize].x;
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = objects[i as usize].y;
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (3.0_f64) as usize;
                let __flight_value = (state.velocities[vt as usize] as f64);
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (4.0_f64) as usize;
                let __flight_value = (state.velocities[(vt + 1.0_f64) as usize] as f64);
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (5.0_f64) as usize;
                let __flight_value = 0.0_f64;
                if __flight_index == S.lock().unwrap().len() {
                    S.lock().unwrap().push(__flight_value);
                } else {
                    S.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            if resolve_colliders(colliders, &mut (*S.lock().unwrap())) {
                objects[i as usize].x = (*S.lock().unwrap())[0.0_f64 as usize].clone();
                objects[i as usize].y = (*S.lock().unwrap())[1.0_f64 as usize].clone();
                state.velocities[vt as usize] =
                    ((*S.lock().unwrap())[3.0_f64 as usize].clone()) as f32;
                state.velocities[(vt + 1.0_f64) as usize] =
                    ((*S.lock().unwrap())[4.0_f64 as usize].clone()) as f32;
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:82 (sha256:d50f926b7f60766da2638bdb2ea2ae1b1bb36986bf631dabc24ebb73232992d8)
fn resolve_colliders(colliders: &Vec<ParticleCollider>, p: &mut Vec<f64>) -> bool {
    let mut hit = false;
    {
        let mut c = 0.0_f64;
        while (c < (colliders.len() as f64)) {
            let collider = colliders[c as usize].clone();
            {
                let __switch_value = match &((collider).clone()) {
                    crate::FlightUnion2::A(value) => (value).kind.clone(),
                    crate::FlightUnion2::B(value) => match value {
                        crate::FlightUnion2::A(value) => (value).kind.clone(),
                        crate::FlightUnion2::B(value) => match value {
                            crate::FlightUnion2::A(value) => (value).kind.clone(),
                            crate::FlightUnion2::B(value) => (value).kind.clone(),
                        },
                    },
                };
                let __flight_case = if __switch_value == "PlaneCollider" {
                    0_usize
                } else if __switch_value == "CircleCollider" {
                    1_usize
                } else if __switch_value == "RectangleCollider" {
                    2_usize
                } else if __switch_value == "SphereCollider" {
                    3_usize
                } else {
                    4_usize
                };
                '__flight_switch: {
                    if __flight_case <= 0_usize {
                        hit = (resolve_plane(&collider, p)) || (hit);
                        break '__flight_switch;
                    }
                    if __flight_case <= 1_usize {
                        hit = (resolve_circle(&collider, p)) || (hit);
                        break '__flight_switch;
                    }
                    if __flight_case <= 2_usize {
                        hit = (resolve_rect(&collider, p)) || (hit);
                        break '__flight_switch;
                    }
                    if __flight_case <= 3_usize {
                        hit = (resolve_sphere(&collider, p)) || (hit);
                        break '__flight_switch;
                    }
                }
            }
            {
                c += 1.0;
                c
            };
        }
    }
    return hit;
}

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:107 (sha256:2f7629357555182b9698fe333030d6fc59746e9b88fb8ee25d10c575812bc5f9)
fn resolve_plane(c: &PlaneCollider, p: &mut Vec<f64>) -> bool {
    let nz = (c.nz).unwrap_or(0.0_f64);
    let depth = ((((c.nx * p[0.0_f64 as usize].clone()) + (c.ny * p[1.0_f64 as usize].clone()))
        + (nz * p[2.0_f64 as usize].clone()))
        - c.distance);
    if (depth >= 0.0_f64) {
        return false;
    }
    p[0.0_f64 as usize] -= (c.nx * depth);
    p[1.0_f64 as usize] -= (c.ny * depth);
    p[2.0_f64 as usize] -= (nz * depth);
    reflect3(
        p,
        c.nx,
        c.ny,
        nz,
        (c.restitution).unwrap_or(0.0_f64),
        (c.friction).unwrap_or(0.0_f64),
    );
    return true;
}

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:118 (sha256:abfc6f0860a1f50d9f69f48f9dac1b72355dbc858b348234b5600222ea363bba)
fn resolve_circle(c: &CircleCollider, p: &mut Vec<f64>) -> bool {
    let dx = (p[0.0_f64 as usize].clone() - c.x);
    let dy = (p[1.0_f64 as usize].clone() - c.y);
    let dist = ((dx * dx) + (dy * dy)).sqrt();
    if ((c.mode).clone() == "exclude") {
        if (dist >= c.radius) || (dist <= 0.000001_f64) {
            return false;
        }
        let nx = (dx / dist);
        let ny = (dy / dist);
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = (c.x + (nx * c.radius));
            if __flight_index == p.len() {
                p.push(__flight_value);
            } else {
                p[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (1.0_f64) as usize;
            let __flight_value = (c.y + (ny * c.radius));
            if __flight_index == p.len() {
                p.push(__flight_value);
            } else {
                p[__flight_index] = __flight_value;
            }
        };
        reflect3(
            p,
            nx,
            ny,
            0.0_f64,
            (c.restitution).unwrap_or(0.0_f64),
            (c.friction).unwrap_or(0.0_f64),
        );
        return true;
    }
    if (dist <= c.radius) {
        return false;
    }
    let nx = if (dist <= 0.000001_f64) {
        0.0_f64
    } else {
        ((-dx) / dist)
    };
    let ny = if (dist <= 0.000001_f64) {
        (-1.0_f64)
    } else {
        ((-dy) / dist)
    };
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (c.x - (nx * c.radius));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (c.y - (ny * c.radius));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
    reflect3(
        p,
        nx,
        ny,
        0.0_f64,
        (c.restitution).unwrap_or(0.0_f64),
        (c.friction).unwrap_or(0.0_f64),
    );
    return true;
}

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:141 (sha256:6a8f8e64aa64d75f4b21bb36fdd45500a272fa077c782492190344e7a0ba9fa2)
fn resolve_rect(c: &RectangleCollider, p: &mut Vec<f64>) -> bool {
    let hw = (c.width / 2.0_f64);
    let hh = (c.height / 2.0_f64);
    let min_x = (c.x - hw);
    let max_x = (c.x + hw);
    let min_y = (c.y - hh);
    let max_y = (c.y + hh);
    let restitution = (c.restitution).unwrap_or(0.0_f64);
    let friction = (c.friction).unwrap_or(0.0_f64);
    if ((c.mode).clone() == "contain") {
        let mut hit = false;
        if (p[0.0_f64 as usize].clone() < min_x) {
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = min_x;
                if __flight_index == p.len() {
                    p.push(__flight_value);
                } else {
                    p[__flight_index] = __flight_value;
                }
            };
            reflect3(p, 1.0_f64, 0.0_f64, 0.0_f64, restitution, friction);
            hit = true;
        } else {
            if (p[0.0_f64 as usize].clone() > max_x) {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = max_x;
                    if __flight_index == p.len() {
                        p.push(__flight_value);
                    } else {
                        p[__flight_index] = __flight_value;
                    }
                };
                reflect3(p, (-1.0_f64), 0.0_f64, 0.0_f64, restitution, friction);
                hit = true;
            }
        }
        if (p[1.0_f64 as usize].clone() < min_y) {
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = min_y;
                if __flight_index == p.len() {
                    p.push(__flight_value);
                } else {
                    p[__flight_index] = __flight_value;
                }
            };
            reflect3(p, 0.0_f64, 1.0_f64, 0.0_f64, restitution, friction);
            hit = true;
        } else {
            if (p[1.0_f64 as usize].clone() > max_y) {
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = max_y;
                    if __flight_index == p.len() {
                        p.push(__flight_value);
                    } else {
                        p[__flight_index] = __flight_value;
                    }
                };
                reflect3(p, 0.0_f64, (-1.0_f64), 0.0_f64, restitution, friction);
                hit = true;
            }
        }
        return hit;
    }
    if (((p[0.0_f64 as usize].clone() <= min_x) || (p[0.0_f64 as usize].clone() >= max_x))
        || (p[1.0_f64 as usize].clone() <= min_y))
        || (p[1.0_f64 as usize].clone() >= max_y)
    {
        return false;
    }
    let left = (p[0.0_f64 as usize].clone() - min_x);
    let right = (max_x - p[0.0_f64 as usize].clone());
    let top = (p[1.0_f64 as usize].clone() - min_y);
    let bottom = (max_y - p[1.0_f64 as usize].clone());
    let min_pen = (((left).min(right)).min(top)).min(bottom);
    if (min_pen == left) {
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = min_x;
            if __flight_index == p.len() {
                p.push(__flight_value);
            } else {
                p[__flight_index] = __flight_value;
            }
        };
        reflect3(p, (-1.0_f64), 0.0_f64, 0.0_f64, restitution, friction);
    } else {
        if (min_pen == right) {
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = max_x;
                if __flight_index == p.len() {
                    p.push(__flight_value);
                } else {
                    p[__flight_index] = __flight_value;
                }
            };
            reflect3(p, 1.0_f64, 0.0_f64, 0.0_f64, restitution, friction);
        } else {
            if (min_pen == top) {
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = min_y;
                    if __flight_index == p.len() {
                        p.push(__flight_value);
                    } else {
                        p[__flight_index] = __flight_value;
                    }
                };
                reflect3(p, 0.0_f64, (-1.0_f64), 0.0_f64, restitution, friction);
            } else {
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = max_y;
                    if __flight_index == p.len() {
                        p.push(__flight_value);
                    } else {
                        p[__flight_index] = __flight_value;
                    }
                };
                reflect3(p, 0.0_f64, 1.0_f64, 0.0_f64, restitution, friction);
            }
        }
    }
    return true;
}

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:197 (sha256:accd0258cfb57476868e77990de8724615063eb9ef0725dfe6653011585fbdcb)
fn resolve_sphere(c: &SphereCollider, p: &mut Vec<f64>) -> bool {
    let dx = (p[0.0_f64 as usize].clone() - c.x);
    let dy = (p[1.0_f64 as usize].clone() - c.y);
    let dz = (p[2.0_f64 as usize].clone() - c.z);
    let dist = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
    if ((c.mode).clone() == "exclude") {
        if (dist >= c.radius) || (dist <= 0.000001_f64) {
            return false;
        }
        let nx = (dx / dist);
        let ny = (dy / dist);
        let nz = (dz / dist);
        {
            let __flight_index = (0.0_f64) as usize;
            let __flight_value = (c.x + (nx * c.radius));
            if __flight_index == p.len() {
                p.push(__flight_value);
            } else {
                p[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (1.0_f64) as usize;
            let __flight_value = (c.y + (ny * c.radius));
            if __flight_index == p.len() {
                p.push(__flight_value);
            } else {
                p[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = (2.0_f64) as usize;
            let __flight_value = (c.z + (nz * c.radius));
            if __flight_index == p.len() {
                p.push(__flight_value);
            } else {
                p[__flight_index] = __flight_value;
            }
        };
        reflect3(
            p,
            nx,
            ny,
            nz,
            (c.restitution).unwrap_or(0.0_f64),
            (c.friction).unwrap_or(0.0_f64),
        );
        return true;
    }
    if (dist <= c.radius) {
        return false;
    }
    let nx = if (dist <= 0.000001_f64) {
        0.0_f64
    } else {
        ((-dx) / dist)
    };
    let ny = if (dist <= 0.000001_f64) {
        0.0_f64
    } else {
        ((-dy) / dist)
    };
    let nz = if (dist <= 0.000001_f64) {
        (-1.0_f64)
    } else {
        ((-dz) / dist)
    };
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (c.x - (nx * c.radius));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (c.y - (ny * c.radius));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (c.z - (nz * c.radius));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
    reflect3(
        p,
        nx,
        ny,
        nz,
        (c.restitution).unwrap_or(0.0_f64),
        (c.friction).unwrap_or(0.0_f64),
    );
    return true;
}

// Source: upstream/packages/particles/src/applyParticleCollisions.ts:228 (sha256:6c7ea9f94c234b8385254ede2cb81e22d5bc06c60b4c69eccd2874f370660383)
fn reflect3(p: &mut Vec<f64>, nx: f64, ny: f64, nz: f64, restitution: f64, friction: f64) -> () {
    let vn = (((p[3.0_f64 as usize].clone() * nx) + (p[4.0_f64 as usize].clone() * ny))
        + (p[5.0_f64 as usize].clone() * nz));
    if (vn >= 0.0_f64) {
        return;
    }
    let tvx = (p[3.0_f64 as usize].clone() - (vn * nx));
    let tvy = (p[4.0_f64 as usize].clone() - (vn * ny));
    let tvz = (p[5.0_f64 as usize].clone() - (vn * nz));
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = ((tvx * (1.0_f64 - friction)) - ((restitution * vn) * nx));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (4.0_f64) as usize;
        let __flight_value = ((tvy * (1.0_f64 - friction)) - ((restitution * vn) * ny));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (5.0_f64) as usize;
        let __flight_value = ((tvz * (1.0_f64 - friction)) - ((restitution * vn) * nz));
        if __flight_index == p.len() {
            p.push(__flight_value);
        } else {
            p[__flight_index] = __flight_value;
        }
    };
}
