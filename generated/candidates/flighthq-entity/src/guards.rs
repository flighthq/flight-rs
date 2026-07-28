// @generated from upstream/packages/entity/src/guards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::EntityRuntime;

// Source: upstream/packages/entity/src/guards.ts:5 (sha256:a6a6d86e631f0e50d4b1290373dd79ce0a8bf2cd8e09309fea2f41dc8d7dff73)
pub fn are_entity_runtime_guards_enabled() -> bool {
    return _GUARDS_ENABLED.load(std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/entity/src/guards.ts:12 (sha256:0ee15d649f5ff2ddc07a5570ff48f656990db22c9db9bdf84bef7931bc9dbe6e)
pub fn create_guarded_entity<Type: Clone>(entity: Type) -> Type {
    return (entity).clone();
}

// Source: upstream/packages/entity/src/guards.ts:36 (sha256:5335355a591f3f3a60987a874a5a18276cb513faad77ab2763aa898a956c94dc)
pub fn create_guarded_entity_runtime(runtime: EntityRuntime) -> EntityRuntime {
    return (runtime).clone();
}

// Source: upstream/packages/entity/src/guards.ts:60 (sha256:c6dc53a1535aaf94af488f459d1e9c1ca70f71b1bd7d99d7bb7ac37e0d337549)
pub fn enable_entity_runtime_guards() -> () {
    return;
}

// Source: upstream/packages/entity/src/guards.ts:65 (sha256:5bb4b1e497198679b7466bef0df75df8e20d0ddc7ae9c5b14653f31bfbe36d23)
static _GUARDS_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
