// @generated from upstream/packages/application/src/application.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{connect_signal, create_signal, disconnect_signal, emit_signal};
use flighthq_types::{Application, ApplicationLoopOptions, ApplicationWindow, LoopBackend};

// Source: upstream/packages/application/src/application.ts:4 (sha256:e5ae275a20d89e9bb0fc349ab81b206c324f66adf01de3a2fc8f432ad0e14880)
const DEFAULT_BACKGROUND_FRAME_RATE: f64 = 0.0_f64;

// Source: upstream/packages/application/src/application.ts:5 (sha256:29852d9f0d9d7f94e620e0fcbe810aa9dd8b2ccaf33d73295012f096f9be1ae9)
const DEFAULT_FIXED_TIMESTEP: f64 = 0.0_f64;

// Source: upstream/packages/application/src/application.ts:6 (sha256:a5731dd8c44a2d2ef70e1571325f1e47d1956b64321a484113351b619b9bab55)
const DEFAULT_MAX_DELTA_TIME: f64 = 250.0_f64;

// Source: upstream/packages/application/src/application.ts:7 (sha256:dc3064629419b3d091c01e50e6dcf6fadcff48bb76727c4aaf213b249bbdeac9)
const DEFAULT_MAX_UPDATES_PER_FRAME: f64 = 5.0_f64;

