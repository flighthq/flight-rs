// @generated from upstream/packages/application/src/window.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::WindowBackend;

// Source: upstream/packages/application/src/window.ts:574 (sha256:9f0742acf83a54ca621d73cf94284a142bbfd43b2a29c54942af9f4bca88cfe7)
pub fn set_window_backend(backend: &Option<WindowBackend>) -> () {
    (*_WINDOW_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/application/src/window.ts:684 (sha256:7f675fc35dcb2bdca66d04a67f9a9380dbd5491de37d22e545cf3b348a093e27)
static _WINDOW_BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<WindowBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
