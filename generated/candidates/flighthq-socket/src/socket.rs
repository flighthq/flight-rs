// @generated from upstream/packages/socket/src/socket.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    Socket, SocketBackend, SocketCloseInfo, SocketConnection, SocketEventSink, SocketGuard,
    SocketGuardNotice, SocketMessage, SocketOptions, SocketReadyState, SocketRuntime,
    SocketSignals,
};

// Source: upstream/packages/socket/src/socket.ts:19 (sha256:f636c66f5d16dadbbe4d1bedd3cda008f7b2b30248a4ae78e15bf1ff917ef053)
pub fn attach_socket(socket: &mut Socket) -> () {
    if socket.runtime.disposed {
        return;
    }
    socket.runtime.delivering = true;
}

// Source: upstream/packages/socket/src/socket.ts:27 (sha256:9bece9e36eb3f342b1b28a6d5d53ea1aaa7e6f22add65c5b0b3ea984019a3092)
pub fn close_socket(socket: &mut Socket, code: Option<f64>, reason: Option<String>) -> () {
    if socket.runtime.disposed {
        {
            let __flight_callback = (*_GUARD.lock().unwrap()).clone();
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()(SocketGuardNotice {
                    __flight_identity: std::sync::Arc::new(()),
                    operation: "closeSocket".to_owned(),
                    reason: "disposed".to_owned(),
                    socket: (*socket).clone(),
                })
            })
        };
        return;
    }
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
            .as_ref()
            .map(|value| (value.close_socket_connection).clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()(code, (reason).clone()))
    };
}

// Source: upstream/packages/socket/src/socket.ts:43 (sha256:330c01c8013d5fb64a72136f5d68fcf6ac6dc416d270846cc79300a7b68c8e92)
pub fn create_socket(options: &SocketOptions) -> Socket {
    let mut runtime: SocketRuntime = SocketRuntime {
        __flight_identity: std::sync::Arc::new(()),
        connection: None,
        signals: None,
        ready_state: "connecting".to_owned(),
        delivering: true,
        disposed: false,
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
    if ((socket.runtime.connection).clone()).is_none() {
        {
            let __flight_callback = (*_GUARD.lock().unwrap()).clone();
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()(SocketGuardNotice {
                    __flight_identity: std::sync::Arc::new(()),
                    operation: "createSocket".to_owned(),
                    reason: "no-connection".to_owned(),
                    socket: (socket).clone(),
                })
            })
        };
    }
    return socket;
}

// Source: upstream/packages/socket/src/socket.ts:61 (sha256:c495e1eda04a84d7016bba32bd0196f75fc215a124d9e1f6d2e3c1409570ba0f)
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

// Source: upstream/packages/socket/src/socket.ts:91 (sha256:3fcfca6300ac00a0048297a5aaa3f053db8c8448aa882ffd9350f7b3466bade0)
pub fn detach_socket(socket: &mut Socket) -> () {
    socket.runtime.delivering = false;
}

// Source: upstream/packages/socket/src/socket.ts:98 (sha256:6cf84be641474b87a35a8d72db6b4c11217ba9c44a5740ad0e22073c410d3f1e)
pub fn dispose_socket(socket: &mut Socket) -> () {
    if socket.runtime.disposed {
        return;
    }
    close_socket(socket, None, None);
    detach_socket(socket);
    socket.runtime.connection = None;
    socket.runtime.signals = None;
    socket.runtime.ready_state = "closed".to_owned();
    socket.runtime.disposed = true;
}

// Source: upstream/packages/socket/src/socket.ts:113 (sha256:fbc9eba83005df84ae5acbdb371f4084aa8dd5e9e239770a5ad529e3a38ce5ee)
pub fn enable_socket_signals(socket: &mut Socket) -> SocketSignals {
    if socket.runtime.disposed {
        {
            let __flight_callback = (*_GUARD.lock().unwrap()).clone();
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()(SocketGuardNotice {
                    __flight_identity: std::sync::Arc::new(()),
                    operation: "enableSocketSignals".to_owned(),
                    reason: "disposed".to_owned(),
                    socket: (*socket).clone(),
                })
            })
        };
    }
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

