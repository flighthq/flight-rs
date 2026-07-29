// @generated from upstream/packages/updater/src/updater.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    AppUpdater, UpdateInfo, UpdateProgress, UpdaterBackend, UpdaterConfig, UpdaterError,
    UpdaterSignatureConfig, UpdaterState,
};

// Source: upstream/packages/updater/src/updater.ts:15 (sha256:fd8a1317be06ca50a269122e314ed5613fb826740b104ed95176ff688bb5cee9)
pub fn attach_app_updater(updater: AppUpdater) -> () {
    detach_app_updater(&updater);
    let backend = get_updater_backend();
    let unsubscribes = vec![
        {
            let __flight_callback = (backend.subscribe_checking).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move || -> () {
                        _set_state(
                            &updater,
                            &(crate::FlightUnion2::<
                                UpdaterState,
                                std::sync::Arc<
                                    std::sync::Mutex<
                                        Box<
                                            dyn FnMut(UpdaterState) -> UpdaterState
                                                + Send
                                                + 'static,
                                        >,
                                    >,
                                >,
                            >::A(UpdaterState {
                                __flight_identity: std::sync::Arc::new(()),
                                phase: "Checking".to_owned(),
                                info: None,
                                progress: None,
                                error: None,
                            })),
                        );
                        emit_signal((updater.on_checking).clone(), ());
                    }
                })
                    as Box<dyn FnMut() -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_update_available).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move |info: UpdateInfo| -> () {
                        _set_state(
                            &updater,
                            &(crate::FlightUnion2::<
                                UpdaterState,
                                std::sync::Arc<
                                    std::sync::Mutex<
                                        Box<
                                            dyn FnMut(UpdaterState) -> UpdaterState
                                                + Send
                                                + 'static,
                                        >,
                                    >,
                                >,
                            >::A(UpdaterState {
                                __flight_identity: std::sync::Arc::new(()),
                                phase: "UpdateAvailable".to_owned(),
                                info: Some((info).clone()),
                                progress: None,
                                error: None,
                            })),
                        );
                        emit_signal((updater.on_update_available).clone(), ((info).clone(),));
                    }
                })
                    as Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_update_not_available).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move || -> () {
                        _set_state(
                            &updater,
                            &(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                                move |prev: crate::OpaqueHostValue| -> f64 {
                                    UpdaterState {
                                        phase: "Idle".to_owned(),
                                        ..(prev).clone()
                                    }
                                },
                            )
                                as Box<
                                    dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static,
                                >))),
                        );
                        emit_signal((updater.on_update_not_available).clone(), ());
                    }
                })
                    as Box<dyn FnMut() -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_download_progress).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move |progress: UpdateProgress| -> () {
                        _set_state(
                            &updater,
                            &(std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                                let progress = progress.clone();
                                move |prev: crate::OpaqueHostValue| -> f64 {
                                    UpdaterState {
                                        phase: "Downloading".to_owned(),
                                        progress: Some((progress).clone()),
                                        ..(prev).clone()
                                    }
                                }
                            })
                                as Box<
                                    dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static,
                                >))),
                        );
                        emit_signal(
                            (updater.on_download_progress).clone(),
                            ((progress).clone(),),
                        );
                    }
                })
                    as Box<dyn FnMut(UpdateProgress) -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_update_downloaded).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move |info: UpdateInfo| -> () {
                        _set_state(
                            &updater,
                            &(std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                                let info = info.clone();
                                move |prev: crate::OpaqueHostValue| -> f64 {
                                    UpdaterState {
                                        phase: "Downloaded".to_owned(),
                                        info: Some((info).clone()),
                                        progress: None,
                                        ..(prev).clone()
                                    }
                                }
                            })
                                as Box<
                                    dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static,
                                >))),
                        );
                        emit_signal((updater.on_update_downloaded).clone(), ((info).clone(),));
                    }
                })
                    as Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_error).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move |error: UpdaterError| -> () {
                        _set_state(
                            &updater,
                            &(std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                                let error = error.clone();
                                move |prev: crate::OpaqueHostValue| -> f64 {
                                    UpdaterState {
                                        phase: "Error".to_owned(),
                                        error: Some((error).clone()),
                                        ..(prev).clone()
                                    }
                                }
                            })
                                as Box<
                                    dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static,
                                >))),
                        );
                        emit_signal((updater.on_error).clone(), ((error).clone(),));
                    }
                })
                    as Box<dyn FnMut(UpdaterError) -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_update_cancelled).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move || -> () {
                        _set_state(
                            &updater,
                            &(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                                move |prev: crate::OpaqueHostValue| -> f64 {
                                    UpdaterState {
                                        phase: "Idle".to_owned(),
                                        ..(prev).clone()
                                    }
                                },
                            )
                                as Box<
                                    dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static,
                                >))),
                        );
                        emit_signal((updater.on_update_cancelled).clone(), ());
                    }
                })
                    as Box<dyn FnMut() -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_update_staging).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move || -> () {
                        _set_state(
                            &updater,
                            &(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                                move |prev: crate::OpaqueHostValue| -> f64 {
                                    UpdaterState {
                                        phase: "Staging".to_owned(),
                                        ..(prev).clone()
                                    }
                                },
                            )
                                as Box<
                                    dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static,
                                >))),
                        );
                        emit_signal((updater.on_update_staging).clone(), ());
                    }
                })
                    as Box<dyn FnMut() -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_update_verified).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move || -> () {
                        emit_signal((updater.on_update_verified).clone(), ());
                    }
                })
                    as Box<dyn FnMut() -> () + Send + 'static>),
            ));
            __flight_result
        },
        {
            let __flight_callback = (backend.subscribe_update_rolled_back).clone();
            let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
                std::sync::Mutex::new(Box::new({
                    let updater = updater.clone();
                    move || -> () {
                        _set_state(
                            &updater,
                            &(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                                move |prev: crate::OpaqueHostValue| -> f64 {
                                    UpdaterState {
                                        phase: "Idle".to_owned(),
                                        info: None,
                                        progress: None,
                                        error: None,
                                        ..(prev).clone()
                                    }
                                },
                            )
                                as Box<
                                    dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static,
                                >))),
                        );
                        emit_signal((updater.on_update_rolled_back).clone(), ());
                    }
                })
                    as Box<dyn FnMut() -> () + Send + 'static>),
            ));
            __flight_result
        },
    ];
    {
        let __flight_key = (updater).clone();
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let unsubscribes = unsubscribes.clone();
            move || -> () {
                for unsubscribe in (unsubscribes).iter().cloned() {
                    {
                        let __flight_callback = (unsubscribe).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    };
                }
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

// Source: upstream/packages/updater/src/updater.ts:66 (sha256:75c1e146a55599d6c0258a55bfdaa70534a64288afab3b362a6a56afb47e233f)
pub fn cancel_app_update_download() -> () {
    {
        let __flight_callback = (get_updater_backend().cancel_download).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:72 (sha256:a454c82981843f23d7d065d458ab0db86edf88648067fb96517f013b40219d19)
pub fn check_and_download_app_update() -> () {
    let config = get_updater_config();
    {
        let __flight_callback = (get_updater_backend().check_for_updates).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    if config.auto_download {
        {
            let __flight_callback = (get_updater_backend().download_update).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
}

// Source: upstream/packages/updater/src/updater.ts:82 (sha256:dec0b7f68ac4336bcc82c8647c767219f3b312e166383b0661ee3479d85e5cbf)
pub fn check_for_app_update() -> () {
    {
        let __flight_callback = (get_updater_backend().check_for_updates).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:88 (sha256:08f2bafe1650d403befd73af2252551ac697a296466be384a96fe0979658c2b2)
pub fn create_app_updater() -> AppUpdater {
    let updater: AppUpdater = AppUpdater {
        __flight_identity: std::sync::Arc::new(()),
        on_checking: create_signal(),
        on_download_progress: create_signal(),
        on_error: create_signal(),
        on_update_available: create_signal(),
        on_update_cancelled: create_signal(),
        on_update_downloaded: create_signal(),
        on_update_not_available: create_signal(),
        on_update_rolled_back: create_signal(),
        on_update_staging: create_signal(),
        on_update_verified: create_signal(),
    };
    {
        let __flight_key = (updater).clone();
        let __flight_value = create_updater_state();
        if let Some((_, value)) = (*_STATES.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_STATES.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return updater;
}

// Source: upstream/packages/updater/src/updater.ts:107 (sha256:738ce195afda78080525e61b0312e3c6c01ad49cb3dae2f7757c0e96a96baf87)
#[derive(Clone, Default)]
struct CreateUpdaterConfigRecord1 {
    __flight_identity: std::sync::Arc<()>,
    allow_prerelease: bool,
    auto_download: bool,
    auto_install_on_app_quit: bool,
}
impl PartialEq for CreateUpdaterConfigRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_updater_config() -> UpdaterConfig {
    return UpdaterConfig {
        __flight_identity: std::sync::Arc::new(()),
        allow_prerelease: false,
        auto_download: false,
        auto_install_on_app_quit: false,
    };
}

// Source: upstream/packages/updater/src/updater.ts:116 (sha256:1948ab64f8f044242ff1cef20f246907b622678626b0d81f40fafc3fe272b5c7)
pub fn create_updater_state() -> UpdaterState {
    return UpdaterState {
        __flight_identity: std::sync::Arc::new(()),
        error: None,
        info: None,
        phase: "Idle".to_owned(),
        progress: None,
    };
}

// Source: upstream/packages/updater/src/updater.ts:127 (sha256:da62bc8a2a6db684d4950e6d99130792d0a760b35cc00d1c029bc550ad834dbe)
pub fn create_web_updater_backend() -> UpdaterBackend {
    let _config: std::sync::Arc<std::sync::Mutex<UpdaterConfig>> =
        std::sync::Arc::new(std::sync::Mutex::new(create_updater_config()));
    let _channel: std::sync::Arc<std::sync::Mutex<String>> =
        std::sync::Arc::new(std::sync::Mutex::new("stable"));
    return UpdaterBackend {
        __flight_identity: std::sync::Arc::new(()),
        cancel_download: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        check_for_updates: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        download_update: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        get_channel: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _channel = _channel.clone();
            move || -> String {
                return (*_channel.lock().unwrap()).clone();
            }
        })
            as Box<dyn FnMut() -> String + Send + 'static>)),
        get_config: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _config = _config.clone();
            move || -> UpdaterConfig {
                return (*_config.lock().unwrap()).clone();
            }
        })
            as Box<dyn FnMut() -> UpdaterConfig + Send + 'static>)),
        quit_and_install: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        rollback: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        set_channel: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _channel = _channel.clone();
            move |channel: String| -> () {
                (*_channel.lock().unwrap()) = (channel).clone();
            }
        })
            as Box<dyn FnMut(String) -> () + Send + 'static>)),
        set_config: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut _config = _config.clone();
            move |config: UpdaterConfig| -> () {
                (*_config.lock().unwrap()) = (config).clone();
            }
        })
            as Box<dyn FnMut(UpdaterConfig) -> () + Send + 'static>)),
        set_feed_url: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> () {},
        )
            as Box<dyn FnMut(String) -> () + Send + 'static>)),
        set_signature_config: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: Option<UpdaterSignatureConfig>| -> () {},
        )
            as Box<dyn FnMut(Option<UpdaterSignatureConfig>) -> () + Send + 'static>)),
        subscribe_checking: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_download_progress: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(UpdateProgress) -> () + Send + 'static>>,
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
                                    Box<dyn FnMut(UpdateProgress) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_error: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(UpdaterError) -> () + Send + 'static>>,
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
                                    Box<dyn FnMut(UpdaterError) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_update_available: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>,
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
                                std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_update_cancelled: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_update_downloaded: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |__flight_unused_0: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>,
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
                                std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
        subscribe_update_not_available: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_update_rolled_back: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_update_staging: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_update_verified: std::sync::Arc::new(std::sync::Mutex::new(
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
    };
}

// Source: upstream/packages/updater/src/updater.ts:184 (sha256:855ed0fb255c2b6e2e4784c30d55d6e879478b89ffa3781077c73942142a1d9e)
pub fn detach_app_updater(updater: &AppUpdater) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*updater).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        {
            let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        {
            let __flight_key = (*updater).clone();
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

// Source: upstream/packages/updater/src/updater.ts:194 (sha256:d98e52e9c1921f043660b91888fe268b12a3b372733cf2dbaee7b0ef14e48e79)
pub fn dispose_app_updater(updater: &AppUpdater) -> () {
    detach_app_updater(updater);
}

// Source: upstream/packages/updater/src/updater.ts:200 (sha256:b4ef9537ce5e38ee7c5366c806b2490def09ee66b5e9bff8b44135ba70cf21dc)
pub fn download_app_update() -> () {
    {
        let __flight_callback = (get_updater_backend().download_update).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:207 (sha256:8ee035f77b829a74dde4ac2aaec19cee00332412a2dd7f254ab93c8ecaeb783d)
pub fn get_app_updater_state(updater: &AppUpdater) -> UpdaterState {
    return ((*_STATES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*updater).clone())
        .map(|(_, value)| value.clone()))
    .unwrap_or(create_updater_state());
}

// Source: upstream/packages/updater/src/updater.ts:212 (sha256:d0d9a1c8583827a1d23075f92434486ad92f576c00452605fc827d736724f7a7)
pub fn get_updater_backend() -> UpdaterBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_updater_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/updater/src/updater.ts:218 (sha256:7957891f4f20d1bbbccc1aced1d46d2724efb374ba9323ca7887f92dd3d60bef)
pub fn get_updater_channel() -> String {
    return {
        let __flight_callback = (get_updater_backend().get_channel).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:223 (sha256:d4ee2cdb56bc5563302b4612044575a9a396bbc8fc6a6d46a01921910d96943b)
pub fn get_updater_config() -> UpdaterConfig {
    return {
        let __flight_callback = (get_updater_backend().get_config).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:231 (sha256:d50ed20c8a6641d661e4048d5b51bb9b168ce343793c85888bd76b0b79fe14f9)
pub fn is_app_update_eligible(info: &UpdateInfo, rollout_seed: f64) -> bool {
    return ((rollout_seed * 100.0_f64) < info.staged_rollout_percent);
}

// Source: upstream/packages/updater/src/updater.ts:236 (sha256:adab1c9ef574d353ed398a54571c07698371bf8d1789af669c6c2de40645a374)
pub fn quit_and_install_update() -> () {
    {
        let __flight_callback = (get_updater_backend().quit_and_install).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:242 (sha256:6c6afe297f273f6d6e4731f0a393cb67781f6d75a32e3c2f8c02920826cb5291)
pub fn rollback_app_update() -> () {
    {
        let __flight_callback = (get_updater_backend().rollback).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:247 (sha256:39ea0a19a899da404cc4c19b38107b1248a613e04dc7d11cb36ab04c07656724)
pub fn set_updater_backend(backend: Option<UpdaterBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/updater/src/updater.ts:253 (sha256:abc3961299f0d36af7a72625fe580b291b96c2112890a47e9832bbaa79c71286)
pub fn set_updater_channel(channel: String) -> () {
    {
        let __flight_callback = (get_updater_backend().set_channel).clone();
        let __flight_result = __flight_callback.lock().unwrap()((channel).clone());
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:259 (sha256:42974bff0b0812de77972e486e8e028be627bfa138a7072322d34e3c823438d2)
pub fn set_updater_config(config: &UpdaterConfig) -> () {
    {
        let __flight_callback = (get_updater_backend().set_config).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*config).clone());
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:264 (sha256:56c6db882f2db556d3bd92fefde3f763423b0467611e0be379808b5c7905515d)
pub fn set_updater_feed_url(url: String) -> () {
    {
        let __flight_callback = (get_updater_backend().set_feed_url).clone();
        let __flight_result = __flight_callback.lock().unwrap()((url).clone());
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:271 (sha256:c54b5fc45783e8e353ca16901bde6186fbcb4afe7bd107e8fdce8dc0e7c33988)
pub fn set_updater_signature_config(config: Option<UpdaterSignatureConfig>) -> () {
    {
        let __flight_callback = (get_updater_backend().set_signature_config).clone();
        let __flight_result = __flight_callback.lock().unwrap()((config).clone());
        __flight_result
    };
}

// Source: upstream/packages/updater/src/updater.ts:275 (sha256:ab35d09b587222f70dbe78fb088b7ab2c7629363f60dc40a0f5b8c79b012d737)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<UpdaterBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/updater/src/updater.ts:276 (sha256:4e9d73dca83cee0f34e86d9687320f0194552880ca4aec4a14b054fde5b6e2c8)
static _STATES: std::sync::LazyLock<std::sync::Mutex<Vec<(AppUpdater, UpdaterState)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/updater/src/updater.ts:277 (sha256:0ffb7cc9596df3e9001cc3e6d0f4b024d4d768627e4a9223f837e2f5ad16a30d)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            AppUpdater,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/updater/src/updater.ts:280 (sha256:f19e74d91d9e18ab1d428071bee6353e4875bdfa21941ddfd6f6a29fb9d2f1c2)
fn _set_state(
    updater: &AppUpdater,
    update: &crate::FlightUnion2<
        UpdaterState,
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(UpdaterState) -> UpdaterState + Send + 'static>>,
        >,
    >,
) -> () {
    let prev = ((*_STATES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*updater).clone())
        .map(|(_, value)| value.clone()))
    .unwrap_or(create_updater_state());
    let next = if (match &(update) {
        crate::FlightUnion2::A(_) => "object",
        crate::FlightUnion2::B(value) => "function",
    } == "function")
    {
        {
            let __flight_callback = match (*update).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            };
            let __flight_result = __flight_callback.lock().unwrap()((prev).clone());
            __flight_result
        }
    } else {
        match (*update).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        }
    };
    {
        let __flight_key = (*updater).clone();
        let __flight_value = (next).clone();
        if let Some((_, value)) = (*_STATES.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_STATES.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}
