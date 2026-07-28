// @generated from upstream/packages/power/src/power.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal, has_signal_slots};
use flighthq_types::{
    Power, PowerBackend, PowerBatteryHealth, PowerIdleState, PowerKeepAwakeMode, PowerStatus,
    PowerThermalState,
};

// Source: upstream/packages/power/src/power.ts:19 (sha256:054380b2c8004b957fa9a0d71bcec41fc179bc5ceaa4b2d1ad689665b970ee3f)
pub fn attach_power(power: Power, idle_threshold_seconds: Option<f64>) -> () {
    let idle_threshold_seconds = idle_threshold_seconds.unwrap_or(60.0_f64);
    detach_power(&power);
    let backend = get_power_backend();
    let was_charging: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(
            ((backend.get_status).clone()).lock().unwrap()(((*_SCRATCH).clone()).clone())
                .is_charging,
        ));
    let unsubscribe_change = ((backend.subscribe).clone()).lock().unwrap()(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let backend = backend.clone();
            let power = power.clone();
            let mut was_charging = was_charging.clone();
            move || -> () {
                let status =
                    ((backend.get_status).clone()).lock().unwrap()(((*_SCRATCH).clone()).clone());
                if ((power.on_change).clone()).is_some() {
                    emit_signal(((power.on_change).clone()).unwrap(), (status,));
                }
                if (status.is_charging != (*was_charging.lock().unwrap()).clone()) {
                    (*was_charging.lock().unwrap()) = status.is_charging;
                    let transition = if status.is_charging {
                        (power.on_charging).clone()
                    } else {
                        (power.on_discharging).clone()
                    };
                    if (transition).is_some() {
                        emit_signal(transition.as_ref().unwrap(), ());
                    }
                }
            }
        }) as Box<dyn FnMut() -> () + Send + 'static>),
    ));
    let unsubscribe_lock_screen = ((backend.subscribe_lock_screen).clone()).lock().unwrap()(
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let power = power.clone();
            move || -> () {
                if ((power.on_lock_screen).clone()).is_some() {
                    emit_signal(((power.on_lock_screen).clone()).unwrap(), ());
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
    let unsubscribe_low_power_mode_change = ((backend.subscribe_low_power_mode_change).clone())
        .lock()
        .unwrap()(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let power = power.clone();
            move || -> () {
                if ((power.on_low_power_mode_change).clone()).is_some() {
                    emit_signal(((power.on_low_power_mode_change).clone()).unwrap(), ());
                }
            }
        }) as Box<dyn FnMut() -> () + Send + 'static>),
    ));
    let unsubscribe_resume = ((backend.subscribe_resume).clone()).lock().unwrap()(
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let power = power.clone();
            move || -> () {
                if ((power.on_resume).clone()).is_some() {
                    emit_signal(((power.on_resume).clone()).unwrap(), ());
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
    let unsubscribe_suspend = ((backend.subscribe_suspend).clone()).lock().unwrap()(
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let power = power.clone();
            move || -> () {
                if ((power.on_suspend).clone()).is_some() {
                    emit_signal(((power.on_suspend).clone()).unwrap(), ());
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
    let unsubscribe_thermal_state_change = ((backend.subscribe_thermal_state_change).clone())
        .lock()
        .unwrap()(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let power = power.clone();
            move || -> () {
                if ((power.on_thermal_state_change).clone()).is_some() {
                    emit_signal(((power.on_thermal_state_change).clone()).unwrap(), ());
                }
            }
        }) as Box<dyn FnMut() -> () + Send + 'static>),
    ));
    let unsubscribe_unlock_screen = ((backend.subscribe_unlock_screen).clone()).lock().unwrap()(
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let power = power.clone();
            move || -> () {
                if ((power.on_unlock_screen).clone()).is_some() {
                    emit_signal(((power.on_unlock_screen).clone()).unwrap(), ());
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
    let last_idle_state: std::sync::Arc<std::sync::Mutex<PowerIdleState>> =
        std::sync::Arc::new(std::sync::Mutex::new(((backend.get_system_idle_state)
            .clone())
        .lock()
        .unwrap()(
            idle_threshold_seconds
        )));
    let idle_interval_id: Option<crate::FlightTimeout> = crate::set_interval(
        {
            let backend = backend.clone();
            let mut last_idle_state = last_idle_state.clone();
            let power = power.clone();
            move || -> () {
                let idle_signal = (power.on_idle_state_change).clone();
                if ((idle_signal).is_none() || (!has_signal_slots(&idle_signal))) {
                    return;
                }
                let current = ((backend.get_system_idle_state).clone()).lock().unwrap()(
                    idle_threshold_seconds,
                );
                if (current != (*last_idle_state.lock().unwrap()).clone()) {
                    (*last_idle_state.lock().unwrap()) = (current).clone();
                    emit_signal((idle_signal).clone().unwrap(), ());
                }
            }
        },
        (*_IDLE_POLLING_INTERVAL_MS.lock().unwrap()).clone(),
    );
    {
        let __flight_key = (power).clone();
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let unsubscribe_change = unsubscribe_change.clone();
            let unsubscribe_lock_screen = unsubscribe_lock_screen.clone();
            let unsubscribe_low_power_mode_change = unsubscribe_low_power_mode_change.clone();
            let unsubscribe_resume = unsubscribe_resume.clone();
            let unsubscribe_suspend = unsubscribe_suspend.clone();
            let unsubscribe_thermal_state_change = unsubscribe_thermal_state_change.clone();
            let unsubscribe_unlock_screen = unsubscribe_unlock_screen.clone();
            move || -> () {
                ((unsubscribe_change).clone()).lock().unwrap()();
                ((unsubscribe_lock_screen).clone()).lock().unwrap()();
                ((unsubscribe_low_power_mode_change).clone())
                    .lock()
                    .unwrap()();
                ((unsubscribe_resume).clone()).lock().unwrap()();
                ((unsubscribe_suspend).clone()).lock().unwrap()();
                ((unsubscribe_thermal_state_change).clone()).lock().unwrap()();
                ((unsubscribe_unlock_screen).clone()).lock().unwrap()();
                if let Some(__flight_timer) = (idle_interval_id).clone() {
                    crate::clear_interval(__flight_timer);
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

// Source: upstream/packages/power/src/power.ts:80 (sha256:f5d0c3edf8d35c422c3b0c415a80bfb4f1eef13921066a86fce23f8bcc330bc0)
pub fn create_power() -> Power {
    return Power {
        __flight_identity: std::sync::Arc::new(()),
        on_change: None,
        on_charging: None,
        on_discharging: None,
        on_idle_state_change: None,
        on_lock_screen: None,
        on_low_power_mode_change: None,
        on_resume: None,
        on_suspend: None,
        on_thermal_state_change: None,
        on_unlock_screen: None,
    };
}

// Source: upstream/packages/power/src/power.ts:96 (sha256:31220e79dea98836efc9259d9f01eddae2313fc885d4221b7b90d85ce2f76e08)
#[derive(Clone)]
struct CreatePowerBatteryHealthRecord1 {
    __flight_identity: std::sync::Arc<()>,
    capacity_wear_level: f64,
    cycle_count: f64,
    health_state: String,
    temperature_celsius: f64,
    voltage: f64,
}
impl PartialEq for CreatePowerBatteryHealthRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_power_battery_health() -> PowerBatteryHealth {
    return PowerBatteryHealth {
        __flight_identity: std::sync::Arc::new(()),
        capacity_wear_level: (-1.0_f64),
        cycle_count: (-1.0_f64),
        health_state: "Unknown".to_owned(),
        temperature_celsius: (-1.0_f64),
        voltage: (-1.0_f64),
    };
}

// Source: upstream/packages/power/src/power.ts:107 (sha256:31f2f439ed220bd4e26c34219a518b7a055c690cb8ce9b28feb699516c18352f)
#[derive(Clone)]
struct CreatePowerStatusRecord1 {
    __flight_identity: std::sync::Arc<()>,
    battery_level: f64,
    charging_time: f64,
    discharging_time: f64,
    is_battery_low: bool,
    is_charging: bool,
    is_low_power: bool,
    is_on_battery: bool,
    thermal_state: String,
}
impl PartialEq for CreatePowerStatusRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_power_status() -> PowerStatus {
    return PowerStatus {
        __flight_identity: std::sync::Arc::new(()),
        battery_level: (-1.0_f64),
        charging_time: (-1.0_f64),
        discharging_time: (-1.0_f64),
        is_battery_low: false,
        is_charging: false,
        is_low_power: false,
        is_on_battery: false,
        thermal_state: "Unknown".to_owned(),
    };
}

// Source: upstream/packages/power/src/power.ts:122 (sha256:ae048d02da49237e65fb444ca5b6ecad80db05371477c1dacf61c69c5132aac6)
#[derive(Clone)]
struct CreateWebPowerBackendRecord1 {
    __flight_identity: std::sync::Arc<()>,
    wake_lock: Option<WebWakeLock>,
}
impl PartialEq for CreateWebPowerBackendRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_web_power_backend() -> PowerBackend {
    let cached_level: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-1.0_f64)));
    let cached_charging: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let cached_charging_time: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-1.0_f64)));
    let cached_discharging_time: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-1.0_f64)));
    return PowerBackend {
        __flight_identity: std::sync::Arc::new(()),
        get_battery_health: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_out: PowerBatteryHealth| -> Option<PowerBatteryHealth> {
                return None;
            },
        )
            as Box<dyn FnMut(PowerBatteryHealth) -> Option<PowerBatteryHealth> + Send + 'static>)),
        is_keep_awake_active: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> bool {
                return ((*_WAKE_LOCK_SENTINEL.lock().unwrap()).clone()).is_some();
            }) as Box<dyn FnMut() -> bool + Send + 'static>,
        )),
        get_status: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut cached_charging = cached_charging.clone();
            let mut cached_charging_time = cached_charging_time.clone();
            let mut cached_discharging_time = cached_discharging_time.clone();
            let mut cached_level = cached_level.clone();
            move |mut out: PowerStatus| -> PowerStatus {
                let level = (*cached_level.lock().unwrap()).clone();
                let charging = (*cached_charging.lock().unwrap()).clone();
                let charging_time = (*cached_charging_time.lock().unwrap()).clone();
                let discharging_time = (*cached_discharging_time.lock().unwrap()).clone();
                out.battery_level = level;
                out.charging_time = charging_time;
                out.discharging_time = discharging_time;
                out.is_battery_low = (((level >= 0.0_f64) && (level <= 0.2_f64)) && (!charging));
                out.is_charging = charging;
                out.is_on_battery = ((level >= 0.0_f64) && (!charging));
                out.is_low_power = false;
                out.thermal_state = "Unknown".to_owned();
                return (out).clone();
            }
        })
            as Box<dyn FnMut(PowerStatus) -> PowerStatus + Send + 'static>)),
        get_system_idle_state: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_threshold_seconds: f64| -> PowerIdleState {
                return "Unknown".to_owned();
            },
        )
            as Box<dyn FnMut(f64) -> PowerIdleState + Send + 'static>)),
        get_system_idle_time: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> f64 {
            return (-1.0_f64);
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)),
        set_keep_awake: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |enabled: bool, mode: PowerKeepAwakeMode| -> bool {
                let resolved_mode = (mode).unwrap_or("PreventDisplaySleep".to_owned());
                if (resolved_mode == "PreventAppSuspension") {
                    return false;
                }
                return false;
            },
        )
            as Box<dyn FnMut(bool, PowerKeepAwakeMode) -> bool + Send + 'static>)),
        subscribe: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut cached_charging = cached_charging.clone();
            let mut cached_charging_time = cached_charging_time.clone();
            let mut cached_discharging_time = cached_discharging_time.clone();
            let mut cached_level = cached_level.clone();
            move |listener: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>| -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
      let battery = get_web_battery_manager_promise();
      if (battery).is_none() {
        return std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> () {

        }) as Box<dyn FnMut() -> () + Send + 'static>));
      }
      let manager: std::sync::Arc<std::sync::Mutex<Option<WebBatteryManager>>> = std::sync::Arc::new(std::sync::Mutex::new(None));
      let mut on_level_change: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> = std::sync::Arc::new(std::sync::Mutex::new(Box::new({ let mut cached_level = cached_level.clone(); let listener = listener.clone(); let mut manager = manager.clone(); move || -> () {
        if (((*manager.lock().unwrap())).clone()).is_some() {
          (*cached_level.lock().unwrap()) = (*manager.lock().unwrap()).as_ref().unwrap().level;
        }
        ((listener).clone()).lock().unwrap()();
      } }) as Box<dyn FnMut() -> () + Send + 'static>));
      let mut on_charging_change: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> = std::sync::Arc::new(std::sync::Mutex::new(Box::new({ let mut cached_charging = cached_charging.clone(); let listener = listener.clone(); let mut manager = manager.clone(); move || -> () {
        if (((*manager.lock().unwrap())).clone()).is_some() {
          (*cached_charging.lock().unwrap()) = (*manager.lock().unwrap()).as_ref().unwrap().charging;
        }
        ((listener).clone()).lock().unwrap()();
      } }) as Box<dyn FnMut() -> () + Send + 'static>));
      let mut on_charging_time_change: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> = std::sync::Arc::new(std::sync::Mutex::new(Box::new({ let mut cached_charging_time = cached_charging_time.clone(); let listener = listener.clone(); let mut manager = manager.clone(); move || -> () {
        if (((*manager.lock().unwrap())).clone()).is_some() {
          let t = (*manager.lock().unwrap()).as_ref().unwrap().charging_time;
          (*cached_charging_time.lock().unwrap()) = if (t == f64::INFINITY) { (-1.0_f64) } else { t };
        }
        ((listener).clone()).lock().unwrap()();
      } }) as Box<dyn FnMut() -> () + Send + 'static>));
      let mut on_discharging_time_change: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> = std::sync::Arc::new(std::sync::Mutex::new(Box::new({ let mut cached_discharging_time = cached_discharging_time.clone(); let listener = listener.clone(); let mut manager = manager.clone(); move || -> () {
        if (((*manager.lock().unwrap())).clone()).is_some() {
          let t = (*manager.lock().unwrap()).as_ref().unwrap().discharging_time;
          (*cached_discharging_time.lock().unwrap()) = if (t == f64::INFINITY) { (-1.0_f64) } else { t };
        }
        ((listener).clone()).lock().unwrap()();
      } }) as Box<dyn FnMut() -> () + Send + 'static>));
      let cancelled: std::sync::Arc<std::sync::Mutex<bool>> = std::sync::Arc::new(std::sync::Mutex::new(false));
      ((battery.as_ref().unwrap().then)(std::sync::Arc::new(std::sync::Mutex::new(Box::new({ let mut cached_charging = cached_charging.clone(); let mut cached_charging_time = cached_charging_time.clone(); let mut cached_discharging_time = cached_discharging_time.clone(); let mut cached_level = cached_level.clone(); let mut cancelled = cancelled.clone(); let listener = listener.clone(); let mut manager = manager.clone(); let on_charging_change = on_charging_change.clone(); let on_charging_time_change = on_charging_time_change.clone(); let on_discharging_time_change = on_discharging_time_change.clone(); let on_level_change = on_level_change.clone(); move |m: crate::OpaqueHostValue| -> () {
        if ((*cancelled.lock().unwrap())).clone() {
          return;
        }
        (*manager.lock().unwrap()) = Some(m);
        (*cached_level.lock().unwrap()) = crate::host_value::<f64>("host.level");
        (*cached_charging.lock().unwrap()) = crate::host_value::<bool>("host.charging");
        (*cached_charging_time.lock().unwrap()) = if (crate::host_value::<crate::OpaqueHostValue>("host.chargingTime") == f64::INFINITY) { (-1.0_f64) } else { crate::host_value::<f64>("host.chargingTime") };
        (*cached_discharging_time.lock().unwrap()) = if (crate::host_value::<crate::OpaqueHostValue>("host.dischargingTime") == f64::INFINITY) { (-1.0_f64) } else { crate::host_value::<f64>("host.dischargingTime") };
        Some(());
        Some(());
        Some(());
        Some(());
        ((listener).clone()).lock().unwrap()();
      } }) as Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>))).catch)(std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> () {

      }) as Box<dyn FnMut() -> () + Send + 'static>)));
      return std::sync::Arc::new(std::sync::Mutex::new(Box::new({ let mut cancelled = cancelled.clone(); let mut manager = manager.clone(); let on_charging_change = on_charging_change.clone(); let on_charging_time_change = on_charging_time_change.clone(); let on_discharging_time_change = on_discharging_time_change.clone(); let on_level_change = on_level_change.clone(); move || -> () {
        (*cancelled.lock().unwrap()) = true;
        { let __flight_callback = (*manager.lock().unwrap()).as_ref().and_then(|value| (value.remove_event_listener).clone()); __flight_callback.as_ref().map(|callback| callback.lock().unwrap()("levelchange".to_owned(), (on_level_change).clone())) };
        { let __flight_callback = (*manager.lock().unwrap()).as_ref().and_then(|value| (value.remove_event_listener).clone()); __flight_callback.as_ref().map(|callback| callback.lock().unwrap()("chargingchange".to_owned(), (on_charging_change).clone())) };
        { let __flight_callback = (*manager.lock().unwrap()).as_ref().and_then(|value| (value.remove_event_listener).clone()); __flight_callback.as_ref().map(|callback| callback.lock().unwrap()("chargingtimechange".to_owned(), (on_charging_time_change).clone())) };
        { let __flight_callback = (*manager.lock().unwrap()).as_ref().and_then(|value| (value.remove_event_listener).clone()); __flight_callback.as_ref().map(|callback| callback.lock().unwrap()("dischargingtimechange".to_owned(), (on_discharging_time_change).clone())) };
        (*manager.lock().unwrap()) = None;
      } }) as Box<dyn FnMut() -> () + Send + 'static>));
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
        subscribe_lock_screen: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_low_power_mode_change: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_resume: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |listener: std::sync::Arc<
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
        subscribe_suspend: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |listener: std::sync::Arc<
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
        subscribe_thermal_state_change: std::sync::Arc::new(std::sync::Mutex::new(
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
        subscribe_unlock_screen: std::sync::Arc::new(std::sync::Mutex::new(
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

// Source: upstream/packages/power/src/power.ts:279 (sha256:b4043105db5d0d7017defc25373c760546ff2218ac4d77ea35dfd48a67dfbe3c)
pub fn detach_power(power: &Power) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*power).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        ((unsubscribe.as_ref().unwrap()).clone()).lock().unwrap()();
        {
            let __flight_key = (*power).clone();
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

// Source: upstream/packages/power/src/power.ts:289 (sha256:afb70c8946b17b114a01c268694827dcdb598e5000bda8d809e2b68f514f9283)
pub fn dispose_power(power: &Power) -> () {
    detach_power(power);
}

// Source: upstream/packages/power/src/power.ts:295 (sha256:06a52cc42b810d1eed4e7defca99bda482aaac294e96bd61ceee077e00ab16a6)
pub fn enable_power_signals(power: &mut Power) -> () {
    if ((power.on_change).clone()).is_none() {
        power.on_change = Some(create_signal());
    }
    if ((power.on_charging).clone()).is_none() {
        power.on_charging = Some(create_signal());
    }
    if ((power.on_discharging).clone()).is_none() {
        power.on_discharging = Some(create_signal());
    }
    if ((power.on_idle_state_change).clone()).is_none() {
        power.on_idle_state_change = Some(create_signal());
    }
    if ((power.on_lock_screen).clone()).is_none() {
        power.on_lock_screen = Some(create_signal());
    }
    if ((power.on_low_power_mode_change).clone()).is_none() {
        power.on_low_power_mode_change = Some(create_signal());
    }
    if ((power.on_resume).clone()).is_none() {
        power.on_resume = Some(create_signal());
    }
    if ((power.on_suspend).clone()).is_none() {
        power.on_suspend = Some(create_signal());
    }
    if ((power.on_thermal_state_change).clone()).is_none() {
        power.on_thermal_state_change = Some(create_signal());
    }
    if ((power.on_unlock_screen).clone()).is_none() {
        power.on_unlock_screen = Some(create_signal());
    }
}

// Source: upstream/packages/power/src/power.ts:308 (sha256:d1015f5aeb446231303587fec37a485fd2e411d025b3599ceb5cabb1bf15616f)
pub fn get_power_backend() -> PowerBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_power_backend());
    }
    return ((*_BACKEND.lock().unwrap()).clone()).clone().unwrap();
}

// Source: upstream/packages/power/src/power.ts:315 (sha256:9a1bb90215600e65d7d3cd4692a79e85f02a5cea5ece12f4fb283efe5ed05089)
pub fn get_power_battery_health(out: &PowerBatteryHealth) -> Option<PowerBatteryHealth> {
    return ((get_power_backend().get_battery_health).clone())
        .lock()
        .unwrap()((*out).clone());
}

// Source: upstream/packages/power/src/power.ts:320 (sha256:b45683f595f6dcc4420c7c77a4e051c1a96145a4bc31a4a4befff7ecde20157c)
pub fn get_power_idle_polling_interval_ms() -> f64 {
    return (*_IDLE_POLLING_INTERVAL_MS.lock().unwrap()).clone();
}

// Source: upstream/packages/power/src/power.ts:325 (sha256:5db5ab8c90ae438be3bfafb1dca8e85e673f0f8af0b0688e7e3760ffe74f3ab7)
pub fn get_power_status(out: &PowerStatus) -> PowerStatus {
    return ((get_power_backend().get_status).clone()).lock().unwrap()((*out).clone());
}

// Source: upstream/packages/power/src/power.ts:330 (sha256:b44f0bfb598400a60be66f6870bf648c73dadc05a34ff90c32ed404838d15e3b)
pub fn get_power_system_idle_state(threshold_seconds: f64) -> PowerIdleState {
    return ((get_power_backend().get_system_idle_state).clone())
        .lock()
        .unwrap()(threshold_seconds);
}

// Source: upstream/packages/power/src/power.ts:335 (sha256:1b206567c5ee89e95377125d52d01a90469c6d767ad9076aa866de6d6e3becae)
pub fn get_power_system_idle_time() -> f64 {
    return ((get_power_backend().get_system_idle_time).clone())
        .lock()
        .unwrap()();
}

// Source: upstream/packages/power/src/power.ts:340 (sha256:e71dde7ad3269c8395940eef6e669934bb0e21563f767165010d9474921297a0)
pub fn get_power_thermal_state() -> PowerThermalState {
    return (((get_power_backend().get_status).clone()).lock().unwrap()(
        ((*_SCRATCH).clone()).clone(),
    )
    .thermal_state)
        .clone();
}

// Source: upstream/packages/power/src/power.ts:345 (sha256:32aa8dd5b7dc9afcc490b8e222745cb61cf7f63588227897585942afd7d406bd)
pub fn has_power_keep_awake() -> bool {
    return ((get_power_backend().is_keep_awake_active).clone())
        .lock()
        .unwrap()();
}

// Source: upstream/packages/power/src/power.ts:350 (sha256:587f4b0078c26c5ab6d469aab12864b71d8f05b42620846cd2c79a708b7f2b9d)
pub fn set_power_backend(backend: Option<PowerBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/power/src/power.ts:357 (sha256:6b8bda1333db6e0b8e2881eb36e3a93366eb20bd0262c6f894e1c83d7c7cb0f5)
pub fn set_power_idle_polling_interval_ms(interval_ms: f64) -> () {
    (*_IDLE_POLLING_INTERVAL_MS.lock().unwrap()) = interval_ms;
}

// Source: upstream/packages/power/src/power.ts:363 (sha256:37a422e6e8991f130f8360af2d0e003e5eaac365af27458db4f97445224bf1d6)
pub fn set_power_keep_awake(enabled: bool, mode: Option<PowerKeepAwakeMode>) -> bool {
    return ((get_power_backend().set_keep_awake).clone())
        .lock()
        .unwrap()(enabled, (mode).clone().unwrap());
}

// Source: upstream/packages/power/src/power.ts:367 (sha256:6414a3f1532c56810ee95fc29fc8f6c692e8b42d15f6dfb2a5319a4c14a7aa85)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<PowerBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/power/src/power.ts:368 (sha256:6a1647b478309bb4b8954df1d15b40ded9a4b611f190bd0c4539bf50c5e06772)
static _IDLE_POLLING_INTERVAL_MS: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(5000.0_f64));

// Source: upstream/packages/power/src/power.ts:369 (sha256:b6fd3b29c2b6ba26356583797ca7c3338a5a3badd56689a48e3226ca47795010)
static _WAKE_LOCK_SENTINEL: std::sync::LazyLock<std::sync::Mutex<Option<WebWakeLockSentinel>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/power/src/power.ts:370 (sha256:46526f281546e01e1b12cdd9fda6e14c0404a2a0a987471940356efe92e73f92)
static _SCRATCH: std::sync::LazyLock<PowerStatus> =
    std::sync::LazyLock::new(|| create_power_status());

// Source: upstream/packages/power/src/power.ts:371 (sha256:c00e0655a0be8d24a9c370773c38d9ff1209ef1ac58b926e8d2667f7637b0187)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            Power,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/power/src/power.ts:373 (sha256:527de96a12234d71a669e562030b12271121bdde40dd217825e8972cacaa7103)
#[derive(Clone)]
struct WebBatteryManager {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub charging_time: f64,
    pub discharging_time: f64,
    pub level: f64,
    pub charging: bool,
    pub add_event_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub remove_event_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for WebBatteryManager {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/power/src/power.ts:388 (sha256:0e5f710f35624261383b25f3873b61b77f5a8fc0da86765d0464085fe9522a89)
#[derive(Clone)]
struct WebWakeLockSentinel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_event_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub release: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for WebWakeLockSentinel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/power/src/power.ts:393 (sha256:15f58cf66a44b5016d47eba777e2eb4bc7458ffc56a7dff9d976344f17a4ac8c)
#[derive(Clone)]
struct WebWakeLock {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub request: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::Promise<WebWakeLockSentinel> + Send + 'static>,
        >,
    >,
}
impl PartialEq for WebWakeLock {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/power/src/power.ts:397 (sha256:74647174bb3bd72b1673059fcc3c32e364a4c452cc9325005db72ac38480859a)
#[derive(Clone)]
struct GetWebBatteryManagerPromiseRecord1 {
    __flight_identity: std::sync::Arc<()>,
    get_battery: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut() -> crate::Promise<WebBatteryManager> + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for GetWebBatteryManagerPromiseRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn get_web_battery_manager_promise() -> Option<crate::Promise<WebBatteryManager>> {
    return None;
}
