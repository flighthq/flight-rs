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
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: crate::FlightUnion2<String, Vec<u8>>,
    pub binary: bool,
}
impl PartialEq for SocketMessage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:25 (sha256:70feadd1c025c4997d89d136902b97e58ff5a753a73a96fafbd94f1424acf306)
#[derive(Clone, Default)]
pub struct SocketOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
    pub protocols: Option<Vec<String>>,
    pub binary_type: Option<String>,
}
impl PartialEq for SocketOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:33 (sha256:9c3671a4e8bb7d9c85eb6b2ea7483bc8a1163721fc43826ac9a6dd254805e450)
#[derive(Clone, Default)]
pub struct SocketCloseInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub code: f64,
    pub reason: String,
    pub was_clean: bool,
}
impl PartialEq for SocketCloseInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:41 (sha256:6ec00e505a6b205f7e6ef6cb85ced4c1ec483fbf906c081b17f8190ef348224e)
#[derive(Clone)]
pub struct SocketSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_socket_open:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_socket_message: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SocketMessage) -> () + Send + 'static>>>,
    >,
    pub on_socket_close: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SocketCloseInfo) -> () + Send + 'static>>>,
    >,
    pub on_socket_error:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for SocketSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:52 (sha256:fa59eb0a48a6d50be9e5ba57d9e5e1bbfe01fb3da2bd0ad10dfaac04699c3dab)
#[derive(Clone)]
pub struct SocketEventSink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle_socket_open:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub handle_socket_message:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SocketMessage) -> () + Send + 'static>>>,
    pub handle_socket_close:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SocketCloseInfo) -> () + Send + 'static>>>,
    pub handle_socket_error:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for SocketEventSink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:62 (sha256:239d0c94edaa155bc81822a40e56d796e5f5c0cfea865d9c3647f36f0a6c7c34)
#[derive(Clone)]
pub struct SocketConnection {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub send_socket_frame: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::FlightUnion2<String, Vec<u8>>) -> bool + Send + 'static>,
        >,
    >,
    pub close_socket_connection: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<f64>, Option<String>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for SocketConnection {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:74 (sha256:a1cee6800d778bc3a71904c8977bf5bdc46ed535e9c3ae20ae7266aa2e9c04a2)
#[derive(Clone)]
pub struct SocketBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_socket: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(SocketOptions, SocketEventSink) -> Option<SocketConnection>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for SocketBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:81 (sha256:84a5032e10a50972215d64097cb31bfcac6f4cb43baf03f7b651b7d72bc25864)
#[derive(Clone, Default)]
pub struct SocketRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub connection: Option<SocketConnection>,
    pub signals: Option<SocketSignals>,
    pub ready_state: SocketReadyState,
    pub delivering: bool,
    pub disposed: bool,
}
impl PartialEq for SocketRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:95 (sha256:b86755aef7f21cdbdf6fe0f9b1b5da2c48bbf6395e26a4466d9c7d69a153cfe6)
#[derive(Clone, Default)]
pub struct Socket {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
    pub runtime: SocketRuntime,
}
impl PartialEq for Socket {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:102 (sha256:11e33cada7d583fc63ef297f7db07195cb25b862cb7fe9efbda30809d56b0a81)
#[derive(Clone)]
pub struct SocketSendFailureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub ready_state: crate::FlightUnion2<String, SocketReadyState>,
    pub url: String,
}
impl PartialEq for SocketSendFailureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:113 (sha256:7d108c6da679b8725737f2dab8b4818ecaef2f4732a0c6db679e705a4837af25)
#[derive(Clone, Default)]
pub struct SocketGuardNotice {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: String,
    pub reason: String,
    pub socket: Socket,
}
impl PartialEq for SocketGuardNotice {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:125 (sha256:1131d4997c3a34632582600f4dc965166d26c04891b31cabdb6a4acea2c6b2d4)
pub type SocketGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SocketGuardNotice) -> () + Send + 'static>>>;
