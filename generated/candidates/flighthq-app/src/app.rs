// @generated from upstream/packages/app/src/app.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    App, AppActivationPolicy, AppBackend, AppLoginItem, AppLoginItemLike, AppPathKind,
    MenuItemTemplate,
};

// Source: upstream/packages/app/src/app.ts:14 (sha256:a4137265daf5e7a9562c215db3196a71166ba89e3a89bc932b81d7c9025760fc)
pub fn add_app_recent_document(path: String) -> () {
    {
        let __flight_callback = (get_app_backend().add_recent_document).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:23 (sha256:bcbf7e4d07dca2cd23f7ffea9ed95b8ce69e358368b816234d66c2b7bd16122f)
pub fn attach_app(app: App) -> () {
    detach_app(&app);
    let backend = get_app_backend();
    let unsubscribe_activate = {
        let __flight_callback = (backend.subscribe_activate).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let app = app.clone();
                move || -> () { emit_signal((app.on_activate).clone(), ()) }
            }) as Box<dyn FnMut() -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_all_windows_closed = {
        let __flight_callback = (backend.subscribe_all_windows_closed).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let app = app.clone();
                move || -> () { emit_signal((app.on_all_windows_closed).clone(), ()) }
            }) as Box<dyn FnMut() -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_open_file = {
        let __flight_callback = (backend.subscribe_open_file).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let app = app.clone();
                move |path: String| -> () {
                    emit_signal((app.on_open_file).clone(), ((path).clone(),))
                }
            })
                as Box<dyn FnMut(String) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_quit_request = {
        let __flight_callback = (backend.subscribe_quit_request).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let app = app.clone();
                let backend = backend.clone();
                move |cancel_host: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                >|
                      -> () {
                    emit_signal((app.on_quit_request).clone(), ());
                    if (app
                        .on_quit_request
                        .data
                        .as_ref()
                        .map(|value| value.inner.lock().unwrap().cancelled))
                        == Some(true)
                    {
                        {
                            let __flight_callback = (cancel_host).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        };
                    } else {
                        {
                            let __flight_callback = (backend.quit).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        };
                    }
                }
            })
                as Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >),
        ));
        __flight_result
    };
    let unsubscribe_ready = {
        let __flight_callback = (backend.subscribe_ready).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let app = app.clone();
                move || -> () { emit_signal((app.on_ready).clone(), ()) }
            }) as Box<dyn FnMut() -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_second_instance = {
        let __flight_callback = (backend.subscribe_second_instance).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let app = app.clone();
                move |argv: Vec<String>| -> () {
                    emit_signal((app.on_second_instance).clone(), ((argv).clone(),))
                }
            })
                as Box<dyn FnMut(Vec<String>) -> () + Send + 'static>),
        ));
        __flight_result
    };
    {
        let __flight_key = (app).clone();
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let unsubscribe_activate = unsubscribe_activate.clone();
            let unsubscribe_all_windows_closed = unsubscribe_all_windows_closed.clone();
            let unsubscribe_open_file = unsubscribe_open_file.clone();
            let unsubscribe_quit_request = unsubscribe_quit_request.clone();
            let unsubscribe_ready = unsubscribe_ready.clone();
            let unsubscribe_second_instance = unsubscribe_second_instance.clone();
            move || -> () {
                {
                    let __flight_callback = (unsubscribe_activate).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_all_windows_closed).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_open_file).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_quit_request).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_ready).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_second_instance).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
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

