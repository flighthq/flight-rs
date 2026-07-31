// @generated from upstream/packages/power/src/power.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::PowerBackend;

// Source: upstream/packages/power/src/power.ts:350 (sha256:587f4b0078c26c5ab6d469aab12864b71d8f05b42620846cd2c79a708b7f2b9d)
pub fn set_power_backend(backend: Option<PowerBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/power/src/power.ts:367 (sha256:6414a3f1532c56810ee95fc29fc8f6c692e8b42d15f6dfb2a5319a4c14a7aa85)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<PowerBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
