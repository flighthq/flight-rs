// @generated from upstream/packages/screen/src/screen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ScreenBackend;

// Source: upstream/packages/screen/src/screen.ts:768 (sha256:31514bc7574cf09f3e59a43ab28401928aabd6ba0c3fca9edd57972ed0e323b7)
pub fn set_screen_backend(backend: Option<ScreenBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/screen/src/screen.ts:772 (sha256:478b59e31f645b973aceecaab3659584e2eed603e31838e2afa9caf628c4690c)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<ScreenBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