// Source: upstream/packages/app/src/app.ts:52 (sha256:fee97c9bb232b5d9ce5df3d3190f2d05e237167fccd9e7d6726a8e02d7fb0869)
pub fn bounce_app_dock() -> f64 {
    return {
        let __flight_callback = (get_app_backend().bounce_dock).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:57 (sha256:05f723bd981d59367b10c8c42168a9517841069cc54f32657b07f4085c18d1a4)
pub fn cancel_app_attention(id: f64) -> () {
    {
        let __flight_callback = (get_app_backend().cancel_attention).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id);
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:62 (sha256:c21caf294d4a4b731d081a1ef5e7cc3c9df0b881ab4b89aad2459a54ee8f14ff)
pub fn cancel_app_dock_bounce(id: f64) -> () {
    {
        let __flight_callback = (get_app_backend().cancel_dock_bounce).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id);
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:67 (sha256:3d25d39773be2da1c7bcb8c290384e9f3fba37716ce2bd19eab46a35f86baa30)
pub fn clear_app_recent_documents() -> () {
    {
        let __flight_callback = (get_app_backend().clear_recent_documents).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:72 (sha256:284f56aa69d83a45bf7e53a63fc6cd38ffa491ef9d2dcd363310e7725b09ee62)
pub fn create_app() -> App {
    return App {
        __flight_identity: std::sync::Arc::new(()),
        on_activate: create_signal(),
        on_all_windows_closed: create_signal(),
        on_open_file: create_signal(),
        on_quit_request: create_signal(),
        on_ready: create_signal(),
        on_second_instance: create_signal(),
    };
}

// Source: upstream/packages/app/src/app.ts:84 (sha256:a5d5235d419b4478730ee78867b50f74feb0f1b708470bb6479e9af1871c057f)
pub fn create_app_login_item() -> AppLoginItem {
    return AppLoginItem {
        __flight_identity: std::sync::Arc::new(()),
        args: vec![],
        open_as_hidden: false,
        open_at_login: false,
        path: "".to_owned(),
    };
}

// Source: upstream/packages/app/src/app.ts:90 (sha256:4789c56b23ccfe441ad4f4ea3690516f737cf5fa36f9a153cbcc94dafca736d5)
#[derive(Clone)]
struct CreateWebAppBackendRecord1 {
    __flight_identity: std::sync::Arc<()>,
    set_app_badge: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<f64>) -> crate::Promise<()> + Send + 'static>>,
    >,
}
impl PartialEq for CreateWebAppBackendRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_web_app_backend() -> AppBackend {
    return AppBackend {
        __flight_identity: std::sync::Arc::new(()),
        add_recent_document: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> () {},
        )
            as Box<dyn FnMut(String) -> () + Send + 'static>)),
        bounce_dock: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> f64 {
            return (-1.0_f64);
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)),
        cancel_attention: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> () {},
        )
            as Box<dyn FnMut(f64) -> () + Send + 'static>)),
        cancel_dock_bounce: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: f64| -> () {},
        )
            as Box<dyn FnMut(f64) -> () + Send + 'static>)),
        clear_recent_documents: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>,
        )),
        focus: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        get_app_directory_path: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: AppPathKind| -> String {
                return "".to_owned();
            },
        )
            as Box<dyn FnMut(AppPathKind) -> String + Send + 'static>)),
        get_app_path: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> String {
            return "".to_owned();
        })
            as Box<dyn FnMut() -> String + Send + 'static>)),
        get_command_line: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> Vec<String> {
                return vec![];
            },
        )
            as Box<dyn FnMut() -> Vec<String> + Send + 'static>)),
        get_executable_path: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> String {
                return "".to_owned();
            },
        )
            as Box<dyn FnMut() -> String + Send + 'static>)),
        get_locale: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> String {
            return "".to_owned();
        })
            as Box<dyn FnMut() -> String + Send + 'static>)),
        get_preferred_system_languages: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> Vec<String> {
                return vec![];
            },
        )
            as Box<dyn FnMut() -> Vec<String> + Send + 'static>)),
        get_system_locale: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> String {
            let __flight_try_return: Option<String> = match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| -> Option<String> {
                    {
                        return Some("".to_owned());
                    }
                    None
                }),
            ) {
                Ok(value) => value,
                Err(_) => (|| -> Option<String> {
                    {
                        return Some("".to_owned());
                    }
                    None
                })(),
            };
            return __flight_try_return.expect("TypeScript try/catch completed without returning");
        })
            as Box<dyn FnMut() -> String + Send + 'static>)),
        get_login_item: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> AppLoginItem {
                return AppLoginItem {
                    __flight_identity: std::sync::Arc::new(()),
                    args: vec![],
                    open_as_hidden: false,
                    open_at_login: false,
                    path: "".to_owned(),
                };
            },
        )
            as Box<dyn FnMut() -> AppLoginItem + Send + 'static>)),
        get_name: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> String {
            return "".to_owned();
        })
            as Box<dyn FnMut() -> String + Send + 'static>)),
        get_version: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> String {
            return "".to_owned();
        })
            as Box<dyn FnMut() -> String + Send + 'static>)),
        has_single_instance_lock: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> bool {
                return true;
            },
        )
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        hide_app: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> bool {
            return false;
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        is_app_hidden: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> bool {
            return false;
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        quit: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        relaunch: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        release_single_instance_lock: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> () {},
        )
            as Box<dyn FnMut() -> () + Send + 'static>)),
        request_attention: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: bool| -> f64 {
                return (-1.0_f64);
            },
        )
            as Box<dyn FnMut(bool) -> f64 + Send + 'static>)),
        request_single_instance_lock: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move || -> bool {
                return true;
            },
        )
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        set_activation_policy: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: AppActivationPolicy| -> () {},
        )
            as Box<dyn FnMut(AppActivationPolicy) -> () + Send + 'static>)),
        set_badge_count: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |count: f64| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(f64) -> bool + Send + 'static>)),
        set_dock_badge: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> () {},
        )
            as Box<dyn FnMut(String) -> () + Send + 'static>)),
        set_dock_menu: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: Vec<MenuItemTemplate>| -> () {},
        )
            as Box<dyn FnMut(Vec<MenuItemTemplate>) -> () + Send + 'static>)),
        set_login_item: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: AppLoginItemLike| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(AppLoginItemLike) -> bool + Send + 'static>)),
        set_name: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        set_user_model_id: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> bool {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        show_app: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> bool {
            return false;
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        subscribe_activate: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
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
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_all_windows_closed: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
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
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_open_file: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
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
                                std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_quit_request: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |_listener: std::sync::Arc<
                    std::sync::Mutex<
                        Box<
                            dyn FnMut(
                                    std::sync::Arc<
                                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                                    >,
                                ) -> ()
                                + Send
                                + 'static,
                        >,
                    >,
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
                                    Box<
                                        dyn FnMut(
                                                std::sync::Arc<
                                                    std::sync::Mutex<
                                                        Box<dyn FnMut() -> () + Send + 'static>,
                                                    >,
                                                >,
                                            ) -> ()
                                            + Send
                                            + 'static,
                                    >,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_ready: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |listener: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                >|
                      -> std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                > {
                    let id = {
                        let __flight_promise = { crate::Promise::<()>::default() };
                        let __flight_callback =
                            std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                                let listener = listener.clone();
                                move |__flight_unused_0: ()| -> () {
                                    {
                                        let __flight_callback = (listener).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    }
                                }
                            })
                                as Box<dyn FnMut(()) -> () + Send + 'static>));
                        let _ = (&__flight_promise, &__flight_callback);
                        crate::Promise::<()>::default()
                    };
                    {
                        id;
                        ()
                    };
                    return std::sync::Arc::new(std::sync::Mutex::new(
                        Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
                    ));
                },
            )
                as Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_second_instance: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(Vec<String>) -> () + Send + 'static>>,
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
                                    Box<dyn FnMut(Vec<String>) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
    };
}

