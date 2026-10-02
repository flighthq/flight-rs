// @generated from upstream/packages/types/src/Power.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, HostPowerCapabilities, PowerBatteryHealth, Signal};

// Source: upstream/packages/types/src/Power.ts:7 (sha256:fe755a8c9cfbe6534469e1b2471cad0987fdce6b73d9f48a99c8aa88e8ff8b79)
pub type PowerIdleState = String;

// Source: upstream/packages/types/src/Power.ts:11 (sha256:8a7e78b24a6484da7d88b2eed53ef93edc31b304a6520af7b68321590cee6057)
pub type PowerKeepAwakeMode = String;

// Source: upstream/packages/types/src/Power.ts:14 (sha256:877483b31e28cd19eb79182f4b83890652bee06ded85d06a24ec3a6dc7771b83)
pub type PowerThermalState = String;

// Source: upstream/packages/types/src/Power.ts:16 (sha256:44cf3532b9917b2948b45143545b106cfcaccb118916aab9fa59e3e14c59ba03)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/Power.ts:44 (sha256:0fd380a208b1aa80bcb33bb77ba23f08ec8cb8385f095a7fea64cc0e1e3fe17b)
pub type PowerKeepAwakeAcquireReason = String;

// Source: upstream/packages/types/src/Power.ts:46 (sha256:a5b7dcfc007edd49ff654fb3c8e2ff8d9efa40f58f9078cd7970119564ba3f5e)
pub type PowerKeepAwakeReleaseReason = String;

// Source: upstream/packages/types/src/Power.ts:48 (sha256:f54671c1cf7ae50087167839cc12bf5bcb0f019497755c7ba1b68847732ee81d)
#[derive(Clone, Default)]
pub struct PowerKeepAwakeAcquireResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: PowerKeepAwakeAcquireReason,
}
impl PartialEq for PowerKeepAwakeAcquireResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:52 (sha256:77c1934a173bb77fa64b8be765a926f5217b1d325a34c8b57f9a983a7cdf5044)
#[derive(Clone, Default)]
pub struct PowerKeepAwakeReleaseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: PowerKeepAwakeReleaseReason,
}
impl PartialEq for PowerKeepAwakeReleaseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:68 (sha256:6c6932939f9395b8bd5bf23a3912bd781505e0325f32d1a3e1e7897a6e4ed88c)
#[derive(Clone)]
pub struct HostPowerStatusCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_status: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(PowerStatus) -> PowerStatus + Send + 'static>>,
    >,
}
impl PartialEq for HostPowerStatusCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:74 (sha256:24ea6ec83955b9e7fcb9fc8cbbe04c8a5ee0e98247035d704dbe335df22e0c2a)
#[derive(Clone)]
pub struct HostPowerChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
}
impl PartialEq for HostPowerChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:81 (sha256:bc53b485c3543285e66bdb156d189c44c0cb267244fe8bc25be06b3f6a95273c)
#[derive(Clone)]
pub struct HostPowerKeepAwakeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(PowerKeepAwakeMode) -> crate::FlightTask<PowerKeepAwakeAcquireResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub destroy: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub is_active: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub release: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<PowerKeepAwakeReleaseResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostPowerKeepAwakeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:90 (sha256:decc6cd96aa5308c51445da026f2cf9416ad4ffdd9976f5406b4bc3e99de0227)
#[derive(Clone)]
pub struct HostPowerIdleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_idle_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> PowerIdleState + Send + 'static>>>,
    pub get_idle_time_seconds:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
}
impl PartialEq for HostPowerIdleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:98 (sha256:46d0b90337a90d67f9cefb6905ff2b510bac5916fbcd89a53dab31dee8d2ff4a)
#[derive(Clone)]
pub struct HostPowerSessionLockCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe_lock: std::sync::Arc<
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
    pub subscribe_unlock: std::sync::Arc<
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
impl PartialEq for HostPowerSessionLockCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:105 (sha256:5979b61502c1b692280d0909dac9313f3ce129421304d49998e1ec888a5d0eb7)
#[derive(Clone)]
pub struct HostPowerSuspensionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
}
impl PartialEq for HostPowerSuspensionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:111 (sha256:ef7e9bdfa1b25de118aea95a50dc582f4fbda7a32031d523a09bf776e435423f)
#[derive(Clone)]
pub struct HostPowerBatteryHealthCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_battery_health: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(PowerBatteryHealth) -> PowerBatteryHealth + Send + 'static>>,
    >,
}
impl PartialEq for HostPowerBatteryHealthCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:118 (sha256:120fbef092a9c6f130a26b3ab5352599030b73ff46bc2fc3a463f15b960b610f)
#[derive(Clone)]
pub struct HostPowerThermalCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_thermal_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> PowerThermalState + Send + 'static>>>,
    pub subscribe_thermal_state_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(PowerThermalState) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostPowerThermalCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:123 (sha256:dc0d5a59f0366c0b34baac0bfce42f794763a66428504cce195f2be248464b77)
#[derive(Clone, Default)]
pub struct ElectronPowerCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub battery_health: Option<HostPowerBatteryHealthCapability>,
    pub change: Option<HostPowerChangeCapability>,
    pub idle: Option<HostPowerIdleCapability>,
    pub keep_awake: Option<HostPowerKeepAwakeCapability>,
    pub session_lock: Option<HostPowerSessionLockCapability>,
    pub status: Option<HostPowerStatusCapability>,
    pub suspension: Option<HostPowerSuspensionCapability>,
    pub thermal: Option<HostPowerThermalCapability>,
}
impl PartialEq for ElectronPowerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Power.ts:131 (sha256:36260c02eb4af9962d08c148cadcde0f945e1c60b9ce10d1fff0825a9be0c789)
pub type WebPowerReadingCapabilities = HostPowerCapabilities;

// Source: upstream/packages/types/src/Power.ts:133 (sha256:cc29c67713b480bdc4ba18ffa87ee32edde44e796d4799b118801421ea2ea892)
pub type WebPowerCapabilities = HostPowerCapabilities;

// Source: upstream/packages/types/src/Power.ts:146 (sha256:f07fb496b857fc664f3b9becd6ac2f721d22c9984ea8439bae4098d63355bce8)
#[derive(Clone, Default)]
pub struct Power {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
    pub on_resume:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_suspend:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
    pub on_thermal_state_change: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<Box<dyn FnMut(PowerThermalState) -> () + Send + 'static>>,
            >,
        >,
    >,
    pub on_unlock_screen:
        Option<Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>>,
}
impl PartialEq for Power {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Power {
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
