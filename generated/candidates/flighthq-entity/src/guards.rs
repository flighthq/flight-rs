// @generated from upstream/packages/entity/src/guards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{EntityRuntime, EntityRuntimeWriteGuard};

// Source: upstream/packages/entity/src/guards.ts:5 (sha256:a6a6d86e631f0e50d4b1290373dd79ce0a8bf2cd8e09309fea2f41dc8d7dff73)
pub fn are_entity_runtime_guards_enabled() -> bool {
    return _GUARDS_ENABLED.load(std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/entity/src/guards.ts:12 (sha256:f068d5ad12604ef13f6de78bbb5ee945c619071d8a58b92f3abfae720f17f69b)
pub fn create_guarded_entity<Type: Clone>(entity: Type) -> Type {
    return entity;
}

// Source: upstream/packages/entity/src/guards.ts:32 (sha256:bd51a439951af366deaea8f0838b960dceb2e79d0808c535954f9e3417957cec)
pub fn create_guarded_entity_runtime(runtime: EntityRuntime) -> EntityRuntime {
    return runtime;
}

// Source: upstream/packages/entity/src/guards.ts:47 (sha256:2d31a4693324862df6e0ff81ce99ef29fcc9bfa32ed976ec31b4903dc518c1a7)
pub fn set_entity_runtime_guard_mode(enabled: bool) -> () {
    if (enabled) && ("undefined" == "undefined") {
        return;
    }
    _GUARDS_ENABLED.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/entity/src/guards.ts:54 (sha256:a0fe5e2b9aeaba8a73420ca46b29528728b5fc3f721174894f9c17089687a203)
pub fn set_entity_runtime_write_guard(guard: &Option<EntityRuntimeWriteGuard>) -> () {
    (*_WRITE_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/entity/src/guards.ts:58 (sha256:5bb4b1e497198679b7466bef0df75df8e20d0ddc7ae9c5b14653f31bfbe36d23)
static _GUARDS_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Source: upstream/packages/entity/src/guards.ts:59 (sha256:22abc48b9effcf078245c7875ca573fea02de57773ac372ee1d4fdb17d3eef28)
static _WRITE_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<EntityRuntimeWriteGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