// Source: upstream/packages/application/src/application.ts:9 (sha256:a09b9c2538535866160d6db9bb124c5dd7566f8583b82ad0bd683120a138ea69)
static K_EXIT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/application.ts:10 (sha256:212828b2aa0039f24f48bb8b6510fb16dfebc8303c78627e5314205cd7a0aa1c)
static K_LOOP: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/application.ts:11 (sha256:04a60560c07c3fdb56e1bb5665bb823f56dfca1f7a82c220bfdff49dc8598d76)
static K_PAUSED: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/application/src/application.ts:15 (sha256:7a3b9c2f32279f8cd36d4928eb780120700b07b1502ad3c4c596758051f2947a)
pub fn attach_application_exit(app: Application) -> () {
    let mut observers = get_application_observers(&app);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_EXIT)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut handler: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let app = app.clone();
            move || -> () { emit_signal((app.on_exit).clone(), ()) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    crate::host_value::<()>("host.addEventListener");
    {
        let __flight_key = *K_EXIT;
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

// Source: upstream/packages/application/src/application.ts:26 (sha256:c34794d429d13355e71ae8b380f3ab33e9635bf26f4ea1e4ad45609da3bd39e2)
pub fn attach_application_lifecycle(mut app: Application, mut win: ApplicationWindow) -> () {
    let mut k_lifecycle = (*_LIFECYCLE_KEYS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(win).clone())
        .map(|(_, value)| value.clone());
    if (k_lifecycle).is_none() {
        k_lifecycle = Some(crate::FlightSymbol::new());
        {
            let __flight_key = (win).clone();
            let __flight_value = (k_lifecycle).clone().unwrap();
            if let Some((_, value)) = (*_LIFECYCLE_KEYS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_LIFECYCLE_KEYS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let mut observers = get_application_observers(&app);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &(k_lifecycle).clone().unwrap())
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let mut on_deactivate: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut app = app.clone();
        move || -> () {
            pause_application_loop(&mut app);
            if ((app.on_deactivate).clone()).is_some() {
                emit_signal(((app.on_deactivate).clone()).unwrap(), ());
            }
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    let mut on_activate: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut app = app.clone();
            move || -> () {
                resume_application_loop(&mut app);
                if ((app.on_activate).clone()).is_some() {
                    emit_signal(((app.on_activate).clone()).unwrap(), ());
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    connect_signal(&mut win.on_deactivate, (on_deactivate).clone(), None);
    connect_signal(&mut win.on_activate, (on_activate).clone(), None);
    {
        let __flight_key = (k_lifecycle).clone().unwrap();
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let on_activate = on_activate.clone();
            let on_deactivate = on_deactivate.clone();
            let mut win = win.clone();
            move || -> () {
                disconnect_signal(&mut win.on_deactivate, (on_deactivate).clone());
                disconnect_signal(&mut win.on_activate, (on_activate).clone());
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

// Source: upstream/packages/application/src/application.ts:53 (sha256:3fd82b69492b3bfdb096bcd08d9755df383e1a5080247b9135d7d42bb54f4733)
pub fn create_application() -> Application {
    return Application {
        __flight_identity: std::sync::Arc::new(()),
        delta_time: 0.0_f64,
        elapsed_time: 0.0_f64,
        frame_count: 0.0_f64,
        interpolation_alpha: 1.0_f64,
        is_running: false,
        on_activate: None,
        on_deactivate: None,
        on_error: None,
        on_exit: create_signal(),
        on_fixed_update: None,
        on_render: create_signal(),
        on_update: create_signal(),
        windows: vec![],
    };
}

// Source: upstream/packages/application/src/application.ts:71 (sha256:2daefb103d1eb90b4c2ff86f0560a035d330269844df7ba0801ed9a4f75989f0)
pub fn create_web_loop_backend() -> LoopBackend {
    return LoopBackend {
        __flight_identity: std::sync::Arc::new(()),
        request_frame: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |callback: std::sync::Arc<
                std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>,
            >|
                  -> crate::OpaqueHostValue {
                return crate::host_value::<crate::OpaqueHostValue>("host.call");
            },
        )
            as Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
                    ) -> crate::OpaqueHostValue
                    + Send
                    + 'static,
            >)),
        cancel_frame: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |handle: crate::OpaqueHostValue| -> () {
                crate::host_value::<()>("host.call");
            },
        )
            as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>)),
        now: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> f64 {
            return crate::host_value::<f64>("host.now");
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)),
    };
}

// Source: upstream/packages/application/src/application.ts:85 (sha256:f301ba0725b21e3742b90a3642073a9142804308e678b963de139bb9ceb4a653)
pub fn detach_application_exit(app: &Application) -> () {
    let mut observers = get_application_observers(app);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_EXIT)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_EXIT;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/application/src/application.ts:91 (sha256:528eaa6ce8e42ce4f802fa39bd71a3a7eb53a8ffeedec023aca2da9708b851a2)
pub fn dispose_application(app: &mut Application) -> () {
    let mut observers = get_application_observers(app);
    for cleanup in (observers
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        ((cleanup).clone()).lock().unwrap()();
    }
    observers.clear();
    app.is_running = false;
}

// Source: upstream/packages/application/src/application.ts:101 (sha256:e16d70358b0d6260ff9436e3ca55a72b92b7b64f7d30aef2d002a39af77d8f67)
pub fn enable_application_lifecycle_signals(app: &mut Application) -> () {
    if ((app.on_activate).clone()).is_none() {
        app.on_activate = Some(create_signal());
    }
    if ((app.on_deactivate).clone()).is_none() {
        app.on_deactivate = Some(create_signal());
    }
    if ((app.on_error).clone()).is_none() {
        app.on_error = Some(create_signal());
    }
    if ((app.on_fixed_update).clone()).is_none() {
        app.on_fixed_update = Some(create_signal());
    }
}

// Source: upstream/packages/application/src/application.ts:109 (sha256:07c4e88d0a41cf10f3f27afa06a4eb484c31bdbe784a9e50bbeb4f7698dace0b)
pub fn for_each_application_window(
    app: &Application,
    fn_: &mut impl FnMut(ApplicationWindow) -> (),
) -> () {
    for win in ((app.windows).clone()).iter().cloned() {
        fn_((win).clone());
    }
}

// Source: upstream/packages/application/src/application.ts:115 (sha256:2085ed2efb6f5b106b8cfcbdbf1b847948ae6f40caf1bea71d44aa262fba3b42)
pub fn get_application_frame_rate(app: &Application) -> f64 {
    let state = (*_APPLICATION_LOOP_STATE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*app).clone())
        .map(|(_, value)| value.clone());
    if ((state).is_none() || ((state.as_ref().unwrap().fps_buffer.len() as f64) < 2.0_f64)) {
        return 0.0_f64;
    }
    let len = (state.as_ref().unwrap().fps_buffer.len() as f64);
    let mut total = 0.0_f64;
    let mut count = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < len) {
            if (state.as_ref().unwrap().fps_buffer[i as usize].clone() > 0.0_f64) {
                total += state.as_ref().unwrap().fps_buffer[i as usize].clone();
                {
                    count += 1.0;
                    count
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if (count == 0.0_f64) {
        return 0.0_f64;
    }
    let avg_delta = (total / count);
    return if (avg_delta > 0.0_f64) {
        (1000.0_f64 / avg_delta)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/application/src/application.ts:136 (sha256:356d0857fd495466cbfe7c022c8c8d8ef31266e26c2ca8736ecf9c30538c3357)
pub fn get_application_main_window(app: &Application) -> Option<ApplicationWindow> {
    return Some(
        ((*_MAIN_WINDOWS.lock().unwrap())
            .iter()
            .find(|(key, _)| key == &(*app).clone())
            .map(|(_, value)| value.clone()))
        .unwrap_or(app.windows[0.0_f64 as usize].clone()),
    );
}

// Source: upstream/packages/application/src/application.ts:142 (sha256:cb9365e164c71b654e42bda01ad6c65248b17d6b5f91d8ca53231d582f72446d)
pub fn get_application_windows(app: &Application) -> Vec<ApplicationWindow> {
    return ((app.windows).clone()).clone();
}

// Source: upstream/packages/application/src/application.ts:146 (sha256:9bcd91f51754c154d9c85bd4d77f833ab31002826474df1be4292ebc936b4514)
pub fn get_loop_backend() -> LoopBackend {
    if ((*_LOOP_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_LOOP_BACKEND.lock().unwrap()) = Some(create_web_loop_backend());
    }
    return ((*_LOOP_BACKEND.lock().unwrap()).clone()).clone().unwrap();
}

// Source: upstream/packages/application/src/application.ts:151 (sha256:9bbe5c835e6327c56cf71f349067c93d88215c1d7a4acd3dac81cc9256927f56)
pub fn is_application_running(app: &Application) -> bool {
    return app.is_running;
}

// Source: upstream/packages/application/src/application.ts:155 (sha256:468a64b9c52a7bf5203ce9a34ec557957d49563390f9d640a7fbbeaa4496daf0)
pub fn pause_application_loop(app: &mut Application) -> () {
    let mut observers = get_application_observers(app);
    if ((!app.is_running) || observers.iter().any(|(key, _)| key == &*K_PAUSED)) {
        return;
    }
    app.is_running = false;
    {
        let __flight_key = *K_PAUSED;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        ));
        if let Some((_, value)) = observers.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            observers.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/application.ts:164 (sha256:1f7d153197036616d01242d5b2f903f6d03c04ffd191379a51d93f4b082c664a)
pub fn register_application_window(app: &mut Application, win: &ApplicationWindow) -> () {
    if {
        let __flight_value = (*win).clone();
        ((app.windows).clone())
            .iter()
            .any(|item| item == &__flight_value)
    } {
        return;
    }
    app.windows.push(((*win).clone()).clone());
}

// Source: upstream/packages/application/src/application.ts:169 (sha256:f984e14a003c3935107004babfd5392ebafc8c7a1a5266ba774954f9a744068e)
pub fn resume_application_loop(app: &mut Application) -> () {
    let mut observers = get_application_observers(app);
    if (!observers.iter().any(|(key, _)| key == &*K_PAUSED)) {
        return;
    }
    {
        let __flight_key = *K_PAUSED;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
    let mut loop_state = (*_APPLICATION_LOOP_STATE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*app).clone())
        .map(|(_, value)| value.clone());
    if (loop_state).is_some() {
        loop_state.as_mut().unwrap().last_time = (-1.0_f64);
        loop_state.as_mut().unwrap().fixed_accumulator = 0.0_f64;
        loop_state.as_mut().unwrap().frame_rate_accumulated = 0.0_f64;
    }
    app.is_running = true;
}

// Source: upstream/packages/application/src/application.ts:185 (sha256:e0bfc3dc78241a0ea380c1733170efa8cabdc1e1dc51dbb9b729d1d8424019ee)
pub fn set_application_main_window(app: &mut Application, win: &ApplicationWindow) -> () {
    register_application_window(app, win);
    {
        let __flight_key = (*app).clone();
        let __flight_value = (*win).clone();
        if let Some((_, value)) = (*_MAIN_WINDOWS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_MAIN_WINDOWS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/application.ts:191 (sha256:8b909762712641019ca399eea4014d78e0dbb45ecd396885b291d4721b9d62d0)
pub fn set_loop_backend(backend: Option<LoopBackend>) -> () {
    (*_LOOP_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/application/src/application.ts:195 (sha256:cc31ce6e7b7ffb1349bb39c965cc5c7f5ed12afbb68a1d65479c31ca34408474)
#[derive(Clone)]
struct StartApplicationLoopRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for StartApplicationLoopRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn start_application_loop(mut app: Application, options: Option<ApplicationLoopOptions>) -> () {
    let options = options.unwrap_or(ApplicationLoopOptions {
        __flight_identity: std::sync::Arc::new(()),
        max_delta_time: None,
        target_frame_rate: None,
        background_frame_rate: None,
        fixed_time_step: None,
        max_updates_per_frame: None,
    });
    let observers: std::sync::Arc<
        std::sync::Mutex<
            Vec<(
                crate::FlightSymbol,
                std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
            )>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(get_application_observers(&app)));
    {
        let __flight_callback = (*observers.lock().unwrap())
            .iter()
            .find(|(key, _)| key == &*K_LOOP)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_PAUSED;
        if let Some(__flight_index) = (*observers.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*observers.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
    let backend = get_loop_backend();
    let max_delta_time = (options.max_delta_time).unwrap_or(DEFAULT_MAX_DELTA_TIME);
    let target_frame_rate = (options.target_frame_rate).unwrap_or(0.0_f64);
    let background_frame_rate =
        (options.background_frame_rate).unwrap_or(DEFAULT_BACKGROUND_FRAME_RATE);
    let fixed_time_step = (options.fixed_time_step).unwrap_or(DEFAULT_FIXED_TIMESTEP);
    let max_updates_per_frame =
        (options.max_updates_per_frame).unwrap_or(DEFAULT_MAX_UPDATES_PER_FRAME);
    let frame_interval = if (target_frame_rate > 0.0_f64) {
        (1000.0_f64 / target_frame_rate)
    } else {
        0.0_f64
    };
    let bg_interval = if (background_frame_rate > 0.0_f64) {
        (1000.0_f64 / background_frame_rate)
    } else {
        0.0_f64
    };
    let loop_state: std::sync::Arc<std::sync::Mutex<LoopState>> =
        std::sync::Arc::new(std::sync::Mutex::new(LoopState {
            __flight_identity: std::sync::Arc::new(()),
            fixed_accumulator: 0.0_f64,
            fps_buffer: vec![],
            fps_head: 0.0_f64,
            frame_handle: crate::OpaqueHostValue::Null,
            frame_rate_accumulated: 0.0_f64,
            last_time: (-1.0_f64),
            max_delta_time: max_delta_time,
        }));
    {
        let __flight_key = (app).clone();
        let __flight_value = (*loop_state.lock().unwrap()).clone();
        if let Some((_, value)) = (*_APPLICATION_LOOP_STATE.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_APPLICATION_LOOP_STATE.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    app.is_running = true;
    let __flight_recursive_tick: std::sync::Arc<
        std::sync::Mutex<
            Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut tick: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let __flight_recursive_tick = __flight_recursive_tick.clone();
            let mut app = app.clone();
            let backend = backend.clone();
            let mut loop_state = loop_state.clone();
            let mut observers = observers.clone();
            move |time: f64| -> () {
                if (!app.is_running) {
                    (*loop_state.lock().unwrap()).frame_handle =
                        ((backend.request_frame).clone()).lock().unwrap()(
                            (__flight_recursive_tick
                                .lock()
                                .unwrap()
                                .as_ref()
                                .unwrap()
                                .clone())
                            .clone(),
                        );
                    {
                        let __flight_key = *K_LOOP;
                        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                            let backend = backend.clone();
                            let mut loop_state = loop_state.clone();
                            move || -> () {
                                ((backend.cancel_frame).clone()).lock().unwrap()(
                                    ((*loop_state.lock().unwrap()).frame_handle).clone(),
                                )
                            }
                        })
                            as Box<dyn FnMut() -> () + Send + 'static>));
                        if let Some((_, value)) = (*observers.lock().unwrap())
                            .iter_mut()
                            .find(|(key, _)| key == &__flight_key)
                        {
                            *value = __flight_value;
                        } else {
                            (*observers.lock().unwrap()).push((__flight_key, __flight_value));
                        }
                    };
                    return;
                }
                let is_first_tick = ((*loop_state.lock().unwrap()).last_time < 0.0_f64);
                let raw = if is_first_tick {
                    0.0_f64
                } else {
                    (time - (*loop_state.lock().unwrap()).last_time)
                };
                (*loop_state.lock().unwrap()).last_time = time;
                let active_interval = if ((app.is_running && (bg_interval > 0.0_f64))
                    && (!_is_application_visible()))
                {
                    bg_interval
                } else {
                    frame_interval
                };
                if (!is_first_tick) {
                    (*loop_state.lock().unwrap()).frame_rate_accumulated += raw;
                    if ((active_interval > 0.0_f64)
                        && ((*loop_state.lock().unwrap()).frame_rate_accumulated < active_interval))
                    {
                        (*loop_state.lock().unwrap()).frame_handle =
                            ((backend.request_frame).clone()).lock().unwrap()(
                                (__flight_recursive_tick
                                    .lock()
                                    .unwrap()
                                    .as_ref()
                                    .unwrap()
                                    .clone())
                                .clone(),
                            );
                        {
                            let __flight_key = *K_LOOP;
                            let __flight_value =
                                std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                                    let backend = backend.clone();
                                    let mut loop_state = loop_state.clone();
                                    move || -> () {
                                        ((backend.cancel_frame).clone()).lock().unwrap()(
                                            ((*loop_state.lock().unwrap()).frame_handle).clone(),
                                        )
                                    }
                                })
                                    as Box<dyn FnMut() -> () + Send + 'static>));
                            if let Some((_, value)) = (*observers.lock().unwrap())
                                .iter_mut()
                                .find(|(key, _)| key == &__flight_key)
                            {
                                *value = __flight_value;
                            } else {
                                (*observers.lock().unwrap()).push((__flight_key, __flight_value));
                            }
                        };
                        return;
                    }
                }
                let delta = if ((active_interval > 0.0_f64) && (!is_first_tick)) {
                    (*loop_state.lock().unwrap()).frame_rate_accumulated
                } else {
                    raw
                };
                (*loop_state.lock().unwrap()).frame_rate_accumulated = 0.0_f64;
                let clamped = (delta).min(max_delta_time);
                app.delta_time = clamped;
                app.elapsed_time += (clamped / 1000.0_f64);
                app.frame_count += 1.0_f64;
                record_fps_sample(&mut (*loop_state.lock().unwrap()), clamped);
                if ((fixed_time_step > 0.0_f64) && ((app.on_fixed_update).clone()).is_some()) {
                    (*loop_state.lock().unwrap()).fixed_accumulator += clamped;
                    let mut iters = 0.0_f64;
                    while (((*loop_state.lock().unwrap()).fixed_accumulator >= fixed_time_step)
                        && (iters < max_updates_per_frame))
                    {
                        (*loop_state.lock().unwrap()).fixed_accumulator -= fixed_time_step;
                        {
                            iters += 1.0;
                            iters
                        };
                        if ((app.on_error).clone()).is_some() {
                            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                emit_signal(
                                    ((app.on_fixed_update).clone()).unwrap(),
                                    (fixed_time_step,),
                                );
                            })) {
                                Ok(_) => {}
                                Err(_) => {
                                    let err = crate::OpaqueHostValue::Object;
                                    {
                                        emit_signal(((app.on_error).clone()).unwrap(), (err,));
                                    }
                                }
                            }
                        } else {
                            emit_signal(
                                ((app.on_fixed_update).clone()).unwrap(),
                                (fixed_time_step,),
                            );
                        }
                    }
                    if (iters >= max_updates_per_frame) {
                        (*loop_state.lock().unwrap()).fixed_accumulator = 0.0_f64;
                    }
                    app.interpolation_alpha = if (fixed_time_step > 0.0_f64) {
                        ((*loop_state.lock().unwrap()).fixed_accumulator / fixed_time_step)
                    } else {
                        1.0_f64
                    };
                } else {
                    app.interpolation_alpha = 1.0_f64;
                }
                if ((app.on_error).clone()).is_some() {
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        emit_signal((app.on_update).clone(), (clamped,));
                    })) {
                        Ok(_) => {}
                        Err(_) => {
                            let err = crate::OpaqueHostValue::Object;
                            {
                                emit_signal(((app.on_error).clone()).unwrap(), (err,));
                            }
                        }
                    }
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        emit_signal((app.on_render).clone(), ());
                    })) {
                        Ok(_) => {}
                        Err(_) => {
                            let err = crate::OpaqueHostValue::Object;
                            {
                                emit_signal(((app.on_error).clone()).unwrap(), (err,));
                            }
                        }
                    }
                } else {
                    emit_signal((app.on_update).clone(), (clamped,));
                    emit_signal((app.on_render).clone(), ());
                }
                (*loop_state.lock().unwrap()).frame_handle =
                    ((backend.request_frame).clone()).lock().unwrap()(
                        (__flight_recursive_tick
                            .lock()
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .clone())
                        .clone(),
                    );
                {
                    let __flight_key = *K_LOOP;
                    let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                        let backend = backend.clone();
                        let mut loop_state = loop_state.clone();
                        move || -> () {
                            ((backend.cancel_frame).clone()).lock().unwrap()(
                                ((*loop_state.lock().unwrap()).frame_handle).clone(),
                            )
                        }
                    })
                        as Box<dyn FnMut() -> () + Send + 'static>));
                    if let Some((_, value)) = (*observers.lock().unwrap())
                        .iter_mut()
                        .find(|(key, _)| key == &__flight_key)
                    {
                        *value = __flight_value;
                    } else {
                        (*observers.lock().unwrap()).push((__flight_key, __flight_value));
                    }
                };
            }
        })
            as Box<dyn FnMut(f64) -> () + Send + 'static>));
    *__flight_recursive_tick.lock().unwrap() = Some(tick.clone());
    (*loop_state.lock().unwrap()).frame_handle =
        ((backend.request_frame).clone()).lock().unwrap()((tick).clone());
    {
        let __flight_key = *K_LOOP;
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let backend = backend.clone();
            let mut loop_state = loop_state.clone();
            move || -> () {
                ((backend.cancel_frame).clone()).lock().unwrap()(
                    ((*loop_state.lock().unwrap()).frame_handle).clone(),
                )
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = (*observers.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*observers.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/application/src/application.ts:312 (sha256:0e0ceb9d9615235426ca0a113015f36e0dac67e70b1ca1ea3d3a95ca397755f4)
pub fn step_application_loop(app: &mut Application, delta_time: f64) -> () {
    let mut loop_state = (*_APPLICATION_LOOP_STATE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*app).clone())
        .map(|(_, value)| value.clone());
    let max_delta =
        (loop_state.as_ref().map(|value| value.max_delta_time)).unwrap_or(DEFAULT_MAX_DELTA_TIME);
    let clamped = (delta_time).min(max_delta);
    app.delta_time = clamped;
    app.elapsed_time += (clamped / 1000.0_f64);
    app.frame_count += 1.0_f64;
    app.interpolation_alpha = 1.0_f64;
    if (loop_state).is_some() {
        record_fps_sample(&mut loop_state.as_mut().unwrap(), clamped);
    }
    if ((app.on_error).clone()).is_some() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            emit_signal((app.on_update).clone(), (clamped,));
        })) {
            Ok(_) => {}
            Err(_) => {
                let err = crate::OpaqueHostValue::Object;
                {
                    emit_signal(((app.on_error).clone()).unwrap(), (err,));
                }
            }
        }
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            emit_signal((app.on_render).clone(), ());
        })) {
            Ok(_) => {}
            Err(_) => {
                let err = crate::OpaqueHostValue::Object;
                {
                    emit_signal(((app.on_error).clone()).unwrap(), (err,));
                }
            }
        }
    } else {
        emit_signal((app.on_update).clone(), (clamped,));
        emit_signal((app.on_render).clone(), ());
    }
}

