// @generated from upstream/packages/application/src/application.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::LoopBackend;

// Source: upstream/packages/application/src/application.ts:197 (sha256:8b909762712641019ca399eea4014d78e0dbb45ecd396885b291d4721b9d62d0)
pub fn set_loop_backend(backend: &Option<LoopBackend>) -> () {
    (*_LOOP_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/application/src/application.ts:317 (sha256:dd1556279acc82cf7cc2b97b11df1bfc4023c027af22cc948801d28ecc274388)
static _LOOP_BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<LoopBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
