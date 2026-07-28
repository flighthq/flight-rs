// @generated from upstream/packages/types/src/Lifecycle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Lifecycle.ts:3 (sha256:931d083a169061720cbac42f89074d27afca46d8a08ce35b69d484cf86048cb1)
pub type AppLifecycleState = String;

// Source: upstream/packages/types/src/Lifecycle.ts:7 (sha256:5a53941968e6de824e4edaa553463f20c24ed90b4606928adb697165bd019b9d)
pub type AppLaunchKind = String;

// Source: upstream/packages/types/src/Lifecycle.ts:11 (sha256:a1d378acb3277cc7ced84be8634c24b28dc9eec481f194eead4e9cff03590f45)
pub type AppMemoryPressure = String;

// Source: upstream/packages/types/src/Lifecycle.ts:16 (sha256:03ca8563ffb8efc48fd12319d2e69063bba9996dbda8ac0dbe4eb56caac633e9)
#[derive(Clone)]
pub struct LifecycleBackend {
    pub get_state: crate::OpaqueHostValue,
    pub subscribe: crate::OpaqueHostValue,
    pub get_launch_kind: Option<crate::OpaqueHostValue>,
    pub subscribe_memory_warning: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/Lifecycle.ts:30 (sha256:e1e5b2e2eae928794ef748e91dc725ac0315d3ea36f3e762f005931fb53e093f)
#[derive(Clone)]
pub struct AppLifecycle {
    pub on_state_change: Signal,
    pub on_resume: Signal,
    pub on_pause: Signal,
    pub on_back_button: Signal,
    pub on_memory_warning: Signal,
    pub on_save_state: Signal,
    pub on_restore_state: Signal,
}
