import {
  Physics2DAbiBodyValue,
  Physics2DAbiCapability,
  Physics2DAbiVersion,
  createPhysics2DAbiBodyBuffer,
  createPhysics2DAbiCommandBuffer,
  createPhysics2DAbiExecutionResult,
  createPhysics2DAbiQueryBuffer,
  writePhysics2DAbiApplyForceCommand,
  writePhysics2DAbiApplyLinearImpulseCommand,
  writePhysics2DAbiApplyTorqueCommand,
  writePhysics2DAbiDestroyBodyCommand,
  writePhysics2DAbiSetBodyCommand,
  writePhysics2DAbiSetGravityCommand,
} from '@flighthq/physics2d-abi';
import { createRigidBody2D } from '@flighthq/physics2d';

import { createPhysics2DAbi, initPhysics2DAbiWasm } from './physics2DAbiWasm';

describe('Physics2D wasm ABI', () => {
  it('initializes synchronously and exposes only implemented capabilities', () => {
    initPhysics2DAbiWasm();
    initPhysics2DAbiWasm();
    const abi = createPhysics2DAbi();
    expect(abi.version).toBe(Physics2DAbiVersion);
    expect(abi.capabilities).toBe(Physics2DAbiCapability.PersistentWorlds | Physics2DAbiCapability.SelectiveReadback);
  });

  it('owns persistent, non-reused world handles in wasm', () => {
    const abi = createPhysics2DAbi();
    const first = abi.createWorld();
    expect(first).toBeGreaterThan(0);
    expect(abi.getWorldStatus(first)).toBe('Ready');
    expect(abi.destroyWorld(first)).toBe(true);
    expect(abi.getWorldStatus(first)).toBe('Stale');
    expect(abi.createWorld()).toBeGreaterThan(first);
  });

  it('executes packed body commands, integrates state, and publishes selective readback', () => {
    const abi = createPhysics2DAbi();
    const world = abi.createWorld();
    const body = createRigidBody2D('dynamic', 0, 0);
    body.mass = 2;
    body.inertia = 4;
    body.gravityScale = 1;
    const commands = createPhysics2DAbiCommandBuffer();
    expect(writePhysics2DAbiSetGravityCommand(commands, 0, 0)).toBe(true);
    expect(writePhysics2DAbiSetBodyCommand(commands, 7, body)).toBe(true);
    expect(writePhysics2DAbiApplyForceCommand(commands, 7, 4, 0)).toBe(true);
    expect(writePhysics2DAbiApplyLinearImpulseCommand(commands, 7, 2, 0)).toBe(true);
    expect(writePhysics2DAbiApplyTorqueCommand(commands, 7, 4)).toBe(true);

    const result = createPhysics2DAbiExecutionResult();
    expect(abi.execute(world, commands, result)).toBe(true);
    expect(result.status).toBe('Complete');

    const before = createPhysics2DAbiBodyBuffer(1);
    expect(abi.readBodies(world, new Uint32Array([404, 7]), before)).toBe(true);
    expect([before.count, before.requiredCount, before.ids[0]]).toEqual([1, 1, 7]);
    expect(before.values[Physics2DAbiBodyValue.VelocityX]).toBe(1);
    expect(before.values[Physics2DAbiBodyValue.ForceX]).toBe(4);

    expect(abi.step(world, 0.5, null)).toBe('Complete');
    const after = createPhysics2DAbiBodyBuffer(1);
    expect(abi.readBodies(world, null, after)).toBe(true);
    expect(after.values[Physics2DAbiBodyValue.X]).toBe(1);
    expect(after.values[Physics2DAbiBodyValue.VelocityX]).toBe(2);
    expect(after.values[Physics2DAbiBodyValue.AngularVelocity]).toBe(0.5);
    expect(after.values[Physics2DAbiBodyValue.ForceX]).toBe(0);
  });

  it('commits the valid command prefix and reports unsupported capability calls honestly', () => {
    const abi = createPhysics2DAbi();
    const world = abi.createWorld();
    const commands = createPhysics2DAbiCommandBuffer();
    writePhysics2DAbiSetBodyCommand(commands, 1, createRigidBody2D('dynamic', 0, 0));
    writePhysics2DAbiDestroyBodyCommand(commands, 999);
    const result = createPhysics2DAbiExecutionResult();

    expect(abi.execute(world, commands, result)).toBe(false);
    expect(result).toMatchObject({ status: 'MissingBody', commandIndex: 1 });
    const bodies = createPhysics2DAbiBodyBuffer(1);
    expect(abi.readBodies(world, null, bodies)).toBe(true);
    expect(bodies.ids[0]).toBe(1);

    const query = createPhysics2DAbiQueryBuffer(1);
    expect(abi.queryPoint(world, 0, 0, null, query)).toBe(false);
    expect([query.count, query.requiredCount]).toEqual([0, 0]);
  });
});
