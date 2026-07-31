// @generated from upstream/packages/application/src/window.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{connect_signal, create_signal, disconnect_signal, emit_signal};
use flighthq_types::{
    ApplicationWindow, Matrix, RenderState, WindowBackend, WindowBounds, WindowOptions,
};

// Source: upstream/packages/application/src/window.ts:11 (sha256:1c6476d76752137f561655b9f88a2c6d2897a3276a7ddbec2056780732056d47)
static K_CLOSE: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:12 (sha256:094d8f1d9093dc403928d15f903fd8e92c36410890b11c09a72da3e397e36c99)
static K_DROP_FILE: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:13 (sha256:809084895595b9e9ac1045c3ebcf8f0d182ac3a9fec60c7ca644897b9b0e154d)
static K_FOCUS: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:14 (sha256:f638848f56f83b4f7e1a74f2adeaa881c880ad3d584569ddfa5dbc2760609529)
static K_FULLSCREEN: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:15 (sha256:6770d167196f6fc2dd70db48e3f22a164529e330a77d1762526c44539101d836)
static K_MOVE: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:16 (sha256:de78974d9f5ad1978732376d21fac68934d97fad4fbe4a632cd6ed71bcb3f451)
static K_ORIENTATION: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:17 (sha256:6035a5db433f929c472a69fa0e164d504b19d7bbe3a5ec9714d5b8110389e9f3)
static K_RENDER_CONTEXT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:18 (sha256:ce8e0f9e75b7c935dd0e77d015764c053f763e87d13206a5689d947b91d1dc2a)
static K_RENDER_STATE: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:19 (sha256:607317ced5dce1e67bc3b06f26abb8528c342ebe2b5e756ef0ab6f71591e5778)
static K_RESIZE: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:20 (sha256:9bad8385c5901caf1fb184c78c663b5f1d51e0264c3ebbe9669d32420cd18b63)
static K_VISIBILITY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/window.ts:25 (sha256:512a62afddcf3aa0da9a96831a07cfad42679602a1cd0e3ecb4a27b442ea995c)
pub fn attach_window_close(win: ApplicationWindow) -> () {
    let observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_CLOSE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    return;
}

