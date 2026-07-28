// @generated from upstream/packages/types/src/Sensors.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Sensors.ts:4 (sha256:0cf7accce78a809f61aff1748412306e8aea94882aed71d3ec66f507a5d009d2)
pub type SensorAccuracy = String;

// Source: upstream/packages/types/src/Sensors.ts:8 (sha256:ad12fb987cd42c73a7cc85d18304e253b02083e21959b51cd6c3fa4031875cd3)
pub type SensorsPermissionState = String;

// Source: upstream/packages/types/src/Sensors.ts:12 (sha256:044a0d9f6a0c93c6f5432e7a36417455ea0eb827d6cf5cee4f59db1e2310e551)
#[derive(Clone)]
pub struct SensorSubscribeOptions {
    pub frequency: Option<f64>,
}

// Source: upstream/packages/types/src/Sensors.ts:18 (sha256:51bd776e4a7a8b10fcbbc699dc76e06d0449bb319ba3fec3622edf9a25ad972d)
#[derive(Clone)]
pub struct SensorReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
}

// Source: upstream/packages/types/src/Sensors.ts:25 (sha256:a0fee4c9a83cb406ba8bf5c31d9c8db5ed796722c31d24065f01bb70c78501b6)
#[derive(Clone)]
pub struct AmbientLightReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub illuminance: f64,
}

// Source: upstream/packages/types/src/Sensors.ts:31 (sha256:d9789d44f99a99d533ddaa60cb9ceacd1d9cde74fa6fb2fa844977259fa2f44d)
#[derive(Clone)]
pub struct MotionReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Source: upstream/packages/types/src/Sensors.ts:38 (sha256:67d8fdfd155627df7d5bb19d20914d7369b80a52ccc97af1013f7ead21399bd2)
#[derive(Clone)]
pub struct OrientationReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub absolute: bool,
    pub heading: f64,
}

// Source: upstream/packages/types/src/Sensors.ts:49 (sha256:5069ef36bc445bb25c7a2257fb9cd9441de5beac960ff089221e17b4d34f22a2)
#[derive(Clone)]
pub struct PressureReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub altitude: f64,
    pub pressure: f64,
}

// Source: upstream/packages/types/src/Sensors.ts:56 (sha256:c3fccbc7fd5d65aa51a854b8d20a5a3fcdae05e9e5fb4bc9b578f8d7d82c0fb1)
#[derive(Clone)]
pub struct ProximityReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub distance: f64,
    pub max: f64,
    pub near: bool,
}

// Source: upstream/packages/types/src/Sensors.ts:63 (sha256:99e9f923970fafdabc49c090481797531b17ac7305f6e25b774fab108fd30893)
#[derive(Clone)]
pub struct QuaternionReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Source: upstream/packages/types/src/Sensors.ts:71 (sha256:1bf3e1d56477d4bcc013cd9c52fb7b529c2a0bb296a02b8de6556d6e217b9e4b)
#[derive(Clone)]
pub struct RotationRateReading {
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

// Source: upstream/packages/types/src/Sensors.ts:83 (sha256:0ccecba8c3adf9819497f609d0f604bac54090f90b8f754ce4b43e6aafe99cc0)
#[derive(Clone)]
pub struct SensorsBackend {
    pub get_permission_state: crate::OpaqueHostValue,
    pub is_ambient_light_supported: crate::OpaqueHostValue,
    pub is_barometer_supported: crate::OpaqueHostValue,
    pub is_gravity_supported: crate::OpaqueHostValue,
    pub is_gyroscope_supported: crate::OpaqueHostValue,
    pub is_linear_acceleration_supported: crate::OpaqueHostValue,
    pub is_magnetometer_supported: crate::OpaqueHostValue,
    pub is_motion_supported: crate::OpaqueHostValue,
    pub is_orientation_supported: crate::OpaqueHostValue,
    pub is_proximity_supported: crate::OpaqueHostValue,
    pub request_permission: crate::OpaqueHostValue,
    pub subscribe_absolute_orientation: crate::OpaqueHostValue,
    pub subscribe_ambient_light: crate::OpaqueHostValue,
    pub subscribe_barometer: crate::OpaqueHostValue,
    pub subscribe_gravity: crate::OpaqueHostValue,
    pub subscribe_linear_acceleration: crate::OpaqueHostValue,
    pub subscribe_magnetometer: crate::OpaqueHostValue,
    pub subscribe_motion: crate::OpaqueHostValue,
    pub subscribe_orientation: crate::OpaqueHostValue,
    pub subscribe_proximity: crate::OpaqueHostValue,
    pub subscribe_quaternion: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Sensors.ts:159 (sha256:28a6a6a08f66e6b2bd46b5abc29cd8e400a5427f4277c2cbaef57a017f21f0c2)
#[derive(Clone)]
pub struct Sensors {
    pub on_absolute_orientation: Signal,
    pub on_accelerometer: Signal,
    pub on_ambient_light: Signal,
    pub on_barometer: Signal,
    pub on_gravity: Signal,
    pub on_gyroscope: Signal,
    pub on_linear_acceleration: Signal,
    pub on_magnetometer: Signal,
    pub on_orientation: Signal,
    pub on_proximity: Signal,
    pub on_quaternion: Signal,
}
