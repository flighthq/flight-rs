// @generated from upstream/packages/socket/src/socket.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    Socket, SocketBackend, SocketCloseInfo, SocketConnection, SocketEventSink, SocketMessage,
    SocketOptions, SocketReadyState, SocketRuntime, SocketSignals,
};

// Source: upstream/packages/socket/src/socket.ts:17 (sha256:c0eba7161fd3531ec11dd2250639972a85b279d91bc055fb54957a1478e59cfa)
pub fn attach_socket(socket: &mut Socket) -> () {
    socket.runtime.delivering = true;
}

// Source: upstream/packages/socket/src/socket.ts:24 (sha256:0e3ba67045fae5ad761e3c0a74a34af228114ee6add36f08ed1345f7781d79b1)
pub fn close_socket(socket: &mut Socket, code: Option<f64>, reason: Option<String>) -> () {
    if ((socket.runtime.ready_state).clone() == "closing")
        || ((socket.runtime.ready_state).clone() == "closed")
    {
        return;
    }
    socket.runtime.ready_state = "closing".to_owned();
    {
        let __flight_callback = socket
            .runtime
            .connection
            .as_mut()
            .unwrap()
            .close_socket_connection
            .as_ref()
            .unwrap()
            .clone();
        let __flight_result = __flight_callback.lock().unwrap()(code, (reason).clone());
        __flight_result
    };
}

// Source: upstream/packages/socket/src/socket.ts:36 (sha256:9267ddb639208c7963a1411016c49d9436aa0f39e3a3308ae9e0623f0a26095a)
pub fn create_socket(options: &SocketOptions) -> Socket {
    let mut runtime: SocketRuntime = SocketRuntime {
        __flight_identity: std::sync::Arc::new(()),
        connection: None,
        signals: None,
        ready_state: "connecting".to_owned(),
        delivering: true,
    };
    let mut socket: Socket = Socket {
        __flight_identity: std::sync::Arc::new(()),
        url: (options.url).clone(),
        runtime: (runtime).clone(),
    };
    socket.runtime.connection = {
        let __flight_callback = (get_socket_backend().open_socket).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (*options).clone(),
            make_socket_event_sink((socket.runtime).clone()),
        );
        __flight_result
    };
    return socket;
}

// Source: upstream/packages/socket/src/socket.ts:52 (sha256:c495e1eda04a84d7016bba32bd0196f75fc215a124d9e1f6d2e3c1409570ba0f)
pub fn create_web_socket_backend() -> SocketBackend {
    return SocketBackend {
        __flight_identity: std::sync::Arc::new(()),
        open_socket: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: SocketOptions, events: SocketEventSink| -> Option<SocketConnection> {
                return None;
            },
        )
            as Box<
                dyn FnMut(SocketOptions, SocketEventSink) -> Option<SocketConnection>
                    + Send
                    + 'static,
            >)),
    };
}

// Source: upstream/packages/socket/src/socket.ts:82 (sha256:3fcfca6300ac00a0048297a5aaa3f053db8c8448aa882ffd9350f7b3466bade0)
pub fn detach_socket(socket: &mut Socket) -> () {
    socket.runtime.delivering = false;
}

// Source: upstream/packages/socket/src/socket.ts:89 (sha256:2ed571366abbac982c72493c5de1cbe6ca021d5451fa04ec9e991a53075bddb0)
pub fn dispose_socket(socket: &mut Socket) -> () {
    close_socket(socket, None, None);
    detach_socket(socket);
    socket.runtime.signals = None;
}

// Source: upstream/packages/socket/src/socket.ts:98 (sha256:870a0a145fd2a265acc7ba57a14474b906454f90daad5f4a18125f0390524da2)
pub fn enable_socket_signals(socket: &mut Socket) -> SocketSignals {
    if ((socket.runtime.signals).clone()).is_none() {
        socket.runtime.signals = Some(SocketSignals {
            __flight_identity: std::sync::Arc::new(()),
            on_socket_open: create_signal(),
            on_socket_message: create_signal(),
            on_socket_close: create_signal(),
            on_socket_error: create_signal(),
        });
    }
    return ((socket.runtime.signals).clone()).unwrap();
}

// Source: upstream/packages/socket/src/socket.ts:112 (sha256:139860d3c558d97b05d71b51d814cfa201cba4fecf36e1b15e9cf132bd3a4d38)
pub fn get_socket_backend() -> SocketBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_socket_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/socket/src/socket.ts:118 (sha256:51bc08949197ed721cc8f797add000aa23ec337ff407f0eb0da4d23483e7182d)
pub fn get_socket_ready_state(socket: &Socket) -> SocketReadyState {
    return (socket.runtime.ready_state).clone();
}