// Source: upstream/packages/application/src/application.ts:338 (sha256:ea8cb082f1bc86cf8e3d6e10ac636676f4b889d540481d6ecc6b759c7e694c28)
pub fn stop_application_loop(app: &mut Application) -> () {
    let mut observers = get_application_observers(app);
    {
        let __flight_callback = observers
            .iter()
            .find(|(key, _)| key == &*K_LOOP)
            .map(|(_, value)| value.clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    {
        let __flight_key = *K_LOOP;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_key = *K_PAUSED;
        if let Some(__flight_index) = observers.iter().position(|(key, _)| key == &__flight_key) {
            observers.remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_key = (*app).clone();
        if let Some(__flight_index) = (*_APPLICATION_LOOP_STATE.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_APPLICATION_LOOP_STATE.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
    app.is_running = false;
}

// Source: upstream/packages/application/src/application.ts:348 (sha256:6fb059c952a642bb1c80df2e5c104bada68d042669ef3f6859d9f409107084a3)
pub fn unregister_application_window(app: &mut Application, win: &ApplicationWindow) -> () {
    let idx = {
        let __flight_value = (*win).clone();
        ((app.windows).clone())
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (idx != (-1.0_f64)) {
        app.windows
            .splice((idx) as usize..((idx) + (1.0_f64)) as usize, vec![]);
    }
    if ((*_MAIN_WINDOWS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*app).clone())
        .map(|(_, value)| value.clone()))
        == Some((*win).clone())
    {
        {
            let __flight_key = (*app).clone();
            if let Some(__flight_index) = (*_MAIN_WINDOWS.lock().unwrap())
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                (*_MAIN_WINDOWS.lock().unwrap()).remove(__flight_index);
                true
            } else {
                false
            }
        };
    }
}

// Source: upstream/packages/application/src/application.ts:356 (sha256:cf2f4507eca19c34acd189429659f38893e7fb7d2bcc936229ddba23d2fa4bbc)
static _APPLICATION_OBSERVERS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            Application,
            Vec<(
                crate::FlightSymbol,
                std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
            )>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/application/src/application.ts:359 (sha256:f459fe375e2ae44908785bae27015cd20f32bbc7ce599c15f61508adceb769d1)
static _APPLICATION_LOOP_STATE: std::sync::LazyLock<
    std::sync::Mutex<Vec<(Application, LoopState)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/application/src/application.ts:362 (sha256:28cc3d8a592daf6aa9edc02659ee9c587c2ffe4712673ea98c42b0b3ac0761e7)
static _LIFECYCLE_KEYS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(ApplicationWindow, crate::FlightSymbol)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/application/src/application.ts:365 (sha256:5fbc132166fceba44298dee88c92640ece71d565d079ff721760e4a067bddbd3)
static _MAIN_WINDOWS: std::sync::LazyLock<std::sync::Mutex<Vec<(Application, ApplicationWindow)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/application/src/application.ts:367 (sha256:d1d3aff309b4d9405bbe102c4537929af9327a90527455ed59bfb79b9e144f22)
const ROLLING_FPS_WINDOW: f64 = 60.0_f64;

// Source: upstream/packages/application/src/application.ts:369 (sha256:dd1556279acc82cf7cc2b97b11df1bfc4023c027af22cc948801d28ecc274388)
static _LOOP_BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<LoopBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/application/src/application.ts:371 (sha256:37211d505503f8ecae55a905713eed072dcae393446b9ebecee2c5eca7ba70ab)
#[derive(Clone)]
struct LoopState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fixed_accumulator: f64,
    pub fps_buffer: Vec<f64>,
    pub fps_head: f64,
    pub frame_handle: crate::OpaqueHostValue,
    pub frame_rate_accumulated: f64,
    pub last_time: f64,
    pub max_delta_time: f64,
}
impl PartialEq for LoopState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/application/src/application.ts:381 (sha256:09db27b105508d1a1d47dbc1dd27f3cf671cb4c5cd52760bd88c6fa968f9f548)
fn get_application_observers(
    app: &Application,
) -> Vec<(
    crate::FlightSymbol,
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
)> {
    let mut observers = (*_APPLICATION_OBSERVERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*app).clone())
        .map(|(_, value)| value.clone());
    if (observers).is_none() {
        observers = Some(Vec::new());
        {
            let __flight_key = (*app).clone();
            let __flight_value = (observers).clone().unwrap();
            if let Some((_, value)) = (*_APPLICATION_OBSERVERS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_APPLICATION_OBSERVERS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    return (observers).clone().unwrap();
}

// Source: upstream/packages/application/src/application.ts:392 (sha256:fd56e55c4c3005d431b7d73851e47d30e31fc9e129b9954d732a92262c1c9208)
fn _is_application_visible() -> bool {
    return (("undefined" == "undefined") || !(crate::host_value::<bool>("host.hidden")));
}

// Source: upstream/packages/application/src/application.ts:396 (sha256:ba92246f3bbc130f965686a1304e87c8eb97acc4914f0b9b67ffbcfa57322d09)
fn record_fps_sample(state: &mut LoopState, delta: f64) -> () {
    if ((state.fps_buffer.len() as f64) < ROLLING_FPS_WINDOW) {
        state.fps_buffer.push(delta);
    } else {
        {
            let __flight_index = (state.fps_head) as usize;
            let __flight_value = delta;
            if __flight_index == state.fps_buffer.len() {
                state.fps_buffer.push(__flight_value);
            } else {
                state.fps_buffer[__flight_index] = __flight_value;
            }
        };
        state.fps_head = ((state.fps_head + 1.0_f64) % ROLLING_FPS_WINDOW);
    }
}