// Source: upstream/packages/app/src/app.ts:265 (sha256:c1adf5a811275acefe5bab2468eddde82387ba0af4730936ab8a4d0b06206ef4)
pub fn detach_app(app: &App) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*app).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        {
            let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
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

// Source: upstream/packages/app/src/app.ts:275 (sha256:ff1ae97a3476392d0d2494928f07e4af9bcb71b2a10c8e18e2599a0e277d76ad)
pub fn dispose_app(app: &App) -> () {
    detach_app(app);
}

// Source: upstream/packages/app/src/app.ts:280 (sha256:920279f6b29abb9955687fbf08f9243f8179546c76fdab6c9807319943cae4b9)
pub fn focus_app() -> () {
    {
        let __flight_callback = (get_app_backend().focus).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:285 (sha256:a53a4b517977d7f0daac92ebaec57ba7414b75caa4616af29c817c2a07ca2738)
pub fn get_app_backend() -> AppBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_app_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/app/src/app.ts:291 (sha256:da105da55ade13169ba8780bd0157aa73cc42223019ff2d812a107df539070e4)
pub fn get_app_command_line() -> Vec<String> {
    return {
        let __flight_callback = (get_app_backend().get_command_line).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:296 (sha256:d84f266e0f740e349ddc9279b6b8933de1f04906d52e4da27647a836eec7d2ee)
pub fn get_app_command_line_switch(name: String) -> Option<String> {
    let prefix = format!("--{}=", name);
    let args = {
        let __flight_callback = (get_app_backend().get_command_line).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    for arg in (args).iter().cloned() {
        if (arg == format!("--{}", name)) {
            return Some("".to_owned());
        }
        if (arg).starts_with(((prefix).clone()).as_str()) {
            return Some(String::from_utf16_lossy(
                &(arg)
                    .encode_utf16()
                    .skip((prefix.encode_utf16().count() as f64) as usize)
                    .collect::<Vec<u16>>(),
            ));
        }
    }
    return None;
}

// Source: upstream/packages/app/src/app.ts:308 (sha256:24420127daafcf3ed45afd16425ebf5cd9d75ea62f6a568ef6b185398026b79c)
pub fn get_app_directory_path(kind: AppPathKind) -> String {
    return {
        let __flight_callback = (get_app_backend().get_app_directory_path).clone();
        let __flight_result = __flight_callback.lock().unwrap()((kind).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:313 (sha256:8bf1fc8ea7251cad960873acfa91eea0b5eb4f278a1ddabb2bac28ba563fceea)
pub fn get_app_executable_path() -> String {
    return {
        let __flight_callback = (get_app_backend().get_executable_path).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:318 (sha256:0b876c036e736f21b804696b6654ec957d4f8c7eaf7cddee70927de4a35d476d)
pub fn get_app_locale() -> String {
    return {
        let __flight_callback = (get_app_backend().get_locale).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:323 (sha256:3c4820c7e395c6bac4be540cdf3b9bfd9a2208c943b6e8113eb95ce8dca8aa29)
pub fn get_app_login_item() -> AppLoginItem {
    return {
        let __flight_callback = (get_app_backend().get_login_item).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:328 (sha256:a29f6e404e33e6f5f90ef918f906db5300d944520a60ce541fb8e9108e971f4f)
pub fn get_app_name() -> String {
    return {
        let __flight_callback = (get_app_backend().get_name).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:333 (sha256:8738886389c8059750d13a5242b1b5f287b096a05e902251dc79571e148c4368)
pub fn get_app_path() -> String {
    return {
        let __flight_callback = (get_app_backend().get_app_path).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:339 (sha256:f5145d3c286ce49a2f51ff4831efa2c8c343f1015549f82dec3b2205cf58e092)
pub fn get_app_preferred_system_languages() -> Vec<String> {
    return {
        let __flight_callback = (get_app_backend().get_preferred_system_languages).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:345 (sha256:47b525b381af103732770970cd86f653c5473acb996d98eeee079a83fa2dc6b4)
pub fn get_app_system_locale() -> String {
    return {
        let __flight_callback = (get_app_backend().get_system_locale).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:350 (sha256:d09cdf59937d3caa392a65cc23027caa3dd0357f3231ba3a9c5dbe03ea6bfd8c)
pub fn get_app_version() -> String {
    return {
        let __flight_callback = (get_app_backend().get_version).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:355 (sha256:8abc45eb35a8230c08ee7ef855bde7cc7a974bb5c7cdd68f7fb5000f050c38c8)
pub fn has_app_command_line_switch(name: String) -> bool {
    return (get_app_command_line_switch((name).clone())).is_some();
}

// Source: upstream/packages/app/src/app.ts:360 (sha256:4acf0dfbb2feded73dbd94336325772b320c4e59fd2e0f1b838df52e2f879d3d)
pub fn has_app_single_instance_lock() -> bool {
    return {
        let __flight_callback = (get_app_backend().has_single_instance_lock).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:365 (sha256:b6f7b18c8a1728be230c51a67c5611b1c38970b8e0299d549c6840e3dd8af66f)
pub fn hide_app() -> bool {
    return {
        let __flight_callback = (get_app_backend().hide_app).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:370 (sha256:3e5f8b4fde7edae1afdc4bf090977dbc5444d7dbe5e4c0748b7d31176ed04d4d)
pub fn is_app_hidden() -> bool {
    return {
        let __flight_callback = (get_app_backend().is_app_hidden).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:375 (sha256:823da62dd0e17eeea56cb6946fa1dca21c7c2bef7b4faff509260a928c3d3035)
pub fn quit_app() -> () {
    {
        let __flight_callback = (get_app_backend().quit).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:380 (sha256:835d8298f8f660362b5a3e4d89755e10e16a8e6b8780576f8f1a3c733776f1e3)
pub fn relaunch_app() -> () {
    {
        let __flight_callback = (get_app_backend().relaunch).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:385 (sha256:b3d567293d3f1da0d384a2302a03f8c615af34ffba8962a1c12fa34775772b77)
pub fn release_app_single_instance_lock() -> () {
    {
        let __flight_callback = (get_app_backend().release_single_instance_lock).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:391 (sha256:1d2acf044a8e81d4a8f158886c4514bba3c9777d01382413b31d62f70e37c2c0)
pub fn request_app_attention(critical: bool) -> f64 {
    return {
        let __flight_callback = (get_app_backend().request_attention).clone();
        let __flight_result = __flight_callback.lock().unwrap()(critical);
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:397 (sha256:e14ccc23128390865182006203e2bdeea98f2a9392229ae423488c1a7d456100)
pub fn request_app_single_instance_lock() -> bool {
    return {
        let __flight_callback = (get_app_backend().request_single_instance_lock).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:403 (sha256:4b319a083f115ac99a861c39cb271bb4a7b7f22e83921e3144c2a6fe45c49c2c)
pub fn set_app_activation_policy(policy: AppActivationPolicy) -> () {
    {
        let __flight_callback = (get_app_backend().set_activation_policy).clone();
        let __flight_result = __flight_callback.lock().unwrap()((policy).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:408 (sha256:bab95754ec2faefba337c31f65a5248c46498b0e3b22cfa18a767b00f7ff73a2)
pub fn set_app_backend(backend: Option<AppBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/app/src/app.ts:414 (sha256:449456a46bad981201faa38c97a3f2635f93fe47b10756f540d8415b17c55784)
pub fn set_app_badge_count(count: f64) -> bool {
    return {
        let __flight_callback = (get_app_backend().set_badge_count).clone();
        let __flight_result = __flight_callback.lock().unwrap()(count);
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:419 (sha256:13947ca8977121a5fe273898a1311c8a8c81ea9564c97e7e5171db02a88b8636)
pub fn set_app_dock_badge(text: String) -> () {
    {
        let __flight_callback = (get_app_backend().set_dock_badge).clone();
        let __flight_result = __flight_callback.lock().unwrap()((text).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:424 (sha256:2eb5ae083ff23bb4e083d5742ae947257e8ebc2a7cf27bd498ef1294e6e305b6)
pub fn set_app_dock_menu(items: &Vec<MenuItemTemplate>) -> () {
    {
        let __flight_callback = (get_app_backend().set_dock_menu).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*items).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:430 (sha256:c60d40e01e7c10718af3f2012fb516661e217ec5dbcf5778040c32e6475a5f1d)
pub fn set_app_login_item(settings: &AppLoginItemLike) -> bool {
    return {
        let __flight_callback = (get_app_backend().set_login_item).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*settings).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:436 (sha256:80b6a031cce0dccebc18772d8c53908d6911cc6dc75141b84432cf009e7a5a35)
pub fn set_app_name(name: String) -> bool {
    return {
        let __flight_callback = (get_app_backend().set_name).clone();
        let __flight_result = __flight_callback.lock().unwrap()((name).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:442 (sha256:0c42b0b8e97449ae335bf1450a5396a43aa8aab0349e259450a050651b7201eb)
pub fn set_app_user_model_id(id: String) -> bool {
    return {
        let __flight_callback = (get_app_backend().set_user_model_id).clone();
        let __flight_result = __flight_callback.lock().unwrap()((id).clone());
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:447 (sha256:c221facdc501e7151b7fc4137258525f939ae79bd99b8b3c0b2ce84102e0287f)
pub fn show_app() -> bool {
    return {
        let __flight_callback = (get_app_backend().show_app).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/app/src/app.ts:451 (sha256:14b678aa9c98d8b4e277abfc34a438bb2c8ea137f8aededa802068d0e89cfe22)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<AppBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/app/src/app.ts:452 (sha256:15425b3c4aca3cda2f600a0d9571438bb6e24fda25481b882c4ecf08c911fc1f)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            App,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