// Source: upstream/packages/socket/src/socket.ts:124 (sha256:7a7a83744be405548de446f329307c11e0fcc867e907822517aa26a8a7f73e7c)
pub fn send_socket_message(socket: &Socket, data: &crate::FlightUnion2<String, Vec<u8>>) -> bool {
    if ((socket.runtime.ready_state).clone() != "open")
        || (((socket.runtime.connection).clone()).is_none())
    {
        return false;
    }
    return {
        let __flight_callback = (socket
            .runtime
            .connection
            .as_ref()
            .unwrap()
            .send_socket_frame)
            .clone();
        let __flight_result = __flight_callback.lock().unwrap()((*data).clone());
        __flight_result
    };
}

// Source: upstream/packages/socket/src/socket.ts:131 (sha256:7e6cc71f55473a95d9efdc5409b4d1d2515f4a9607d28a8178bbe8cd771e9503)
pub fn set_socket_backend(backend: Option<SocketBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/socket/src/socket.ts:135 (sha256:02c38b70551b0563dfd295210862b4bd33a5f9d442cc0b89c5c882888bd69c09)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<SocketBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/socket/src/socket.ts:140 (sha256:2e919419729f0f461c35a7dfa3d9bde0ae4cb9ad94232deebf0b1c348fc8f5d4)
fn make_socket_event_sink(mut runtime: SocketRuntime) -> SocketEventSink {
    return SocketEventSink {
        __flight_identity: std::sync::Arc::new(()),
        handle_socket_open: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut runtime = runtime.clone();
            move || -> () {
                if (!runtime.delivering) {
                    return;
                }
                runtime.ready_state = "open".to_owned();
                if ((runtime.signals).clone()).is_some() {
                    emit_signal(
                        (runtime.signals.as_mut().unwrap().on_socket_open).clone(),
                        (),
                    );
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        handle_socket_message: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut runtime = runtime.clone();
            move |message: SocketMessage| -> () {
                if (!runtime.delivering) {
                    return;
                }
                if ((runtime.signals).clone()).is_some() {
                    emit_signal(
                        (runtime.signals.as_ref().unwrap().on_socket_message).clone(),
                        ((message).clone(),),
                    );
                }
            }
        })
            as Box<dyn FnMut(SocketMessage) -> () + Send + 'static>)),
        handle_socket_close: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut runtime = runtime.clone();
            move |info: SocketCloseInfo| -> () {
                if (!runtime.delivering) {
                    return;
                }
                runtime.ready_state = "closed".to_owned();
                if ((runtime.signals).clone()).is_some() {
                    emit_signal(
                        (runtime.signals.as_mut().unwrap().on_socket_close).clone(),
                        ((info).clone(),),
                    );
                }
            }
        })
            as Box<dyn FnMut(SocketCloseInfo) -> () + Send + 'static>)),
        handle_socket_error: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut runtime = runtime.clone();
            move || -> () {
                if (!runtime.delivering) {
                    return;
                }
                if ((runtime.signals).clone()).is_some() {
                    emit_signal(
                        (runtime.signals.as_ref().unwrap().on_socket_error).clone(),
                        (),
                    );
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    };
}

// Source: upstream/packages/socket/src/socket.ts:165 (sha256:bc19cc2f23e8031bc1e2836d35411297f051b7dc7e93b977dd02c04518edc230)
#[derive(Clone, Default)]
struct ToSocketMessageRecord1 {
    __flight_identity: std::sync::Arc<()>,
    data: Vec<u8>,
    binary: bool,
}
impl PartialEq for ToSocketMessageRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn to_socket_message(data: crate::OpaqueHostValue) -> SocketMessage {
    if (match &(data) {
        crate::OpaqueHostValue::Undefined => "undefined",
        crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
        crate::OpaqueHostValue::Bool(_) => "boolean",
        crate::OpaqueHostValue::Number(_) => "number",
        crate::OpaqueHostValue::String(_) => "string",
    } == "string")
    {
        return SocketMessage {
            __flight_identity: std::sync::Arc::new(()),
            data: data,
            binary: false,
        };
    }
    return SocketMessage {
        __flight_identity: std::sync::Arc::new(()),
        data: crate::FlightUnion2::<String, Vec<u8>>::B(data),
        binary: true,
    };
}
