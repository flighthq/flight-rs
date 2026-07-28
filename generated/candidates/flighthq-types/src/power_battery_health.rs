// @generated from upstream/packages/types/src/PowerBatteryHealth.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PowerBatteryHealth.ts:4 (sha256:2b1e7228e14fe09951fbae6a4dd37ecba0f001ce64c98e212657861bcac07409)
#[derive(Clone)]
pub struct PowerBatteryHealth {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub capacity_wear_level: f64,
    pub cycle_count: f64,
    pub health_state: PowerBatteryHealthState,
    pub temperature_celsius: f64,
    pub voltage: f64,
}
impl PartialEq for PowerBatteryHealth {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PowerBatteryHealth.ts:16 (sha256:4ae12c556040e12fa427110e48f61b5760f6288645876a674e74076ea55412e6)
pub type PowerBatteryHealthState = String;
