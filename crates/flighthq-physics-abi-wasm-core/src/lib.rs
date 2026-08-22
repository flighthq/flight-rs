#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::marker::PhantomData;

pub const ABI_VERSION: u32 = 1;
pub const CAPABILITIES: u32 = (1 << 1) | (1 << 3);
pub const WORLD_STALE: u32 = 0;
pub const WORLD_READY: u32 = 1;

pub const EXEC_COMPLETE: u32 = 0;
pub const EXEC_INVALID_BUFFER: u32 = 2;
pub const EXEC_INVALID_COMMAND: u32 = 3;
pub const EXEC_MISSING_BODY: u32 = 4;
pub const EXEC_MISSING_COLLIDER: u32 = 5;
pub const EXEC_MISSING_JOINT: u32 = 6;
pub const EXEC_REJECTED_MUTATION: u32 = 7;
pub const EXEC_STALE_WORLD: u32 = 8;

pub const STEP_COMPLETE: u32 = 0;
pub const STEP_DECLINED: u32 = 2;
pub const STEP_STALE_WORLD: u32 = 4;

const HEADER_LENGTH: usize = 16;
const RECORD_HEADER_LENGTH: usize = 16;
const BODY_TYPE_MASK: u32 = 0b11;
const BODY_FIXED_ROTATION: u32 = 1 << 2;
const BODY_SLEEPING: u32 = 1 << 4;
const KNOWN_BODY_FLAGS: u32 = 0b11_1111;

const SET_GRAVITY: u32 = 1;
const SET_SOLVER_CONFIG: u32 = 2;
const SET_BODY: u32 = 3;
const DESTROY_BODY: u32 = 4;
const SET_COLLIDER: u32 = 5;
const DESTROY_COLLIDER: u32 = 6;
const SET_JOINT: u32 = 7;
const DESTROY_JOINT: u32 = 8;
const APPLY_FORCE: u32 = 9;
const APPLY_FORCE_AT_POINT: u32 = 10;
const APPLY_LINEAR_IMPULSE: u32 = 11;
const APPLY_LINEAR_IMPULSE_AT_POINT: u32 = 12;
const APPLY_TORQUE: u32 = 13;
const WAKE_BODY: u32 = 14;

pub trait PhysicsSpec: 'static {
    const MAGIC: u32;
    const DIMENSIONS: usize;
    const BODY_STRIDE: usize;
    const SET_GRAVITY_LENGTH: usize;
    const SET_BODY_LENGTH: usize;
    const SET_JOINT_LENGTH: usize;
    const BODY_ACTION_LENGTH: usize;
    const DEFAULT_GRAVITY: &'static [f64];

    fn apply_action(body: &mut Body, kind: u32, values: &[f64]);
    fn integrate(body: &mut Body, gravity: &[f64], dt: f64);
}

#[derive(Clone, Debug)]
pub struct Body {
    pub flags: u32,
    pub values: Vec<f64>,
}

#[derive(Clone, Debug)]
struct Collider {
    body_id: u32,
}

#[derive(Clone, Debug)]
struct Joint {
    body_a_id: u32,
    body_b_id: u32,
}

#[derive(Clone, Debug)]
struct World<S: PhysicsSpec> {
    gravity: Vec<f64>,
    bodies: BTreeMap<u32, Body>,
    colliders: BTreeMap<u32, Collider>,
    joints: BTreeMap<u32, Joint>,
    marker: PhantomData<S>,
}