// Source: upstream/packages/application/src/window.ts:45 (sha256:f9c67b766df632b31f065767d6ba39cf31b71cba24fb0554d065a06c2eb751bd)
pub fn attach_window_drop_file(win: ApplicationWindow, element: crate::OpaqueHostValue) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_DROP_FILE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut on_drag_over: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |e: crate::OpaqueHostValue| -> f64 { crate::host_value::<f64>("host.preventDefault") },
    )
        as Box<dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static>));
    let mut on_drop: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let win = win.clone();
        move |e: crate::OpaqueHostValue| -> () {
            crate::host_value::<()>("host.preventDefault");
            for file in (crate::host_value::<Vec<crate::OpaqueHostValue>>("host.Array.from"))
                .iter()
                .cloned()
            {
                emit_signal(
                    (win.on_drop_file).clone(),
                    (crate::host_value::<String>("host.name"),),
                );
            }
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    {
        let __flight_key = *K_DROP_FILE;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_drag_over = on_drag_over.clone();
            let on_drop = on_drop.clone();
            move || -> () {
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:63 (sha256:10fc702aa10315e0f7834cafbaae87024c93f589ca2b557e32fb46d618e3c325)
pub fn attach_window_focus(win: ApplicationWindow, element: crate::OpaqueHostValue) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_FOCUS)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut on_focus: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let win = win.clone();
            move || -> () { emit_signal((win.on_focus_in).clone(), ()) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    let mut on_blur: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let win = win.clone();
            move || -> () { emit_signal((win.on_focus_out).clone(), ()) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    {
        let __flight_key = *K_FOCUS;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_blur = on_blur.clone();
            let on_focus = on_focus.clone();
            move || -> () {
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:76 (sha256:d37f6c313e2d263c49f596c8b598f92c24fbb1f460a58370ff2929ca4bb0464a)
pub fn attach_window_fullscreen(win: ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_FULLSCREEN)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut handler: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let win = win.clone();
            move || -> () { emit_signal((win.on_fullscreen_changed).clone(), ()) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    {
        let __flight_key = *K_FULLSCREEN;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let handler = handler.clone();
            move || -> () { crate::host_value::<()>("host.removeEventListener") }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:87 (sha256:fe2eb02cbf056c8abb4afb4d00697bada952bd702a07a67b0386a046afeb8902)
pub fn attach_window_move(win: ApplicationWindow) -> () {
    let observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_MOVE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    return;
}

// Source: upstream/packages/application/src/window.ts:109 (sha256:4a72131d099dc1de9557e17b4bcfe3e6e7b52164267da0f242dbaaf31966f387)
pub fn attach_window_orientation(win: ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_ORIENTATION)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    if !(crate::host_value::<bool>("host.orientation")) {
        return;
    }
    let mut handler: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let win = win.clone();
            move || -> () { emit_signal((win.on_orientation_changed).clone(), ()) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    {
        let __flight_key = *K_ORIENTATION;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let handler = handler.clone();
            move || -> () { crate::host_value::<()>("host.removeEventListener") }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:118 (sha256:e25adba20f659a1acfe822230d4e632add66f1f64355a820f4a82860e78456b2)
pub fn attach_window_render_context(win: ApplicationWindow, canvas: crate::OpaqueHostValue) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_RENDER_CONTEXT)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut on_context_lost: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let win = win.clone();
        move |e: crate::OpaqueHostValue| -> () {
            crate::host_value::<()>("host.preventDefault");
            emit_signal((win.on_render_context_lost).clone(), ());
        }
    })
        as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>));
    let mut on_context_restored: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let win = win.clone();
        move || -> () { emit_signal((win.on_render_context_restored).clone(), ()) }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    {
        let __flight_key = *K_RENDER_CONTEXT;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_context_lost = on_context_lost.clone();
            let on_context_restored = on_context_restored.clone();
            move || -> () {
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:140 (sha256:9fd9bff083a3fd2fa8d85ecc1a14d864446096a7f05633b0fbc02581693d2fb2)
pub fn attach_window_render_state(
    mut win: ApplicationWindow,
    mut state: RenderState,
    canvas: crate::OpaqueHostValue,
) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_RENDER_STATE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut apply: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut state = state.clone();
            let mut win = win.clone();
            move || -> () {
                crate::host_set("host.width", (win.width * win.device_pixel_ratio).round());
                crate::host_set("host.height", (win.height * win.device_pixel_ratio).round());
                if ((state.render_transform2_d).clone()).is_some() {
                    compute_window_device_transform(
                        &win,
                        state.render_transform2_d.as_mut().unwrap(),
                    );
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    {
        let __flight_callback = (apply).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    connect_signal(&mut win.on_resize, (apply).clone(), None);
    {
        let __flight_key = *K_RENDER_STATE;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let apply = apply.clone();
            let mut win = win.clone();
            move || -> () { disconnect_signal(&mut win.on_resize, (apply).clone()) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:153 (sha256:2cebbab8b9bafa380848474ebc33bbdff79a755dc8045ed3ce4e74865ce0a1f6)
pub fn attach_window_resize(mut win: ApplicationWindow, element: crate::OpaqueHostValue) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_RESIZE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let observer = crate::OpaqueHostValue::Object;
    crate::host_value::<()>("host.observe");
    {
        let __flight_key = *K_RESIZE;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> () {
            crate::host_value::<()>("host.disconnect")
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:168 (sha256:adb664d364c435471d9da9750567e6cf56623df6024182f1592c615025d9265d)
pub fn attach_window_visibility(win: ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(&win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_VISIBILITY)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut handler: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let win = win.clone();
            move || -> () {
                if match &(crate::host_value::<crate::OpaqueHostValue>("host.hidden")) {
                    crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
                    crate::OpaqueHostValue::Bool(value) => *value,
                    crate::OpaqueHostValue::Number(value) => *value != 0.0_f64 && !value.is_nan(),
                    crate::OpaqueHostValue::String(value) => !value.is_empty(),
                    crate::OpaqueHostValue::Object => true,
                } {
                    emit_signal((win.on_deactivate).clone(), ());
                } else {
                    emit_signal((win.on_activate).clone(), ());
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    {
        let __flight_key = *K_VISIBILITY;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let handler = handler.clone();
            move || -> () { crate::host_value::<()>("host.removeEventListener") }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/window.ts:183 (sha256:0167c04c1d975c92c5b68ac39a73fcd5e71b8ee5446aa3604fd9d1f3b9ac0541)
pub fn center_window(win: &ApplicationWindow) -> () {
    {
        let __flight_callback = (get_window_backend().center).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:190 (sha256:1a150ebd11c101de18b31626e68f65e3d2c679d3d0b3ea533372efb57129b7ea)
pub fn close_window(win: &ApplicationWindow) -> bool {
    if (!request_window_close(win)) {
        return false;
    }
    {
        let __flight_callback = (get_window_backend().close).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
    emit_signal((win.on_close).clone(), ());
    return true;
}

// Source: upstream/packages/application/src/window.ts:200 (sha256:618d40df28ec55ab595ff07e6c66586a6cb159243b2e0055fc74339c1dfd4da5)
pub fn compute_window_device_transform(win: &ApplicationWindow, out: &mut Matrix) -> Matrix {
    let scale = win.device_pixel_ratio;
    out.a = scale;
    out.b = 0.0_f64;
    out.c = 0.0_f64;
    out.d = scale;
    out.tx = 0.0_f64;
    out.ty = 0.0_f64;
    return out.clone();
}

// Source: upstream/packages/application/src/window.ts:211 (sha256:6abd74ca39375eb974e4ca96bb75816fc4216bed6ae6ea680c99e8ea8b00424f)
pub fn create_application_window() -> ApplicationWindow {
    return ApplicationWindow {
        __flight_identity: std::sync::Arc::new(()),
        always_on_top: false,
        device_pixel_ratio: 1.0_f64,
        focused: false,
        fullscreen: false,
        height: 0.0_f64,
        icon: "".to_owned(),
        max_height: (-1.0_f64),
        maximized: false,
        max_width: (-1.0_f64),
        min_height: 0.0_f64,
        minimized: false,
        min_width: 0.0_f64,
        opacity: 1.0_f64,
        resizable: true,
        skip_taskbar: false,
        title: "".to_owned(),
        visible: true,
        width: 0.0_f64,
        x: 0.0_f64,
        y: 0.0_f64,
        on_activate: create_signal(),
        on_close: create_signal(),
        on_close_request: create_signal(),
        on_deactivate: create_signal(),
        on_drop_file: create_signal(),
        on_focus_in: create_signal(),
        on_focus_out: create_signal(),
        on_fullscreen_changed: create_signal(),
        on_maximize: create_signal(),
        on_minimize: create_signal(),
        on_move: create_signal(),
        on_orientation_changed: create_signal(),
        on_render_context_lost: create_signal(),
        on_render_context_restored: create_signal(),
        on_resize: create_signal(),
        on_restore: create_signal(),
    };
}

// Source: upstream/packages/application/src/window.ts:256 (sha256:80b8722ea2c95ed6c14634d1d8b6d92b84c81f36191d4b13e3e32ab5fcfebae6)
pub fn create_web_window_backend() -> WindowBackend {
    return WindowBackend {
        __flight_identity: std::sync::Arc::new(()),
        open: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: WindowOptions| -> bool {
                return ("undefined" != "undefined");
            },
        )
            as Box<dyn FnMut(ApplicationWindow, WindowOptions) -> bool + Send + 'static>)),
        close: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        set_title: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_win: ApplicationWindow, title: String| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, String) -> () + Send + 'static>)),
        set_position: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_win: ApplicationWindow, x: f64, y: f64| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>)),
        set_size: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_win: ApplicationWindow, width: f64, height: f64| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>)),
        get_bounds: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |win: ApplicationWindow, mut out: WindowBounds| -> WindowBounds {
                {
                    out.x = win.x;
                    out.y = win.y;
                    out.width = win.width;
                    out.height = win.height;
                    return out;
                }
            },
        )
            as Box<dyn FnMut(ApplicationWindow, WindowBounds) -> WindowBounds + Send + 'static>)),
        minimize: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        maximize: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        restore: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        focus: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        show: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        hide: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        center: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |win: ApplicationWindow| -> () {
                return;
            },
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        set_resizable: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
        set_always_on_top: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
        set_minimum_size: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow,
                  __flight_unused_1: f64,
                  __flight_unused_2: f64|
                  -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>)),
        set_maximum_size: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow,
                  __flight_unused_1: f64,
                  __flight_unused_2: f64|
                  -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>)),
        set_fullscreen: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_win: ApplicationWindow, fullscreen: bool| -> () {
                return;
            },
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
        set_icon: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_win: ApplicationWindow, icon: String| -> () {
                return;
            },
        )
            as Box<dyn FnMut(ApplicationWindow, String) -> () + Send + 'static>)),
        set_opacity: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: f64| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, f64) -> () + Send + 'static>)),
        set_skip_taskbar: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
        set_menu_bar_visible: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
        set_parent: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow,
                  __flight_unused_1: Option<ApplicationWindow>|
                  -> () {},
        )
            as Box<
                dyn FnMut(ApplicationWindow, Option<ApplicationWindow>) -> () + Send + 'static,
            >)),
        set_progress: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: f64| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, f64) -> () + Send + 'static>)),
        request_attention: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
        set_content_protection: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
        flash_window_frame: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>)),
        set_has_shadow: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: ApplicationWindow, __flight_unused_1: bool| -> () {},
        )
            as Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>)),
    };
}

