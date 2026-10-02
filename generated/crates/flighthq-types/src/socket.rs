// @generated from upstream/packages/types/src/Socket.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

#[derive(Clone, Default)]
pub struct SocketSendFailureExplanationRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub ready_state: SocketReadyState,
    pub url: String,
}
impl PartialEq for SocketSendFailureExplanationRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SocketSendFailureExplanationRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub ready_state: SocketReadyState,
    pub url: String,
}
impl PartialEq for SocketSendFailureExplanationRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SocketSendFailureExplanationRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub ready_state: String,
    pub url: String,
}
impl PartialEq for SocketSendFailureExplanationRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SocketGuardNoticeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: String,
    pub reason: String,
    pub socket: Socket,
}
impl PartialEq for SocketGuardNoticeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SocketGuardNoticeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: String,
    pub reason: String,
    pub socket: Socket,
}
impl PartialEq for SocketGuardNoticeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

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

// Source: upstream/packages/types/src/Socket.ts:41 (sha256:46cf1f2365c7022e6e33eb77a17a9791a8c891ec3cd47159b1fb00decd866868)
#[derive(Clone)]
pub struct SocketSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for SocketSignals {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
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

// Source: upstream/packages/types/src/Socket.ts:72 (sha256:53abb7198316dca16351d5a74462b80be30d4e9dddf9cddc50d9d92e5d96d68d)
#[derive(Clone, Default)]
pub struct TcpSocketOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub host: String,
    pub port: f64,
}
impl PartialEq for TcpSocketOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:81 (sha256:66c3baef09353fee2bb9d3a86eb2c78b842c18ac57e092c1c7ffd9bdf16bff09)
#[derive(Clone)]
pub struct TcpSocketConnection {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub readable: crate::OpaqueHostValue,
    pub writable: crate::OpaqueHostValue,
    pub close_tcp_socket_connection:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for TcpSocketConnection {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:91 (sha256:f46297d300ad080c5abbd29c3f468545f5f9ef5d7b39e95418a0c1ebb410e67c)
#[derive(Clone)]
pub struct HostSocketCapability {
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
    pub open_tcp_socket: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TcpSocketOptions) -> Option<TcpSocketConnection> + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for HostSocketCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Socket.ts:99 (sha256:84a5032e10a50972215d64097cb31bfcac6f4cb43baf03f7b651b7d72bc25864)
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

// Source: upstream/packages/types/src/Socket.ts:113 (sha256:b43a5f2c4f7428074ac562490a35971b7813aa6a07efddd3737c18408863589f)
#[derive(Clone, Default)]
pub struct Socket {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub url: String,
    pub runtime: SocketRuntime,
}
impl PartialEq for Socket {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Socket {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Socket.ts:120 (sha256:11e33cada7d583fc63ef297f7db07195cb25b862cb7fe9efbda30809d56b0a81)
pub type SocketSendFailureExplanation = crate::FlightUnion2<
    SocketSendFailureExplanationRecord3,
    crate::FlightUnion2<SocketSendFailureExplanationRecord2, SocketSendFailureExplanationRecord1>,
>;

// Source: upstream/packages/types/src/Socket.ts:131 (sha256:7d108c6da679b8725737f2dab8b4818ecaef2f4732a0c6db679e705a4837af25)
pub type SocketGuardNotice =
    crate::FlightUnion2<SocketGuardNoticeRecord2, SocketGuardNoticeRecord1>;

// Source: upstream/packages/types/src/Socket.ts:143 (sha256:1131d4997c3a34632582600f4dc965166d26c04891b31cabdb6a4acea2c6b2d4)
pub type SocketGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SocketGuardNotice) -> () + Send + 'static>>>;
