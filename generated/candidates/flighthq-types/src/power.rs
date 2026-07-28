// @generated from upstream/packages/types/src/Power.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PowerBatteryHealth, Signal};

// Source: upstream/packages/types/src/Power.ts:5 (sha256:fe755a8c9cfbe6534469e1b2471cad0987fdce6b73d9f48a99c8aa88e8ff8b79)
pub type PowerIdleState = String;

// Source: upstream/packages/types/src/Power.ts:9 (sha256:8a7e78b24a6484da7d88b2eed53ef93edc31b304a6520af7b68321590cee6057)
pub type PowerKeepAwakeMode = String;

// Source: upstream/packages/types/src/Power.ts:12 (sha256:877483b31e28cd19eb79182f4b83890652bee06ded85d06a24ec3a6dc7771b83)
pub type PowerThermalState = String;

// Source: upstream/packages/types/src/Power.ts:14 (sha256:44cf3532b9917b2948b45143545b106cfcaccb118916aab9fa59e3e14c59ba03)
#[derive(Clone)]
pub struct PowerStatus {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub battery_level: f64,
    pub charging_time: f64,
    pub discharging_time: f64,
    pub is_battery_low: bool,
    pub is_charging: bool,
    pub is_low_power: bool,
    pub is_on_battery: bool,
    pub thermal_state: PowerThermalState,
}
impl PartialEq for PowerStatus {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:33 (sha256:601fafa75fa2138d9695563e346b8388a6794302c2964e9e119191f6480e04cd)
#[derive(Clone)]
pub struct PowerBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_battery_health: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(PowerBatteryHealth) -> Option<PowerBatteryHealth> + Send + 'static>,
        >,
    >,
    pub get_status: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(PowerStatus) -> PowerStatus + Send + 'static>>,
    >,
    pub get_system_idle_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> PowerIdleState + Send + 'static>>>,
    pub get_system_idle_time:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub is_keep_awake_active:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub set_keep_awake: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool, PowerKeepAwakeMode) -> bool + Send + 'static>>,
    >,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_lock_screen: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_low_power_mode_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_resume: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_suspend: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_thermal_state_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_unlock_screen: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for PowerBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:65 (sha256:0ce15ac4501682cc3cc8f344a70b5bdea9ffa4d3457eff5b32539b81c89fb28e)
#[derive(Clone)]
pub struct Power {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change: Option<
        Signal<
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PowerStatus) -> () + Send + 'static>>>,
        >,
    >,
    pub on_charging:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_discharging:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_idle_state_change:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_lock_screen:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_low_power_mode_change:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_resume:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_suspend:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_thermal_state_change:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_unlock_screen:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
}
impl PartialEq for Power {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