// Source: upstream/packages/application/src/window.ts:360 (sha256:2e0a81fe80dcc984d0aeb5a4c9fd55ec8b3b31bd9e19da8a256337ba7049687e)
pub fn detach_window_close(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_CLOSE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_CLOSE;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:366 (sha256:f4a0897dcf1e4397b25d9c93ee60ff29e858bee76e66421475ab2fe654a66a54)
pub fn detach_window_drop_file(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_DROP_FILE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_DROP_FILE;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:372 (sha256:53e5ba82b1b4e09a0b8bbbffe4394d2b0438e415834ac480f7b485aaa60155d3)
pub fn detach_window_focus(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_FOCUS)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_FOCUS;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:378 (sha256:42b0822bbd56c1fa7fbb5c5b76e262e391f328a81c2c7e39f205b6f9631c4eab)
pub fn detach_window_fullscreen(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_FULLSCREEN)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_FULLSCREEN;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:384 (sha256:6b026c7bfd931ef4b0b7b0ed6881ec26277079df57722bf93d0da3f872af8f74)
pub fn detach_window_move(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_MOVE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_MOVE;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:390 (sha256:b7e74488f5ffc7cbbae2966e26dc79a9767418dea6c119928c23ad7f76f7c598)
pub fn detach_window_orientation(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_ORIENTATION)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_ORIENTATION;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:396 (sha256:74a17f686009a5ec340aa189732b6d5115cfef594c204d69d61641e6bd252c18)
pub fn detach_window_render_context(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_RENDER_CONTEXT)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_RENDER_CONTEXT;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:402 (sha256:71de87de0a7367d8453f2a34968b3ebdc7c51bcddc07da0a068ee9e5d341bd7f)
pub fn detach_window_render_state(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_RENDER_STATE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_RENDER_STATE;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:408 (sha256:0cbbdc50f19dbd7c0ff486c2233b56a9bcb2f577eaad5ca18f2135d1b86e8ad8)
pub fn detach_window_resize(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_RESIZE)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_RESIZE;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:414 (sha256:0b0b8a5af1fab244de84e9cf417ffb8f62996570abe7961fe051e3372e691f2f)
pub fn detach_window_visibility(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_VISIBILITY)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_VISIBILITY;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/window.ts:420 (sha256:cf4882e162fd730ddf1893377d06fc2df352be8ab3202d3b9652e0b373041075)
pub fn dispose_application_window(win: &ApplicationWindow) -> () {
    let mut observers = get_application_window_observers(win);
    for cleanup in (observers
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        {
            let __flight_callback = (cleanup).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
    observers.clear();
}

// Source: upstream/packages/application/src/window.ts:426 (sha256:63210f4392e35f58213aecc96f101c028fc8db618d5183f19de33281200a179a)
pub fn exit_application_fullscreen() -> crate::FlightTask<()> {
    return crate::host_value::<crate::FlightTask<()>>("host.exitFullscreen");
}

// Source: upstream/packages/application/src/window.ts:431 (sha256:5b581ba0ad01e7c4010b5b4ec3dd8ee53274ae7b8af731161a82c85904e3d70d)
pub fn exit_application_pointer_lock() -> crate::FlightTask<()> {
    {
        return crate::FlightTask::ready(
            (),
            crate::FlightTaskOrigin {
                package: "@flighthq/application",
                source: "upstream/packages/application/src/window.ts",
                line: 433_u32,
                column: 12_u32,
                lexical_path: "exitApplicationPointerLock.ready:433:12:9b8874eb7ffe",
                fingerprint: "sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116",
            },
        );
    }
}

// Source: upstream/packages/application/src/window.ts:441 (sha256:aa22f27bf7b4483a2538840a10a68caac2a329fcacc2031d968414cac55d14cb)
pub fn flash_window_frame(win: &ApplicationWindow) -> () {
    {
        let __flight_callback = (get_window_backend().flash_window_frame).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:446 (sha256:04e13a01db7c45b6762382586d44db003e5e3981f1fbf65dec8efcb1abf9fe3e)
pub fn focus_window(win: &mut ApplicationWindow) -> () {
    win.focused = true;
    {
        let __flight_callback = (get_window_backend().focus).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:452 (sha256:fbc343d5319c509f1443fab39e007bd8ec558ca6ee38cb7a4b5fc82d69bfdede)
pub fn get_window_backend() -> WindowBackend {
    if ((*_WINDOW_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_WINDOW_BACKEND.lock().unwrap()) = Some(create_web_window_backend());
    }
    return (((*_WINDOW_BACKEND.lock().unwrap()).clone())
        .clone()
        .unwrap())
    .clone();
}

// Source: upstream/packages/application/src/window.ts:458 (sha256:4b08e60197e0e3e5abd8ec276cad276041bbe2a444f6cd28e0c0fc1b179c2b66)
pub fn get_window_bounds(win: &ApplicationWindow, out: &WindowBounds) -> WindowBounds {
    return {
        let __flight_callback = (get_window_backend().get_bounds).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), (*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:466 (sha256:b0fdd8521a90318ffbc95fe30587be034f1d184601ac3cafab37e9c626dc8888)
pub fn get_window_display(win: &ApplicationWindow) -> f64 {
    return (-1.0_f64);
}

// Source: upstream/packages/application/src/window.ts:471 (sha256:af1903ff2051b39a359def9204556a34332260489e29b5e7bf2666c64bdcd585)
pub fn hide_window(win: &mut ApplicationWindow) -> () {
    if (!win.visible) {
        return;
    }
    win.visible = false;
    {
        let __flight_callback = (get_window_backend().hide).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:480 (sha256:449cebc4fb63fc9cc3223aeed83676489912c73ff9f5ce66a9ba290010a772b5)
pub fn lock_application_pointer(element: crate::OpaqueHostValue) -> crate::FlightTask<()> {
    return crate::FlightTask::ready(
        (),
        crate::FlightTaskOrigin {
            package: "@flighthq/application",
            source: "upstream/packages/application/src/window.ts",
            line: 481_u32,
            column: 64_u32,
            lexical_path: "lockApplicationPointer.ready:481:64:9b8874eb7ffe",
            fingerprint: "sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116",
        },
    );
}

// Source: upstream/packages/application/src/window.ts:488 (sha256:07aa885b72b80baaaf97f8cdfebdb6df04b3b387e9c8cf7a0e2826faed660f7f)
pub fn maximize_window(win: &mut ApplicationWindow) -> () {
    if win.maximized {
        return;
    }
    win.maximized = true;
    {
        let __flight_callback = (get_window_backend().maximize).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
    emit_signal((win.on_maximize).clone(), ());
}

// Source: upstream/packages/application/src/window.ts:496 (sha256:172c5236508db9ecadbfddef44245a8b6c75b80a067f76b4f6d936cda80d8440)
pub fn minimize_window(win: &mut ApplicationWindow) -> () {
    if win.minimized {
        return;
    }
    win.minimized = true;
    {
        let __flight_callback = (get_window_backend().minimize).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
    emit_signal((win.on_minimize).clone(), ());
}

// Source: upstream/packages/application/src/window.ts:506 (sha256:f07246d7ed70e6d25c1eec41fa04fc97d64878ed8f993099a4e2ee47452ad3e0)
#[derive(Clone, Default)]
struct OpenWindowRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for OpenWindowRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn open_window(win: &mut ApplicationWindow, options: Option<WindowOptions>) -> bool {
    let options = options.unwrap_or(WindowOptions {
        __flight_identity: std::sync::Arc::new(()),
        title: None,
        x: None,
        y: None,
        width: None,
        height: None,
        resizable: None,
        always_on_top: None,
        fullscreen: None,
        minimized: None,
        maximized: None,
        visible: None,
        min_width: None,
        min_height: None,
        max_width: None,
        max_height: None,
        center: None,
        frame: None,
        transparent: None,
    });
    if ((options.title).clone()).is_some() {
        win.title = ((options.title).clone()).unwrap();
    }
    if (options.x).is_some() {
        win.x = (options.x).unwrap();
    }
    if (options.y).is_some() {
        win.y = (options.y).unwrap();
    }
    if (options.width).is_some() {
        win.width = (options.width).unwrap();
    }
    if (options.height).is_some() {
        win.height = (options.height).unwrap();
    }
    if (options.resizable).is_some() {
        win.resizable = (options.resizable).unwrap();
    }
    if (options.always_on_top).is_some() {
        win.always_on_top = (options.always_on_top).unwrap();
    }
    if (options.fullscreen).is_some() {
        win.fullscreen = (options.fullscreen).unwrap();
    }
    if (options.minimized).is_some() {
        win.minimized = (options.minimized).unwrap();
    }
    if (options.maximized).is_some() {
        win.maximized = (options.maximized).unwrap();
    }
    if (options.visible).is_some() {
        win.visible = (options.visible).unwrap();
    }
    if (options.min_width).is_some() {
        win.min_width = (options.min_width).unwrap();
    }
    if (options.min_height).is_some() {
        win.min_height = (options.min_height).unwrap();
    }
    if (options.max_width).is_some() {
        win.max_width = (options.max_width).unwrap();
    }
    if (options.max_height).is_some() {
        win.max_height = (options.max_height).unwrap();
    }
    let result = {
        let __flight_callback = (get_window_backend().open).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), (options).clone());
        __flight_result
    };
    if (options.center) == Some(true) {
        center_window(win);
    }
    return result;
}

// Source: upstream/packages/application/src/window.ts:532 (sha256:23b3d384bae334e33c7abb59e06ede1b32d38578ffb1043115bc833ee75db194)
#[derive(Clone, Default)]
struct PrepareElementForInputRecord1 {
    __flight_identity: std::sync::Arc<()>,
    webkit_tap_highlight_color: String,
}
impl PartialEq for PrepareElementForInputRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn prepare_element_for_input(element: crate::OpaqueHostValue) -> () {
    crate::host_set("host.touchAction", "none");
    crate::host_set("host.userSelect", "none");
    crate::host_set("host.webkitUserSelect", "none");
    crate::host_set("host.webkitTapHighlightColor", "transparent");
    if false {
        crate::host_set("host.transform", "translateZ(0)");
    }
}

// Source: upstream/packages/application/src/window.ts:542 (sha256:a01a799371baf1484816b2b174eb69e019d120940240664550d013af5da11ffe)
pub fn request_application_fullscreen(element: crate::OpaqueHostValue) -> crate::FlightTask<()> {
    return crate::host_value::<crate::FlightTask<()>>("host.requestFullscreen");
}

// Source: upstream/packages/application/src/window.ts:547 (sha256:12155aaaac9bafee613742c944fcdfe0719dba3381b2c5ac5f53dfb09e23818d)
pub fn request_window_attention(win: &ApplicationWindow, attention: bool) -> () {
    {
        let __flight_callback = (get_window_backend().request_attention).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), attention);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:553 (sha256:0ef31d49cbeb5ddf44622bc7c9bf30f103420d91750139689e8c9b62129c21b0)
pub fn request_window_close(win: &ApplicationWindow) -> bool {
    emit_signal((win.on_close_request).clone(), ());
    return !((win
        .on_close_request
        .data
        .as_ref()
        .map(|value| value.inner.lock().unwrap().cancelled))
        == Some(true));
}

// Source: upstream/packages/application/src/window.ts:559 (sha256:79756d68353f2df8916801795173fca670515d76f79903507ee6bc15dbac8ec5)
pub fn restore_window(win: &mut ApplicationWindow) -> () {
    if (!win.minimized) && (!win.maximized) {
        return;
    }
    win.minimized = false;
    win.maximized = false;
    {
        let __flight_callback = (get_window_backend().restore).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
    emit_signal((win.on_restore).clone(), ());
}

// Source: upstream/packages/application/src/window.ts:568 (sha256:6c472d3e8319add9f130cc761b7d2dff8848552253a052a6642d19c81119f972)
pub fn set_window_always_on_top(win: &mut ApplicationWindow, always_on_top: bool) -> () {
    win.always_on_top = always_on_top;
    {
        let __flight_callback = (get_window_backend().set_always_on_top).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), always_on_top);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:574 (sha256:9f0742acf83a54ca621d73cf94284a142bbfd43b2a29c54942af9f4bca88cfe7)
pub fn set_window_backend(backend: Option<WindowBackend>) -> () {
    (*_WINDOW_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/application/src/window.ts:580 (sha256:86d123cccc7c7c426b332c532420e1af1451c0fc550da9bc71672ecfa4bb1d3e)
pub fn set_window_content_protection(win: &ApplicationWindow, enabled: bool) -> () {
    {
        let __flight_callback = (get_window_backend().set_content_protection).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), enabled);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:585 (sha256:62d2948d58dffd19c636cd5936b4512a69994accd838e4ec69b944739ad76922)
pub fn set_window_fullscreen(win: &mut ApplicationWindow, fullscreen: bool) -> () {
    if (win.fullscreen == fullscreen) {
        return;
    }
    win.fullscreen = fullscreen;
    {
        let __flight_callback = (get_window_backend().set_fullscreen).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), fullscreen);
        __flight_result
    };
    emit_signal((win.on_fullscreen_changed).clone(), ());
}

// Source: upstream/packages/application/src/window.ts:593 (sha256:b70953d0888c4b1dba593fa52b203767a47e98c15ed0692b84afaf39b5bf68d4)
pub fn set_window_has_shadow(win: &ApplicationWindow, has_shadow: bool) -> () {
    {
        let __flight_callback = (get_window_backend().set_has_shadow).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), has_shadow);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:598 (sha256:23f5af18d5a3e1aeb2604c43b07e402a85dca84518e3268c788fd86aa3488de5)
pub fn set_window_icon(win: &mut ApplicationWindow, icon: String) -> () {
    win.icon = (icon).clone();
    {
        let __flight_callback = (get_window_backend().set_icon).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), (icon).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:604 (sha256:31de4c9ddbae554214877619d6c81bb033618076a8ff57a863e1052d4701b542)
pub fn set_window_maximum_size(win: &mut ApplicationWindow, width: f64, height: f64) -> () {
    win.max_width = width;
    win.max_height = height;
    {
        let __flight_callback = (get_window_backend().set_maximum_size).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), width, height);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:611 (sha256:7271ef16d63c0cd4ba058e5ed240d91839ebe15ba9e1b292b19ba0a30a603de2)
pub fn set_window_menu_bar_visible(win: &ApplicationWindow, visible: bool) -> () {
    {
        let __flight_callback = (get_window_backend().set_menu_bar_visible).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), visible);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:616 (sha256:b211a2a8a7a159ab1dbbdf3de880673d748c3eb370d5c9fec2e6d2947e48a67c)
pub fn set_window_minimum_size(win: &mut ApplicationWindow, width: f64, height: f64) -> () {
    win.min_width = width;
    win.min_height = height;
    {
        let __flight_callback = (get_window_backend().set_minimum_size).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), width, height);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:623 (sha256:e7dc32e9e56651df02c6319e1dbc1b1cdfe893af4d70b8654ad1faf11ce82470)
pub fn set_window_opacity(win: &mut ApplicationWindow, opacity: f64) -> () {
    win.opacity = opacity;
    {
        let __flight_callback = (get_window_backend().set_opacity).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), opacity);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:629 (sha256:c0e30945952e7929263df3f58d2f68d01fce5a516a750719f2e906a27d86c226)
pub fn set_window_parent(win: &ApplicationWindow, parent: Option<ApplicationWindow>) -> () {
    {
        let __flight_callback = (get_window_backend().set_parent).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), (parent).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:634 (sha256:5ef1ab2b667f70b6453b0b920fd0ec7bfc960a94a3faa6f72605fec33a7c1273)
pub fn set_window_position(win: &mut ApplicationWindow, x: f64, y: f64) -> () {
    win.x = x;
    win.y = y;
    {
        let __flight_callback = (get_window_backend().set_position).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), x, y);
        __flight_result
    };
    emit_signal((win.on_move).clone(), ());
}

// Source: upstream/packages/application/src/window.ts:642 (sha256:04e9e62360c4bb59fb8c6828e09c8163108d8801f036480996d1b1a9bcff44db)
pub fn set_window_progress(win: &ApplicationWindow, progress: f64) -> () {
    {
        let __flight_callback = (get_window_backend().set_progress).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), progress);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:647 (sha256:2158d098bdb6c10a348bc62ac33c2d9c77cee936bd590598001f136b63024468)
pub fn set_window_resizable(win: &mut ApplicationWindow, resizable: bool) -> () {
    win.resizable = resizable;
    {
        let __flight_callback = (get_window_backend().set_resizable).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), resizable);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:653 (sha256:3ec39b5d4a6db28effd9e362f8e511f54d81b720aaa2598221f602a4a35010e2)
pub fn set_window_size(win: &mut ApplicationWindow, width: f64, height: f64) -> () {
    win.width = width;
    win.height = height;
    {
        let __flight_callback = (get_window_backend().set_size).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), width, height);
        __flight_result
    };
    emit_signal((win.on_resize).clone(), ());
}

// Source: upstream/packages/application/src/window.ts:661 (sha256:ecbc7c4b0c4124192ad1e827a5c73616d37a821f2fc05c581379464102ca3809)
pub fn set_window_skip_taskbar(win: &mut ApplicationWindow, skip: bool) -> () {
    win.skip_taskbar = skip;
    {
        let __flight_callback = (get_window_backend().set_skip_taskbar).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), skip);
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:667 (sha256:06e144755144326356817ee3e6cd1b1bd14d8e2e634d968bf88402f86012b6a3)
pub fn set_window_title(win: &mut ApplicationWindow, title: String) -> () {
    win.title = (title).clone();
    {
        let __flight_callback = (get_window_backend().set_title).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone(), (title).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:673 (sha256:426343c80d37bbb23bab31cb4459b6f7e15bc1301b98a5c781f540ca89a5fc8c)
pub fn show_window(win: &mut ApplicationWindow) -> () {
    if win.visible {
        return;
    }
    win.visible = true;
    {
        let __flight_callback = (get_window_backend().show).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*win).clone());
        __flight_result
    };
}

// Source: upstream/packages/application/src/window.ts:682 (sha256:b48d7fdf6235a5e7014397be4dc61433ef6bf2a07085e63f8ed71284aa9d0ad5)
static _APPLICATION_WINDOW_OBSERVERS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            ApplicationWindow,
            Vec<(
                crate::FlightSymbol,
                std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
            )>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/application/src/window.ts:684 (sha256:7f675fc35dcb2bdca66d04a67f9a9380dbd5491de37d22e545cf3b348a093e27)
static _WINDOW_BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<WindowBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/application/src/window.ts:686 (sha256:e0add53906d9cb02cebe5f7fc3392179d99072b9d27c83ef5b150f09ec68b74a)
fn get_application_window_observers(
    win: &ApplicationWindow,
) -> Vec<(
    crate::FlightSymbol,
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
)> {
    let mut observers = (*_APPLICATION_WINDOW_OBSERVERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*win).clone())
        .map(|(_, value)| value.clone());
    if (observers).is_none() {
        observers = Some(Vec::new());
        {
            let __flight_key = (*win).clone();
            let __flight_value = (observers).clone().unwrap();
            if let Some((_, value)) = (*_APPLICATION_WINDOW_OBSERVERS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_APPLICATION_WINDOW_OBSERVERS.lock().unwrap())
                    .push((__flight_key, __flight_value));
            }
        };
    }
    return ((observers).clone().unwrap()).clone();
}
