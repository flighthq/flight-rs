// @generated from upstream/packages/lifecycle/src/lifecycle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    AppLaunchKind, AppLifecycle, AppLifecycleState, AppMemoryPressure, LifecycleBackend,
};

// Source: upstream/packages/lifecycle/src/lifecycle.ts:20 (sha256:89e79b96ca581bb812b5c08e91843553be3b8ab2f6445aac25eba8f6f2d468b9)
#[derive(Clone)]
struct AttachAppLifecycleRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for AttachAppLifecycleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn attach_app_lifecycle(app: AppLifecycle) -> () {
    detach_app_lifecycle(&app);
    let backend = get_lifecycle_backend();
    let previous: std::sync::Arc<std::sync::Mutex<AppLifecycleState>> =
        std::sync::Arc::new(std::sync::Mutex::new(((backend.get_state).clone())
            .lock()
            .unwrap()()));
    let unsubscribe_state = ((backend.subscribe).clone()).lock().unwrap()(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let app = app.clone();
            let backend = backend.clone();
            let mut previous = previous.clone();
            move || -> () {
                let state = ((backend.get_state).clone()).lock().unwrap()();
                emit_signal((app.on_state_change).clone(), ((state).clone(),));
                if ((state == "active") && ((*previous.lock().unwrap()).clone() != "active")) {
                    emit_signal((app.on_resume).clone(), ());
                    let saved = (*_SAVED_STATE.lock().unwrap())
                        .iter()
                        .find(|(key, _)| key == &(app).clone())
                        .map(|(_, value)| value.clone());
                    if (saved).is_some() {
                        emit_signal(
                            (app.on_restore_state).clone(),
                            ((saved.as_ref().unwrap()).clone(),),
                        );
                    }
                } else {
                    if ((state != "active") && ((*previous.lock().unwrap()).clone() == "active")) {
                        emit_signal((app.on_pause).clone(), ());
                        let state_bag: Vec<(String, crate::OpaqueHostValue)> = {
                            let mut __flight_record = Vec::new();
                            __flight_record
                        };
                        emit_signal((app.on_save_state).clone(), ((state_bag).clone(),));
                        {
                            let __flight_key = (app).clone();
                            let __flight_value = (state_bag).clone();
                            if let Some((_, value)) = (*_SAVED_STATE.lock().unwrap())
                                .iter_mut()
                                .find(|(key, _)| key == &__flight_key)
                            {
                                *value = __flight_value;
                            } else {
                                (*_SAVED_STATE.lock().unwrap())
                                    .push((__flight_key, __flight_value));
                            }
                        };
                    }
                }
                (*previous.lock().unwrap()) = (state).clone();
            }
        }) as Box<dyn FnMut() -> () + Send + 'static>),
    ));
    let unsubscribe_memory: std::sync::Arc<
        std::sync::Mutex<
            Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(None));
    let mem_sub = (backend.subscribe_memory_warning).clone();
    if (mem_sub).is_some() {
        (*unsubscribe_memory.lock().unwrap()) =
            Some(((mem_sub.as_ref().unwrap()).clone()).lock().unwrap()(
                std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                    let app = app.clone();
                    move |level: AppMemoryPressure| -> () {
                        emit_signal((app.on_memory_warning).clone(), ((level).clone(),));
                    }
                })
                    as Box<dyn FnMut(AppMemoryPressure) -> () + Send + 'static>)),
            ));
    }
    {
        let __flight_key = (app).clone();
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut unsubscribe_memory = unsubscribe_memory.clone();
            let unsubscribe_state = unsubscribe_state.clone();
            move || -> () {
                ((unsubscribe_state).clone()).lock().unwrap()();
                {
                    let __flight_callback = (*unsubscribe_memory.lock().unwrap()).clone();
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()())
                };
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
        if let Some((_, value)) = (*_SUBSCRIPTIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:60 (sha256:bfa2afd6df640f16b254d006267c860579032d4a52ee2225d79d01154c1fd794)
pub fn create_app_lifecycle() -> AppLifecycle {
    return AppLifecycle {
        __flight_identity: std::sync::Arc::new(()),
        on_state_change: create_signal(),
        on_resume: create_signal(),
        on_pause: create_signal(),
        on_back_button: create_signal(),
        on_memory_warning: create_signal(),
        on_save_state: create_signal(),
        on_restore_state: create_signal(),
    };
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:89 (sha256:2fd3dd4fe0b17b1641da016ca5bcf360ceac15921173494ab985a0b0c1ed8fd7)
pub fn create_web_lifecycle_backend() -> LifecycleBackend {
    let _window_focused: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(("undefined" != "undefined")));
    return LifecycleBackend {
        __flight_identity: std::sync::Arc::new(()),
        get_state: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _window_focused = _window_focused.clone();
            move || -> AppLifecycleState {
                return "active".to_owned();
            }
        })
            as Box<dyn FnMut() -> AppLifecycleState + Send + 'static>)),
        subscribe: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _window_focused = _window_focused.clone();
            move |listener: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>| -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
      return std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> () {

      }) as Box<dyn FnMut() -> () + Send + 'static>));
    }
        })
            as Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >)),
        get_launch_kind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> AppLaunchKind {
                return "cold".to_owned();
            },
        )
            as Box<dyn FnMut() -> AppLaunchKind + Send + 'static>))),
        subscribe_memory_warning: Some(std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |listener: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(AppMemoryPressure) -> () + Send + 'static>>,
                >|
                      -> std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                > {
                    return std::sync::Arc::new(std::sync::Mutex::new(
                        Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
                    ));
                },
            )
                as Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(AppMemoryPressure) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        ))),
    };
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:168 (sha256:3f335fe185d87319b6458d7a847d7fe2c2f562f545cd787b503feb4e5af291b9)
pub fn detach_app_lifecycle(app: &AppLifecycle) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*app).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        ((unsubscribe.as_ref().unwrap()).clone()).lock().unwrap()();
        {
            let __flight_key = (*app).clone();
            if let Some(__flight_index) = (*_SUBSCRIPTIONS.lock().unwrap())
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                (*_SUBSCRIPTIONS.lock().unwrap()).remove(__flight_index);
                true
            } else {
                false
            }
        };
    }
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:178 (sha256:9e994257ee4c63b4e37a0932c77616916cfcbc84270728cad4d9bf8ffe4a2b71)
pub fn dispose_app_lifecycle(app: &AppLifecycle) -> () {
    detach_app_lifecycle(app);
    {
        let __flight_key = (*app).clone();
        if let Some(__flight_index) = (*_SAVED_STATE.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_SAVED_STATE.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:187 (sha256:557f8522c19e6473f4049b5c199e14d7ac5a675a47f4cb2ce692b52906f22f84)
pub fn get_app_launch_kind() -> AppLaunchKind {
    let backend = get_lifecycle_backend();
    return if ((backend.get_launch_kind).clone()).is_some() {
        backend.get_launch_kind.as_ref().unwrap().lock().unwrap()()
    } else {
        "warm".to_owned()
    };
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:193 (sha256:c1392d830e9ab89b26fb50a32280ba73768d9c941f433aa9482f658a98469c92)
pub fn get_app_lifecycle_state() -> AppLifecycleState {
    return ((get_lifecycle_backend().get_state).clone())
        .lock()
        .unwrap()();
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:198 (sha256:6811261a7946ac75681e265bed72cddff3860ac612efdc2e32df77fe916dc590)
pub fn get_lifecycle_backend() -> LifecycleBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_lifecycle_backend());
    }
    return ((*_BACKEND.lock().unwrap()).clone()).clone().unwrap();
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:204 (sha256:d882d858252044c15716ada8699c25e7e235e8cc2d5f117c590718ab3639247f)
pub fn is_app_active() -> bool {
    return (((get_lifecycle_backend().get_state).clone())
        .lock()
        .unwrap()()
        == "active");
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:209 (sha256:521ed26d9bc2baab71a45784f52a6a7fcc99028e8e346bbd2c4d172ba0e904cc)
pub fn is_app_background() -> bool {
    return (((get_lifecycle_backend().get_state).clone())
        .lock()
        .unwrap()()
        == "background");
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:215 (sha256:c9ff5acfd9137da4fa5e14db02abad7a223be4c84827f5f3ab9186b2eb5063d8)
pub fn is_app_inactive() -> bool {
    return (((get_lifecycle_backend().get_state).clone())
        .lock()
        .unwrap()()
        == "inactive");
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:224 (sha256:4000bffc1a6de20600666c12896b5f39a46e54701b89c5743643ddbdb3065b06)
pub fn request_app_back(app: &AppLifecycle) -> bool {
    emit_signal((app.on_back_button).clone(), ());
    return !((app
        .on_back_button
        .data
        .as_ref()
        .map(|value| value.inner.lock().unwrap().cancelled))
        == Some(true));
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:230 (sha256:495f12e83c7ed6cf8511ae39f123fe3b6af24a5780feb5b172af50ecbb0bf92f)
pub fn set_lifecycle_backend(backend: Option<LifecycleBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/lifecycle/src/lifecycle.ts:234 (sha256:b66917944f74882aa564801f7963bd86110c1b136d8a674e7955dfe5738b85f0)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<LifecycleBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/lifecycle/src/lifecycle.ts:235 (sha256:cc52420f3c11c0ead53914ccea887154beeb987597512d86355e067c13f3f3e3)
static _SAVED_STATE: std::sync::LazyLock<
    std::sync::Mutex<Vec<(AppLifecycle, Vec<(String, crate::OpaqueHostValue)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/lifecycle/src/lifecycle.ts:236 (sha256:eeb3737fbc1c1f56a7a809443270d83de35641bbcd610f48b3f1715947a3bc8c)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            AppLifecycle,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