impl<S: PhysicsSpec> Default for World<S> {
    fn default() -> Self {
        Self {
            gravity: S::DEFAULT_GRAVITY.to_vec(),
            bodies: BTreeMap::new(),
            colliders: BTreeMap::new(),
            joints: BTreeMap::new(),
            marker: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
struct Abi<S: PhysicsSpec> {
    next_world: u32,
    worlds: BTreeMap<u32, World<S>>,
}

impl<S: PhysicsSpec> Default for Abi<S> {
    fn default() -> Self {
        Self {
            next_world: 1,
            worlds: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AbiStore<S: PhysicsSpec> {
    next_abi: u32,
    abis: BTreeMap<u32, Abi<S>>,
}

impl<S: PhysicsSpec> Default for AbiStore<S> {
    fn default() -> Self {
        Self {
            next_abi: 1,
            abis: BTreeMap::new(),
        }
    }
}

impl<S: PhysicsSpec> AbiStore<S> {
    pub fn create_abi(&mut self) -> u32 {
        let handle = self.next_abi;
        if handle == 0 {
            return 0;
        }
        self.next_abi = self.next_abi.checked_add(1).unwrap_or(0);
        self.abis.insert(handle, Abi::default());
        handle
    }

    pub fn destroy_abi(&mut self, abi: u32) -> bool {
        self.abis.remove(&abi).is_some()
    }

    pub fn create_world(&mut self, abi: u32) -> u32 {
        let Some(instance) = self.abis.get_mut(&abi) else {
            return 0;
        };
        let handle = instance.next_world;
        if handle == 0 {
            return 0;
        }
        instance.next_world = instance.next_world.checked_add(1).unwrap_or(0);
        instance.worlds.insert(handle, World::default());
        handle
    }

    pub fn destroy_world(&mut self, abi: u32, world: u32) -> bool {
        self.abis
            .get_mut(&abi)
            .and_then(|instance| instance.worlds.remove(&world))
            .is_some()
    }

    pub fn world_status(&self, abi: u32, world: u32) -> u32 {
        if self
            .abis
            .get(&abi)
            .is_some_and(|instance| instance.worlds.contains_key(&world))
        {
            WORLD_READY
        } else {
            WORLD_STALE
        }
    }

    pub fn execute(
        &mut self,
        abi: u32,
        world: u32,
        commands: &[u8],
        byte_length: u32,
        command_count: u32,
        result: &mut [u32],
    ) -> bool {
        let Some(world) = self
            .abis
            .get_mut(&abi)
            .and_then(|instance| instance.worlds.get_mut(&world))
        else {
            write_result(result, EXEC_STALE_WORLD, 0, HEADER_LENGTH as u32, 0);
            return false;
        };

        let byte_length = byte_length as usize;
        if result.len() < 4
            || byte_length < HEADER_LENGTH
            || byte_length > commands.len()
            || read_u32(commands, 0) != Some(S::MAGIC)
            || read_u32(commands, 4) != Some(ABI_VERSION)
            || read_u32(commands, 8) != Some(byte_length as u32)
            || read_u32(commands, 12) != Some(command_count)
        {
            write_result(result, EXEC_INVALID_BUFFER, 0, HEADER_LENGTH as u32, 0);
            return false;
        }

        let mut offset = HEADER_LENGTH;
        for command_index in 0..command_count {
            let Some(kind) = read_u32(commands, offset) else {
                write_result(result, EXEC_INVALID_BUFFER, command_index, offset as u32, 0);
                return false;
            };
            let Some(record_length) = read_u32(commands, offset + 4).map(|value| value as usize)
            else {
                write_result(
                    result,
                    EXEC_INVALID_BUFFER,
                    command_index,
                    offset as u32,
                    kind,
                );
                return false;
            };
            if record_length < RECORD_HEADER_LENGTH
                || record_length % 8 != 0
                || offset.checked_add(record_length).is_none()
                || offset + record_length > byte_length
            {
                write_result(
                    result,
                    EXEC_INVALID_BUFFER,
                    command_index,
                    offset as u32,
                    kind,
                );
                return false;
            }
            let object_id = read_u32(commands, offset + 8).unwrap_or(0);
            let related_id = read_u32(commands, offset + 12).unwrap_or(0);
            let status = execute_record::<S>(
                world,
                commands,
                offset,
                record_length,
                kind,
                object_id,
                related_id,
            );
            if status != EXEC_COMPLETE {
                write_result(result, status, command_index, offset as u32, kind);
                return false;
            }
            offset += record_length;
        }
        if offset != byte_length {
            write_result(result, EXEC_INVALID_BUFFER, command_count, offset as u32, 0);
            return false;
        }
        write_result(result, EXEC_COMPLETE, command_count, offset as u32, 0);
        true
    }

    pub fn step(&mut self, abi: u32, world: u32, dt: f64, has_hooks: bool) -> u32 {
        let Some(world) = self
            .abis
            .get_mut(&abi)
            .and_then(|instance| instance.worlds.get_mut(&world))
        else {
            return STEP_STALE_WORLD;
        };
        if has_hooks || !dt.is_finite() || dt < 0.0 {
            return STEP_DECLINED;
        }
        for body in world.bodies.values_mut() {
            S::integrate(body, &world.gravity, dt);
        }
        STEP_COMPLETE
    }

    pub fn read_bodies(
        &self,
        abi: u32,
        world: u32,
        selection: &[u32],
        has_selection: bool,
        ids: &mut [u32],
        flags: &mut [u32],
        values: &mut [f64],
        counts: &mut [u32],
    ) -> bool {
        let Some(world) = self
            .abis
            .get(&abi)
            .and_then(|instance| instance.worlds.get(&world))
        else {
            return false;
        };
        if counts.len() < 2 {
            return false;
        }
        let body_ids: Vec<u32> = if has_selection {
            selection
                .iter()
                .copied()
                .filter(|id| world.bodies.contains_key(id))
                .collect()
        } else {
            world.bodies.keys().copied().collect()
        };
        counts[1] = body_ids.len() as u32;
        let capacity = ids
            .len()
            .min(flags.len())
            .min(values.len() / S::BODY_STRIDE)
            .min(body_ids.len());
        counts[0] = capacity as u32;
        for (row, id) in body_ids.into_iter().take(capacity).enumerate() {
            let body = &world.bodies[&id];
            ids[row] = id;
            flags[row] = body.flags;
            let start = row * S::BODY_STRIDE;
            values[start..start + S::BODY_STRIDE].copy_from_slice(&body.values);
        }
        true
    }

    pub fn read_joints(
        &self,
        abi: u32,
        world: u32,
        ids: &mut [u32],
        flags: &mut [u32],
        values: &mut [f64],
        value_stride: usize,
        counts: &mut [u32],
    ) -> bool {
        let Some(world) = self
            .abis
            .get(&abi)
            .and_then(|instance| instance.worlds.get(&world))
        else {
            return false;
        };
        if counts.len() < 2 || value_stride == 0 {
            return false;
        }
        let required = world.joints.len();
        let capacity = ids
            .len()
            .min(flags.len())
            .min(values.len() / value_stride)
            .min(required);
        counts[0] = capacity as u32;
        counts[1] = required as u32;
        for (row, id) in world.joints.keys().copied().take(capacity).enumerate() {
            ids[row] = id;
            flags[row] = 0;
            values[row * value_stride..(row + 1) * value_stride].fill(0.0);
        }
        true
    }
}

fn execute_record<S: PhysicsSpec>(
    world: &mut World<S>,
    commands: &[u8],
    offset: usize,
    length: usize,
    kind: u32,
    object_id: u32,
    related_id: u32,
) -> u32 {
    let payload = offset + RECORD_HEADER_LENGTH;
    match kind {
        SET_GRAVITY if length == S::SET_GRAVITY_LENGTH => {
            for axis in 0..S::DIMENSIONS {
                let Some(value) = read_f64(commands, payload + axis * 8) else {
                    return EXEC_INVALID_BUFFER;
                };
                if !value.is_finite() {
                    return EXEC_REJECTED_MUTATION;
                }
                world.gravity[axis] = value;
            }
            EXEC_COMPLETE
        }
        SET_SOLVER_CONFIG if length == 96 => EXEC_COMPLETE,
        SET_BODY if length == S::SET_BODY_LENGTH && object_id != 0 && related_id == 0 => {
            let Some(flags) = read_u32(commands, payload) else {
                return EXEC_INVALID_BUFFER;
            };
            if flags & !KNOWN_BODY_FLAGS != 0 || flags & BODY_TYPE_MASK == 3 {
                return EXEC_INVALID_COMMAND;
            }
            let mut values = Vec::with_capacity(S::BODY_STRIDE);
            for index in 0..S::BODY_STRIDE {
                let Some(value) = read_f64(commands, payload + 8 + index * 8) else {
                    return EXEC_INVALID_BUFFER;
                };
                if !value.is_finite() {
                    return EXEC_REJECTED_MUTATION;
                }
                values.push(value);
            }
            world.bodies.insert(object_id, Body { flags, values });
            EXEC_COMPLETE
        }
        DESTROY_BODY if length == RECORD_HEADER_LENGTH && object_id != 0 && related_id == 0 => {
            if world.bodies.remove(&object_id).is_none() {
                return EXEC_MISSING_BODY;
            }
            world
                .colliders
                .retain(|_, collider| collider.body_id != object_id);
            world
                .joints
                .retain(|_, joint| joint.body_a_id != object_id && joint.body_b_id != object_id);
            EXEC_COMPLETE
        }
        SET_COLLIDER if length >= 72 && object_id != 0 && related_id != 0 => {
            if !world.bodies.contains_key(&related_id) {
                return EXEC_MISSING_BODY;
            }
            world.colliders.insert(
                object_id,
                Collider {
                    body_id: related_id,
                },
            );
            EXEC_COMPLETE
        }
        DESTROY_COLLIDER if length == RECORD_HEADER_LENGTH && object_id != 0 && related_id == 0 => {
            if world.colliders.remove(&object_id).is_some() {
                EXEC_COMPLETE
            } else {
                EXEC_MISSING_COLLIDER
            }
        }
        SET_JOINT if length == S::SET_JOINT_LENGTH && object_id != 0 && related_id == 0 => {
            let body_a_id = read_u32(commands, payload + 4).unwrap_or(0);
            let body_b_id = read_u32(commands, payload + 8).unwrap_or(0);
            if !world.bodies.contains_key(&body_a_id) || !world.bodies.contains_key(&body_b_id) {
                return EXEC_MISSING_BODY;
            }
            world.joints.insert(
                object_id,
                Joint {
                    body_a_id,
                    body_b_id,
                },
            );
            EXEC_COMPLETE
        }
        DESTROY_JOINT if length == RECORD_HEADER_LENGTH && object_id != 0 && related_id == 0 => {
            if world.joints.remove(&object_id).is_some() {
                EXEC_COMPLETE
            } else {
                EXEC_MISSING_JOINT
            }
        }
        APPLY_FORCE
        | APPLY_FORCE_AT_POINT
        | APPLY_LINEAR_IMPULSE
        | APPLY_LINEAR_IMPULSE_AT_POINT
        | APPLY_TORQUE
            if length == S::BODY_ACTION_LENGTH && object_id != 0 && related_id == 0 =>
        {
            let Some(body) = world.bodies.get_mut(&object_id) else {
                return EXEC_MISSING_BODY;
            };
            let mut values = Vec::with_capacity((length - RECORD_HEADER_LENGTH) / 8);
            for index in 0..(length - RECORD_HEADER_LENGTH) / 8 {
                let Some(value) = read_f64(commands, payload + index * 8) else {
                    return EXEC_INVALID_BUFFER;
                };
                if !value.is_finite() {
                    return EXEC_REJECTED_MUTATION;
                }
                values.push(value);
            }
            S::apply_action(body, kind, &values);
            EXEC_COMPLETE
        }
        WAKE_BODY if length == RECORD_HEADER_LENGTH && object_id != 0 && related_id == 0 => {
            let Some(body) = world.bodies.get_mut(&object_id) else {
                return EXEC_MISSING_BODY;
            };
            body.flags &= !BODY_SLEEPING;
            EXEC_COMPLETE
        }
        _ => EXEC_INVALID_COMMAND,
    }
}

pub struct Physics2D;

impl PhysicsSpec for Physics2D {
    const MAGIC: u32 = 0x4144_3250;
    const DIMENSIONS: usize = 2;
    const BODY_STRIDE: usize = 17;
    const SET_GRAVITY_LENGTH: usize = 32;
    const SET_BODY_LENGTH: usize = 160;
    const SET_JOINT_LENGTH: usize = 152;
    const BODY_ACTION_LENGTH: usize = 48;
    const DEFAULT_GRAVITY: &'static [f64] = &[0.0, -9.81];

    fn apply_action(body: &mut Body, kind: u32, values: &[f64]) {
        if body.flags & BODY_TYPE_MASK != 0 {
            return;
        }
        let mass = body.values[9];
        let inertia = body.values[10];
        match kind {
            APPLY_FORCE | APPLY_FORCE_AT_POINT => {
                body.values[6] += values[0];
                body.values[7] += values[1];
                if kind == APPLY_FORCE_AT_POINT && body.flags & BODY_FIXED_ROTATION == 0 {
                    let center_x = body.values[0] + body.values[11];
                    let center_y = body.values[1] + body.values[12];
                    body.values[8] +=
                        (values[2] - center_x) * values[1] - (values[3] - center_y) * values[0];
                }
            }
            APPLY_LINEAR_IMPULSE | APPLY_LINEAR_IMPULSE_AT_POINT if mass > 0.0 => {
                body.values[3] += values[0] / mass;
                body.values[4] += values[1] / mass;
                if kind == APPLY_LINEAR_IMPULSE_AT_POINT
                    && inertia > 0.0
                    && body.flags & BODY_FIXED_ROTATION == 0
                {
                    let center_x = body.values[0] + body.values[11];
                    let center_y = body.values[1] + body.values[12];
                    body.values[5] += ((values[2] - center_x) * values[1]
                        - (values[3] - center_y) * values[0])
                        / inertia;
                }
            }
            APPLY_TORQUE => body.values[8] += values[0],
            _ => {}
        }
        body.flags &= !BODY_SLEEPING;
    }

    fn integrate(body: &mut Body, gravity: &[f64], dt: f64) {
        if body.flags & BODY_SLEEPING != 0 || body.flags & BODY_TYPE_MASK == 2 {
            return;
        }
        if body.flags & BODY_TYPE_MASK == 0 {
            let mass = body.values[9];
            if mass > 0.0 {
                body.values[3] += (gravity[0] * body.values[15] + body.values[6] / mass) * dt;
                body.values[4] += (gravity[1] * body.values[15] + body.values[7] / mass) * dt;
            }
            let inertia = body.values[10];
            if inertia > 0.0 && body.flags & BODY_FIXED_ROTATION == 0 {
                body.values[5] += body.values[8] / inertia * dt;
            }
            body.values[6] = 0.0;
            body.values[7] = 0.0;
            body.values[8] = 0.0;
        }
        body.values[0] += body.values[3] * dt;
        body.values[1] += body.values[4] * dt;
        if body.flags & BODY_FIXED_ROTATION == 0 {
            body.values[2] += body.values[5] * dt;
        }
    }
}

pub struct Physics3D;

impl PhysicsSpec for Physics3D {
    const MAGIC: u32 = 0x4144_3350;
    const DIMENSIONS: usize = 3;
    const BODY_STRIDE: usize = 33;
    const SET_GRAVITY_LENGTH: usize = 40;
    const SET_BODY_LENGTH: usize = 288;
    const SET_JOINT_LENGTH: usize = 272;
    const BODY_ACTION_LENGTH: usize = 64;
    const DEFAULT_GRAVITY: &'static [f64] = &[0.0, -9.81, 0.0];

    fn apply_action(body: &mut Body, kind: u32, values: &[f64]) {
        if body.flags & BODY_TYPE_MASK != 0 {
            return;
        }
        let mass = body.values[19];
        match kind {
            APPLY_FORCE | APPLY_FORCE_AT_POINT => {
                for axis in 0..3 {
                    body.values[13 + axis] += values[axis];
                }
                if kind == APPLY_FORCE_AT_POINT && body.flags & BODY_FIXED_ROTATION == 0 {
                    let r = [
                        values[3] - (body.values[0] + body.values[26]),
                        values[4] - (body.values[1] + body.values[27]),
                        values[5] - (body.values[2] + body.values[28]),
                    ];
                    let torque = cross(r, [values[0], values[1], values[2]]);
                    for axis in 0..3 {
                        body.values[16 + axis] += torque[axis];
                    }
                }
            }
            APPLY_LINEAR_IMPULSE | APPLY_LINEAR_IMPULSE_AT_POINT if mass > 0.0 => {
                for axis in 0..3 {
                    body.values[7 + axis] += values[axis] / mass;
                }
                if kind == APPLY_LINEAR_IMPULSE_AT_POINT && body.flags & BODY_FIXED_ROTATION == 0 {
                    let r = [
                        values[3] - (body.values[0] + body.values[26]),
                        values[4] - (body.values[1] + body.values[27]),
                        values[5] - (body.values[2] + body.values[28]),
                    ];
                    let angular = cross(r, [values[0], values[1], values[2]]);
                    for axis in 0..3 {
                        let inertia = body.values[20 + axis];
                        if inertia > 0.0 {
                            body.values[10 + axis] += angular[axis] / inertia;
                        }
                    }
                }
            }
            APPLY_TORQUE => {
                for axis in 0..3 {
                    body.values[16 + axis] += values[axis];
                }
            }
            _ => {}
        }
        body.flags &= !BODY_SLEEPING;
    }

    fn integrate(body: &mut Body, gravity: &[f64], dt: f64) {
        if body.flags & BODY_SLEEPING != 0 || body.flags & BODY_TYPE_MASK == 2 {
            return;
        }
        if body.flags & BODY_TYPE_MASK == 0 {
            let mass = body.values[19];
            if mass > 0.0 {
                for axis in 0..3 {
                    body.values[7 + axis] +=
                        (gravity[axis] * body.values[31] + body.values[13 + axis] / mass) * dt;
                }
            }
            if body.flags & BODY_FIXED_ROTATION == 0 {
                for axis in 0..3 {
                    let inertia = body.values[20 + axis];
                    if inertia > 0.0 {
                        body.values[10 + axis] += body.values[16 + axis] / inertia * dt;
                    }
                }
            }
            body.values[13..19].fill(0.0);
        }
        for axis in 0..3 {
            body.values[axis] += body.values[7 + axis] * dt;
        }
        if body.flags & BODY_FIXED_ROTATION == 0 {
            integrate_quaternion(&mut body.values, dt);
        }
    }
}

fn integrate_quaternion(values: &mut [f64], dt: f64) {
    let [x, y, z, w] = [values[3], values[4], values[5], values[6]];
    let [wx, wy, wz] = [values[10], values[11], values[12]];
    let half_dt = 0.5 * dt;
    let mut next = [
        x + half_dt * (wx * w + wy * z - wz * y),
        y + half_dt * (-wx * z + wy * w + wz * x),
        z + half_dt * (wx * y - wy * x + wz * w),
        w + half_dt * (-wx * x - wy * y - wz * z),
    ];
    let length = next.iter().map(|value| value * value).sum::<f64>().sqrt();
    if length > 0.0 {
        for value in &mut next {
            *value /= length;
        }
        values[3..7].copy_from_slice(&next);
    }
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let data: [u8; 4] = bytes.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_le_bytes(data))
}

fn read_f64(bytes: &[u8], offset: usize) -> Option<f64> {
    let data: [u8; 8] = bytes.get(offset..offset + 8)?.try_into().ok()?;
    Some(f64::from_le_bytes(data))
}

fn write_result(result: &mut [u32], status: u32, index: u32, offset: u32, kind: u32) {
    if result.len() >= 4 {
        result[..4].copy_from_slice(&[status, index, offset, kind]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_handles_are_scoped_and_never_reused() {
        let mut store = AbiStore::<Physics2D>::default();
        let first = store.create_abi();
        let second = store.create_abi();
        assert_eq!(store.create_world(first), 1);
        assert_eq!(store.create_world(second), 1);
        assert!(store.destroy_world(first, 1));
        assert_eq!(store.create_world(first), 2);
    }

    #[test]
    fn stale_world_execution_is_reported_without_reading_the_stream() {
        let mut store = AbiStore::<Physics3D>::default();
        let abi = store.create_abi();
        let mut result = [0; 4];
        assert!(!store.execute(abi, 99, &[], 0, 0, &mut result));
        assert_eq!(result, [EXEC_STALE_WORLD, 0, 16, 0]);
    }
}