// Source: upstream/packages/socket/src/socket.ts:128 (sha256:139860d3c558d97b05d71b51d814cfa201cba4fecf36e1b15e9cf132bd3a4d38)
pub fn get_socket_backend() -> SocketBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_socket_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/socket/src/socket.ts:134 (sha256:51bc08949197ed721cc8f797add000aa23ec337ff407f0eb0da4d23483e7182d)
pub fn get_socket_ready_state(socket: &Socket) -> SocketReadyState {
    return (socket.runtime.ready_state).clone();
}

// Source: upstream/packages/socket/src/socket.ts:141 (sha256:47770731939e210f5480df8350c4c624193678b327f3f0ca7859ee2cfe2b8d62)
pub fn send_socket_message(socket: &Socket, data: &crate::FlightUnion2<String, Vec<u8>>) -> bool {
    if socket.runtime.disposed {
        {
            let __flight_callback = (*_GUARD.lock().unwrap()).clone();
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()(SocketGuardNotice {
                    __flight_identity: std::sync::Arc::new(()),
                    operation: "sendSocketMessage".to_owned(),
                    reason: "disposed".to_owned(),
                    socket: (*socket).clone(),
                })
            })
        };
        return false;
    }
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

// Source: upstream/packages/socket/src/socket.ts:152 (sha256:7e6cc71f55473a95d9efdc5409b4d1d2515f4a9607d28a8178bbe8cd771e9503)
pub fn set_socket_backend(backend: &Option<SocketBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (*backend).clone();
}

// Source: upstream/packages/socket/src/socket.ts:158 (sha256:211c0b14edc5f66e03d3d224ac23f6f92e92c85b043cba21bf1beb4c6d0be1d9)
pub fn set_socket_guard(guard: &Option<SocketGuard>) -> () {
    (*_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/socket/src/socket.ts:162 (sha256:02c38b70551b0563dfd295210862b4bd33a5f9d442cc0b89c5c882888bd69c09)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<SocketBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/socket/src/socket.ts:163 (sha256:864807b2d29c1e70183dbd1ebdfd26bf3bd48f88512ba44bb2ad01fad354cd6e)
static _GUARD: std::sync::LazyLock<std::sync::Mutex<Option<SocketGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/socket/src/socket.ts:168 (sha256:2e919419729f0f461c35a7dfa3d9bde0ae4cb9ad94232deebf0b1c348fc8f5d4)
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

// Source: upstream/packages/socket/src/socket.ts:193 (sha256:bc19cc2f23e8031bc1e2836d35411297f051b7dc7e93b977dd02c04518edc230)
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

fn to_socket_message(data: crate::FlightValue) -> SocketMessage {
    if ((match &((data).clone()) {
        crate::FlightValue::Undefined => "undefined",
        crate::FlightValue::Null
        | crate::FlightValue::Array(_)
        | crate::FlightValue::Record(_)
        | crate::FlightValue::Error { .. }
        | crate::FlightValue::Object => "object",
        crate::FlightValue::Bool(_) => "boolean",
        crate::FlightValue::Number(_) => "number",
        crate::FlightValue::String(_) => "string",
        crate::FlightValue::Function => "function",
        crate::FlightValue::Symbol => "symbol",
    })
    .to_owned()
        == "string")
    {
        return SocketMessage {
            __flight_identity: std::sync::Arc::new(()),
            data: match (data).clone() {
                crate::FlightValue::String(value) => {
                    crate::FlightUnion2::<String, Vec<u8>>::A(value)
                }
                crate::FlightValue::Array(values) => crate::FlightUnion2::<String, Vec<u8>>::B(
                    values
                        .into_iter()
                        .map(|value| match value {
                            crate::FlightValue::Number(value) => value as u8,
                            _ => {
                                panic!("TypeScript typed-array cast received a non-numeric element")
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                _ => panic!("TypeScript union cast received an incompatible portable value"),
            },
            binary: false,
        };
    }
    return SocketMessage {
        __flight_identity: std::sync::Arc::new(()),
        data: crate::FlightUnion2::<String, Vec<u8>>::B(match (data).clone() {
            crate::FlightValue::Array(values) => values
                .into_iter()
                .map(|value| match value {
                    crate::FlightValue::Number(value) => value as u8,
                    _ => panic!("TypeScript typed-array cast received a non-numeric element"),
                })
                .collect::<Vec<_>>(),
            _ => panic!("TypeScript typed-array cast received a non-array portable value"),
        }),
        binary: true,
    };
}
