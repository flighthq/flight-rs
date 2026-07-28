// @generated from upstream/packages/types/src/Socket.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Socket.ts:13 (sha256:4a92d8460f684d30b2a722a73cd2d73940bcb08dcbbc92d8c93581798d4df661)
pub type SocketReadyState = String;

// Source: upstream/packages/types/src/Socket.ts:17 (sha256:a5ca6717996b73b3642aa80d878cae9b19d6bb9413f3ddda51d82c32edff7782)
#[derive(Clone)]
pub struct SocketMessage {
    pub data: crate::OpaqueHostValue,
    pub binary: bool,
}

// Source: upstream/packages/types/src/Socket.ts:25 (sha256:70feadd1c025c4997d89d136902b97e58ff5a753a73a96fafbd94f1424acf306)
#[derive(Clone)]
pub struct SocketOptions {
    pub url: String,
    pub protocols: Option<Vec<String>>,
    pub binary_type: Option<String>,
}

// Source: upstream/packages/types/src/Socket.ts:33 (sha256:9c3671a4e8bb7d9c85eb6b2ea7483bc8a1163721fc43826ac9a6dd254805e450)
#[derive(Clone)]
pub struct SocketCloseInfo {
    pub code: f64,
    pub reason: String,
    pub was_clean: bool,
}

// Source: upstream/packages/types/src/Socket.ts:41 (sha256:6ec00e505a6b205f7e6ef6cb85ced4c1ec483fbf906c081b17f8190ef348224e)
#[derive(Clone)]
pub struct SocketSignals {
    pub on_socket_open: Signal,
    pub on_socket_message: Signal,
    pub on_socket_close: Signal,
    pub on_socket_error: Signal,
}

// Source: upstream/packages/types/src/Socket.ts:52 (sha256:fa59eb0a48a6d50be9e5ba57d9e5e1bbfe01fb3da2bd0ad10dfaac04699c3dab)
#[derive(Clone)]
pub struct SocketEventSink {
    pub handle_socket_open: crate::OpaqueHostValue,
    pub handle_socket_message: crate::OpaqueHostValue,
    pub handle_socket_close: crate::OpaqueHostValue,
    pub handle_socket_error: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Socket.ts:62 (sha256:239d0c94edaa155bc81822a40e56d796e5f5c0cfea865d9c3647f36f0a6c7c34)
#[derive(Clone)]
pub struct SocketConnection {
    pub send_socket_frame: crate::OpaqueHostValue,
    pub close_socket_connection: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Socket.ts:74 (sha256:a1cee6800d778bc3a71904c8977bf5bdc46ed535e9c3ae20ae7266aa2e9c04a2)
#[derive(Clone)]
pub struct SocketBackend {
    pub open_socket: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Socket.ts:81 (sha256:7940256a4633f11cb9216d01300491a24e5452eef9bbc604fa695e6ecda42659)
#[derive(Clone)]
pub struct SocketRuntime {
    pub connection: Option<SocketConnection>,
    pub signals: Option<SocketSignals>,
    pub ready_state: SocketReadyState,
    pub delivering: bool,
}

// Source: upstream/packages/types/src/Socket.ts:92 (sha256:b86755aef7f21cdbdf6fe0f9b1b5da2c48bbf6395e26a4466d9c7d69a153cfe6)
#[derive(Clone)]
pub struct Socket {
    pub url: String,
    pub runtime: SocketRuntime,
}
