import {
  Physics3DAbiBodyValue,
  Physics3DAbiCapability,
  Physics3DAbiVersion,
  createPhysics3DAbiBodyBuffer,
  createPhysics3DAbiCommandBuffer,
  createPhysics3DAbiExecutionResult,
  createPhysics3DAbiQueryBuffer,
  writePhysics3DAbiApplyForceCommand,
  writePhysics3DAbiApplyLinearImpulseCommand,
  writePhysics3DAbiApplyTorqueCommand,
  writePhysics3DAbiDestroyBodyCommand,
  writePhysics3DAbiSetBodyCommand,
  writePhysics3DAbiSetGravityCommand,
} from '@flighthq/physics3d-abi';
import { createRigidBody3D } from '@flighthq/physics3d';

import { createPhysics3DAbi, initPhysics3DAbiWasm } from './physics3DAbiWasm';

describe('Physics3D wasm ABI', () => {
  it('initializes synchronously and exposes only implemented capabilities', () => {
    initPhysics3DAbiWasm();
    initPhysics3DAbiWasm();
    const abi = createPhysics3DAbi();
    expect(abi.version).toBe(Physics3DAbiVersion);
    expect(abi.capabilities).toBe(Physics3DAbiCapability.PersistentWorlds | Physics3DAbiCapability.SelectiveReadback);
  });

  it('owns persistent, non-reused world handles in wasm', () => {
    const abi = createPhysics3DAbi();
    const first = abi.createWorld();
    expect(first).toBeGreaterThan(0);
    expect(abi.getWorldStatus(first)).toBe('Ready');
    expect(abi.destroyWorld(first)).toBe(true);
    expect(abi.getWorldStatus(first)).toBe('Stale');
    expect(abi.createWorld()).toBeGreaterThan(first);
  });

  it('executes packed body commands, integrates state, and publishes selective readback', () => {
    const abi = createPhysics3DAbi();
    const world = abi.createWorld();
    const body = createRigidBody3D();
    body.mass = 2;
    body.inertiaXX = 4;
    body.inertiaYY = 4;
    body.inertiaZZ = 4;
    body.gravityScale = 1;
    const commands = createPhysics3DAbiCommandBuffer();
    expect(writePhysics3DAbiSetGravityCommand(commands, 0, 0, 0)).toBe(true);
    expect(writePhysics3DAbiSetBodyCommand(commands, 7, body)).toBe(true);
    expect(writePhysics3DAbiApplyForceCommand(commands, 7, 4, 0, 0)).toBe(true);
    expect(writePhysics3DAbiApplyLinearImpulseCommand(commands, 7, 2, 0, 0)).toBe(true);
    expect(writePhysics3DAbiApplyTorqueCommand(commands, 7, 0, 0, 4)).toBe(true);

    const result = createPhysics3DAbiExecutionResult();
    expect(abi.execute(world, commands, result)).toBe(true);
    expect(result.status).toBe('Complete');

    const before = createPhysics3DAbiBodyBuffer(1);
    expect(abi.readBodies(world, new Uint32Array([404, 7]), before)).toBe(true);
    expect([before.count, before.requiredCount, before.ids[0]]).toEqual([1, 1, 7]);
    expect(before.values[Physics3DAbiBodyValue.VelocityX]).toBe(1);
    expect(before.values[Physics3DAbiBodyValue.ForceX]).toBe(4);

    expect(abi.step(world, 0.5, null)).toBe('Complete');
    const after = createPhysics3DAbiBodyBuffer(1);
    expect(abi.readBodies(world, null, after)).toBe(true);
    expect(after.values[Physics3DAbiBodyValue.X]).toBe(1);
    expect(after.values[Physics3DAbiBodyValue.VelocityX]).toBe(2);
    expect(after.values[Physics3DAbiBodyValue.AngularVelocityZ]).toBe(0.5);
    expect(after.values[Physics3DAbiBodyValue.ForceX]).toBe(0);
  });

  it('commits the valid command prefix and reports unsupported capability calls honestly', () => {
    const abi = createPhysics3DAbi();
    const world = abi.createWorld();
    const commands = createPhysics3DAbiCommandBuffer();
    writePhysics3DAbiSetBodyCommand(commands, 1, createRigidBody3D());
    writePhysics3DAbiDestroyBodyCommand(commands, 999);
    const result = createPhysics3DAbiExecutionResult();

    expect(abi.execute(world, commands, result)).toBe(false);
    expect(result).toMatchObject({ status: 'MissingBody', commandIndex: 1 });
    const bodies = createPhysics3DAbiBodyBuffer(1);
    expect(abi.readBodies(world, null, bodies)).toBe(true);
    expect(bodies.ids[0]).toBe(1);

    const query = createPhysics3DAbiQueryBuffer(1);
    expect(abi.queryPoint(world, 0, 0, 0, null, query)).toBe(false);
    expect([query.count, query.requiredCount]).toEqual([0, 0]);
  });
});
