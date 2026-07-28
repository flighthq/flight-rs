// @generated from upstream/packages/types/src/Ipc.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Ipc.ts:3 (sha256:f7cdb31590d86af53e30d5330334731416dff4cc04d00950220c73694a5fb245)
#[derive(Clone)]
pub struct IpcBackendCapabilities {
    pub can_handle: bool,
    pub can_invoke: bool,
    pub can_send: bool,
    pub can_target: bool,
}

// Source: upstream/packages/types/src/Ipc.ts:14 (sha256:2180cfb257d9ae642240adc663e0e22b1b2b8939e38428e4ee19ae9c486a8583)
#[derive(Clone)]
pub struct IpcBackend {
    pub send: crate::OpaqueHostValue,
    pub invoke: crate::OpaqueHostValue,
    pub subscribe: crate::OpaqueHostValue,
    pub handle: Option<crate::OpaqueHostValue>,
    pub send_to: Option<crate::OpaqueHostValue>,
    pub get_capabilities: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/Ipc.ts:30 (sha256:e226e76590272b023938169c7a8a42aea87f7f146f9584181caa951ca6fe81bb)
#[derive(Clone)]
pub struct IpcChannel {
    pub name: String,
}

// Source: upstream/packages/types/src/Ipc.ts:36 (sha256:e168782d01079d277401c4a6f52dedd961921b9079c16bbfcc1db98a15839ee1)
#[derive(Clone)]
pub struct IpcMessageEvent {
    pub channel: String,
    pub sender_id: f64,
    pub args: Vec<crate::OpaqueHostValue>,
    pub reply: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Ipc.ts:44 (sha256:859db417db809b19f94bf2dde63794bf6868b68dd6a2dbdaa2e0b440207ba6da)
#[derive(Clone)]
pub struct IpcTarget {
    pub window_id: f64,
}

// Source: upstream/packages/types/src/Ipc.ts:50 (sha256:ae0eff05a15283e757bfd0eea4bb2fa83c79892c08d78cb719d64a75e62ad4c7)
#[derive(Clone, Debug)]
pub struct IpcTimeoutError {
    pub message: String,
    pub channel: String,
    pub timeout_ms: f64,
    pub name: String,
}

impl IpcTimeoutError {
    pub fn new(channel: String, timeout_ms: f64) -> Self {
        Self {
            message: format!(
                "IPC invoke on channel \"{}\" timed out after {}ms",
                channel, timeout_ms
            ),
            channel: channel,
            timeout_ms: timeout_ms,
            name: "IpcTimeoutError".to_owned(),
        }
    }
}

impl std::fmt::Display for IpcTimeoutError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for IpcTimeoutError {}
