// @generated from upstream/packages/sensors/src/sensors.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_math::{DEG_TO_RAD as deg_to_rad_constant, RAD_TO_DEG as rad_to_deg_constant};
use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    AmbientLightReading, EntityConstruction, HostSensorsCapability, MotionReading,
    OrientationReading, PressureReading, ProximityReading, QuaternionReading, RotationRateReading,
    Sensors, SensorsPermissionState,
};

// Source: upstream/packages/sensors/src/sensors.ts:23 (sha256:86ff1a1d4a5e50aedacfeece43322a2f38f59db7935486a9c4b3dc43af15946d)
pub fn attach_sensors(host_sensors: &HostSensorsCapability, sensors: Sensors) -> () {
    detach_sensors(&sensors);
    let unsubscribe_motion = {
        let __flight_callback = (host_sensors.subscribe_motion).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |acceleration: MotionReading, rotation_rate: RotationRateReading| -> () {
                    emit_signal(
                        (sensors.on_accelerometer).clone(),
                        ((acceleration).clone(),),
                    );
                    emit_signal((sensors.on_gyroscope).clone(), ((rotation_rate).clone(),));
                }
            })
                as Box<dyn FnMut(MotionReading, RotationRateReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_linear_acceleration = {
        let __flight_callback = (host_sensors.subscribe_linear_acceleration).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |reading: MotionReading| -> () {
                    emit_signal(
                        (sensors.on_linear_acceleration).clone(),
                        ((reading).clone(),),
                    );
                }
            })
                as Box<dyn FnMut(MotionReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_gravity = {
        let __flight_callback = (host_sensors.subscribe_gravity).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |reading: MotionReading| -> () {
                    emit_signal((sensors.on_gravity).clone(), ((reading).clone(),));
                }
            })
                as Box<dyn FnMut(MotionReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_orientation = {
        let __flight_callback = (host_sensors.subscribe_orientation).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |orientation: OrientationReading| -> () {
                    emit_signal((sensors.on_orientation).clone(), ((orientation).clone(),));
                }
            })
                as Box<dyn FnMut(OrientationReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_absolute_orientation = {
        let __flight_callback = (host_sensors.subscribe_absolute_orientation).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |orientation: OrientationReading| -> () {
                    emit_signal(
                        (sensors.on_absolute_orientation).clone(),
                        ((orientation).clone(),),
                    );
                }
            })
                as Box<dyn FnMut(OrientationReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_magnetometer = {
        let __flight_callback = (host_sensors.subscribe_magnetometer).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |reading: MotionReading| -> () {
                    emit_signal((sensors.on_magnetometer).clone(), ((reading).clone(),));
                }
            })
                as Box<dyn FnMut(MotionReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_ambient_light = {
        let __flight_callback = (host_sensors.subscribe_ambient_light).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |reading: AmbientLightReading| -> () {
                    emit_signal((sensors.on_ambient_light).clone(), ((reading).clone(),));
                }
            })
                as Box<dyn FnMut(AmbientLightReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_barometer = {
        let __flight_callback = (host_sensors.subscribe_barometer).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |reading: PressureReading| -> () {
                    emit_signal((sensors.on_barometer).clone(), ((reading).clone(),));
                }
            })
                as Box<dyn FnMut(PressureReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_proximity = {
        let __flight_callback = (host_sensors.subscribe_proximity).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |reading: ProximityReading| -> () {
                    emit_signal((sensors.on_proximity).clone(), ((reading).clone(),));
                }
            })
                as Box<dyn FnMut(ProximityReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    let unsubscribe_quaternion = {
        let __flight_callback = (host_sensors.subscribe_quaternion).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let sensors = sensors.clone();
                move |reading: QuaternionReading| -> () {
                    emit_signal((sensors.on_quaternion).clone(), ((reading).clone(),));
                }
            })
                as Box<dyn FnMut(QuaternionReading) -> () + Send + 'static>),
        ));
        __flight_result
    };
    {
        let __flight_key = (sensors).clone();
        let __flight_value = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let unsubscribe_absolute_orientation = unsubscribe_absolute_orientation.clone();
            let unsubscribe_ambient_light = unsubscribe_ambient_light.clone();
            let unsubscribe_barometer = unsubscribe_barometer.clone();
            let unsubscribe_gravity = unsubscribe_gravity.clone();
            let unsubscribe_linear_acceleration = unsubscribe_linear_acceleration.clone();
            let unsubscribe_magnetometer = unsubscribe_magnetometer.clone();
            let unsubscribe_motion = unsubscribe_motion.clone();
            let unsubscribe_orientation = unsubscribe_orientation.clone();
            let unsubscribe_proximity = unsubscribe_proximity.clone();
            let unsubscribe_quaternion = unsubscribe_quaternion.clone();
            move || -> () {
                {
                    let __flight_callback = (unsubscribe_absolute_orientation).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_ambient_light).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_barometer).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_gravity).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_linear_acceleration).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_magnetometer).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_motion).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_orientation).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_proximity).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                {
                    let __flight_callback = (unsubscribe_quaternion).clone();
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

// Source: upstream/packages/sensors/src/sensors.ts:77 (sha256:49cf4ec5afd690a426501dbe0054b2f1377b2f75b5baa6ec04cd7bd737bdbe14)
pub fn compute_euler_from_quaternion(
    out: &mut OrientationReading,
    quaternion: &QuaternionReading,
) -> () {
    let x = quaternion.x;
    let y = quaternion.y;
    let z = quaternion.z;
    let w = quaternion.w;
    let sin_beta = (2.0_f64 * ((w * x) - (y * z)));
    let beta = if ((sin_beta).abs() >= 1.0_f64) {
        (({
            let __flight_value = sin_beta;
            if __flight_value.is_nan() || __flight_value == 0.0 {
                __flight_value
            } else {
                __flight_value.signum()
            }
        } * std::f64::consts::PI)
            / 2.0_f64)
    } else {
        (sin_beta).asin()
    };
    let alpha = (2.0_f64 * ((w * z) + (x * y))).atan2((1.0_f64 - (2.0_f64 * ((x * x) + (z * z)))));
    let gamma = (2.0_f64 * ((w * y) + (x * z))).atan2((1.0_f64 - (2.0_f64 * ((x * x) + (y * y)))));
    out.alpha = ((((alpha * rad_to_deg_constant) % 360.0_f64) + 360.0_f64) % 360.0_f64);
    out.beta = (beta * rad_to_deg_constant);
    out.gamma = (gamma * rad_to_deg_constant);
    out.interval = quaternion.interval;
    out.timestamp = quaternion.timestamp;
    out.accuracy = (quaternion.accuracy).clone();
}

// Source: upstream/packages/sensors/src/sensors.ts:99 (sha256:fc4f6f7c7ebaac5c8adc9d83d1fef7c37f859f7cf4b8944acfb11f3a53f9ee17)
pub fn compute_gravity_from_orientation(
    out: &mut MotionReading,
    orientation: &OrientationReading,
) -> () {
    let b = (orientation.beta * deg_to_rad_constant);
    let g = (orientation.gamma * deg_to_rad_constant);
    let g = 9.80665_f64;
    let sin_g = (g).sin();
    let cos_g = (g).cos();
    let sin_b = (b).sin();
    let cos_b = (b).cos();
    out.x = ((g * cos_b) * sin_g);
    out.y = ((-g) * sin_b);
    out.z = ((g * cos_b) * cos_g);
    out.interval = orientation.interval;
    out.timestamp = orientation.timestamp;
    out.accuracy = (orientation.accuracy).clone();
}

// Source: upstream/packages/sensors/src/sensors.ts:119 (sha256:a32264d89e52f0b9cc0d15e3868edb89e99ea96f6602dd9960aba8316d3c7ced)
pub fn compute_quaternion_from_orientation_reading(
    out: &mut QuaternionReading,
    orientation: &OrientationReading,
) -> () {
    let a = ((orientation.alpha * deg_to_rad_constant) * 0.5_f64);
    let b = ((orientation.beta * deg_to_rad_constant) * 0.5_f64);
    let g = ((orientation.gamma * deg_to_rad_constant) * 0.5_f64);
    let ca = (a).cos();
    let sa = (a).sin();
    let cb = (b).cos();
    let sb = (b).sin();
    let cg = (g).cos();
    let sg = (g).sin();
    out.x = (((sa * sb) * cg) - ((ca * cb) * sg));
    out.y = (((sa * cb) * sg) + ((ca * sb) * cg));
    out.z = (((ca * cb) * sg) - ((sa * sb) * cg));
    out.w = (((ca * cb) * cg) + ((sa * sb) * sg));
    out.interval = orientation.interval;
    out.timestamp = orientation.timestamp;
    out.accuracy = (orientation.accuracy).clone();
}

// Source: upstream/packages/sensors/src/sensors.ts:143 (sha256:f07d33ee33dc8e3c3c0d0589bd2faa3a59b509d7a1cdfde7f15bdba4402737e5)
pub fn compute_rotation_matrix_from_quaternion(
    out: &mut Vec<f64>,
    quaternion: &QuaternionReading,
) -> () {
    let x = quaternion.x;
    let y = quaternion.y;
    let z = quaternion.z;
    let w = quaternion.w;
    let x2 = (x + x);
    let y2 = (y + y);
    let z2 = (z + z);
    let xx = (x * x2);
    let xy = (x * y2);
    let xz = (x * z2);
    let yy = (y * y2);
    let yz = (y * z2);
    let zz = (z * z2);
    let wx = (w * x2);
    let wy = (w * y2);
    let wz = (w * z2);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (1.0_f64 - (yy + zz));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (xy + wz);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (xz - wy);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = (xy - wz);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (4.0_f64) as usize;
        let __flight_value = (1.0_f64 - (xx + zz));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (5.0_f64) as usize;
        let __flight_value = (yz + wx);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (6.0_f64) as usize;
        let __flight_value = (xz + wy);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (7.0_f64) as usize;
        let __flight_value = (yz - wx);
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (8.0_f64) as usize;
        let __flight_value = (1.0_f64 - (xx + yy));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:175 (sha256:452bf3fdc18801043aca8e35383330ed970f0e95369301f44086ccaa138d9cb0)
pub fn compute_screen_relative_orientation(
    out: &mut OrientationReading,
    orientation: &OrientationReading,
    screen_angle: f64,
) -> () {
    let alpha = orientation.alpha;
    let beta = orientation.beta;
    let gamma = orientation.gamma;
    let angle = (screen_angle * deg_to_rad_constant);
    let sin_a = (angle).sin();
    let cos_a = (angle).cos();
    out.alpha = alpha;
    out.beta = ((beta * cos_a) - (gamma * sin_a));
    out.gamma = ((beta * sin_a) + (gamma * cos_a));
    out.absolute = orientation.absolute;
    out.heading = orientation.heading;
    out.interval = orientation.interval;
    out.timestamp = orientation.timestamp;
    out.accuracy = (orientation.accuracy).clone();
}

// Source: upstream/packages/sensors/src/sensors.ts:202 (sha256:2ab7ecef5055f8ab0e24f03779d33ed2da33e08a45897eb1b2d7528c9d7c7868)
pub fn compute_world_acceleration_from_device_acceleration(
    out: &mut MotionReading,
    acceleration: &MotionReading,
    quaternion: &QuaternionReading,
) -> () {
    let ax = acceleration.x;
    let ay = acceleration.y;
    let az = acceleration.z;
    let qx = quaternion.x;
    let qy = quaternion.y;
    let qz = quaternion.z;
    let qw = quaternion.w;
    let twx = (2.0_f64 * qw);
    let cx = ((qy * az) - (qz * ay));
    let cy = ((qz * ax) - (qx * az));
    let cz = ((qx * ay) - (qy * ax));
    let ccx = ((qy * cz) - (qz * cy));
    let ccy = ((qz * cx) - (qx * cz));
    let ccz = ((qx * cy) - (qy * cx));
    out.x = ((ax + (twx * cx)) + (2.0_f64 * ccx));
    out.y = ((ay + (twx * cy)) + (2.0_f64 * ccy));
    out.z = ((az + (twx * cz)) + (2.0_f64 * ccz));
    out.interval = acceleration.interval;
    out.timestamp = acceleration.timestamp;
    out.accuracy = (acceleration.accuracy).clone();
}

// Source: upstream/packages/sensors/src/sensors.ts:232 (sha256:47bcebaf0858097a61237ec0cac1a06d97c62385218446156e4c687e8298e7ca)
pub fn create_ambient_light_reading() -> AmbientLightReading {
    let mut out = allocate_entity();
    initialize_ambient_light_reading((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:238 (sha256:1609b343b4f2a8238bee08031718ca07974deeedcc222929ddbf19172336fca5)
pub fn create_motion_reading() -> MotionReading {
    let mut out = allocate_entity();
    initialize_motion_reading((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:244 (sha256:7ccc4027710a22f24cb8e52edc913c9583c092c32a55cd9425e64870456c6ce9)
pub fn create_orientation_reading() -> OrientationReading {
    let mut out = allocate_entity();
    initialize_orientation_reading((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:250 (sha256:04e5b485eccc4c73c17a9db60611391f9ebb83bc422f226f84091b16fd9c49ae)
pub fn create_pressure_reading() -> PressureReading {
    let mut out = allocate_entity();
    initialize_pressure_reading((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:256 (sha256:cee2c127139538997ded376e9dd197da73f68fe1b4b5314d1852dc2bcd848333)
pub fn create_proximity_reading() -> ProximityReading {
    let mut out = allocate_entity();
    initialize_proximity_reading((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:262 (sha256:7881c1901120b0d920cf07d1fd3c197e91078f97f146866022106559af09a677)
pub fn create_quaternion_reading() -> QuaternionReading {
    let mut out = allocate_entity();
    initialize_quaternion_reading((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:268 (sha256:5a88c865f668217423e8d53784c4c65b5ddf42485b43f77d91aa619102dcb600)
pub fn create_rotation_rate_reading() -> RotationRateReading {
    let mut out = allocate_entity();
    initialize_rotation_rate_reading((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:274 (sha256:54e9d7669fd7fcbd13911fe6a89deb1f07c15254ede1d1a906eda67d8210063f)
pub fn create_sensors() -> Sensors {
    let mut out = allocate_entity();
    initialize_sensors((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/sensors/src/sensors.ts:281 (sha256:2b40dd3e7466c92e02cafbcf87d85275e6f0793fa160cdc22d1bfe7fc870049c)
pub fn detach_sensors(sensors: &Sensors) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*sensors).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        {
            let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        {
            let __flight_key = (*sensors).clone();
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

// Source: upstream/packages/sensors/src/sensors.ts:291 (sha256:1e72a10bf34bde10d60974fa575463f1318a0d74ff6645cb826d1baf5850b19e)
pub fn dispose_sensors(sensors: &Sensors) -> () {
    detach_sensors(sensors);
}

// Source: upstream/packages/sensors/src/sensors.ts:297 (sha256:108467012e02d7e65d0e4376e44122de9c46d9447705bdc80a8360043f1db11e)
pub fn get_sensors_permission_state(
    host_sensors: &HostSensorsCapability,
    sensor: Option<String>,
) -> crate::FlightTask<SensorsPermissionState> {
    return {
        let __flight_callback = (host_sensors.get_permission_state).clone();
        let __flight_result = __flight_callback.lock().unwrap()((sensor).clone());
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:305 (sha256:a3ec06f93d14eefaac54b34d25e8b8c1e9788741dd9f69d24cd5f6cc92b0dc17)
pub fn has_accelerometer(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_motion_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:310 (sha256:a2e816bba4a3b559564dcb32494c16a5c8d05516b9a1b11eb75fdab1696a6caf)
pub fn has_ambient_light_sensor(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_ambient_light_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:315 (sha256:da84d0c43e52d7703505577336891bbca4c87dd7cd2c3bb634e42e7e13fb26bb)
pub fn has_barometer(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_barometer_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:320 (sha256:a4a15c494acc8657b0ff462a3a572e9b01460b49b6c2f56b7b02e8fb25597e84)
pub fn has_gravity_sensor(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_gravity_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:325 (sha256:a56c01bb273b002b6f35ecf69f860de121a1aea983dfa1076054544d156d27be)
pub fn has_gyroscope(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_gyroscope_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:330 (sha256:48c0518a9c19fbe05d9b9fbcefcdb61d03f5b0460165297e27d3f93c9078aac7)
pub fn has_linear_acceleration_sensor(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_linear_acceleration_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:335 (sha256:0e1dd1ceb2f0cc7521a043765768aad8ec9aba74312adaaaad1dc38d1189f193)
pub fn has_magnetometer(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_magnetometer_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:340 (sha256:485e8fe39b6b7391ac6cd8c5532010e0d6dba579257b6f673de5c572becf9a9a)
pub fn has_orientation_sensor(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_orientation_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:345 (sha256:c7de90790457b5af3fe025c4698acb562ea13cf8d6614b6bcc1d335f98dfbe0c)
pub fn has_proximity_sensor(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_proximity_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:350 (sha256:2a7e8136f3ab6e5d97ccadc3acd96c7ba704d828701bc8c9a484770bfd032fda)
pub fn initialize_ambient_light_reading(out: EntityConstruction<AmbientLightReading>) -> () {
    crate::host_set("host.accuracy", "unknown");
    crate::host_set("host.illuminance", 0.0_f64);
    crate::host_set("host.interval", (-1.0_f64));
    crate::host_set("host.timestamp", (-1.0_f64));
}

// Source: upstream/packages/sensors/src/sensors.ts:359 (sha256:3b7a4edbb6b6b3e42ebc449e51189628b5cce07e9e13d61796fd8f620d3f46fe)
pub fn initialize_motion_reading(out: EntityConstruction<MotionReading>) -> () {
    crate::host_set("host.accuracy", "unknown");
    crate::host_set("host.interval", (-1.0_f64));
    crate::host_set("host.timestamp", (-1.0_f64));
    crate::host_set("host.x", 0.0_f64);
    crate::host_set("host.y", 0.0_f64);
    crate::host_set("host.z", 0.0_f64);
}

// Source: upstream/packages/sensors/src/sensors.ts:370 (sha256:fca98dde8bd60d84f527b1e5ce6ff2037842ca842034f8f7be25d007861813bb)
pub fn initialize_orientation_reading(out: EntityConstruction<OrientationReading>) -> () {
    crate::host_set("host.absolute", false);
    crate::host_set("host.accuracy", "unknown");
    crate::host_set("host.alpha", 0.0_f64);
    crate::host_set("host.beta", 0.0_f64);
    crate::host_set("host.gamma", 0.0_f64);
    crate::host_set("host.heading", (-1.0_f64));
    crate::host_set("host.interval", (-1.0_f64));
    crate::host_set("host.timestamp", (-1.0_f64));
}

// Source: upstream/packages/sensors/src/sensors.ts:383 (sha256:107d0d67484f2156ad139368529239dcee6802c53482b8905d8accf399b6480f)
pub fn initialize_pressure_reading(out: EntityConstruction<PressureReading>) -> () {
    crate::host_set("host.accuracy", "unknown");
    crate::host_set("host.altitude", (-1.0_f64));
    crate::host_set("host.interval", (-1.0_f64));
    crate::host_set("host.pressure", 0.0_f64);
    crate::host_set("host.timestamp", (-1.0_f64));
}

// Source: upstream/packages/sensors/src/sensors.ts:393 (sha256:fc554739eb1e5fa8d102714ae54c021c49ce326230dd4a7ea05a91903a9312a9)
pub fn initialize_proximity_reading(out: EntityConstruction<ProximityReading>) -> () {
    crate::host_set("host.accuracy", "unknown");
    crate::host_set("host.distance", (-1.0_f64));
    crate::host_set("host.interval", (-1.0_f64));
    crate::host_set("host.max", (-1.0_f64));
    crate::host_set("host.near", false);
    crate::host_set("host.timestamp", (-1.0_f64));
}

// Source: upstream/packages/sensors/src/sensors.ts:403 (sha256:24b29b0f1f0a7985e06611422c22dc9784a1acd86f2571c2c7ca3d71253ec69b)
pub fn initialize_quaternion_reading(out: EntityConstruction<QuaternionReading>) -> () {
    crate::host_set("host.accuracy", "unknown");
    crate::host_set("host.interval", (-1.0_f64));
    crate::host_set("host.timestamp", (-1.0_f64));
    crate::host_set("host.w", 1.0_f64);
    crate::host_set("host.x", 0.0_f64);
    crate::host_set("host.y", 0.0_f64);
    crate::host_set("host.z", 0.0_f64);
}

// Source: upstream/packages/sensors/src/sensors.ts:415 (sha256:2d9b7e36fe7cc08a2cbcb245254424d77dbc1511e61df153ada04c7ab2d1200e)
pub fn initialize_rotation_rate_reading(out: EntityConstruction<RotationRateReading>) -> () {
    crate::host_set("host.accuracy", "unknown");
    crate::host_set("host.alpha", 0.0_f64);
    crate::host_set("host.beta", 0.0_f64);
    crate::host_set("host.gamma", 0.0_f64);
    crate::host_set("host.interval", (-1.0_f64));
    crate::host_set("host.timestamp", (-1.0_f64));
}

// Source: upstream/packages/sensors/src/sensors.ts:425 (sha256:e1850d88c69750083d8210b3b9745148c5b058c3926e2e15ae5151950c6a8ca3)
pub fn initialize_sensors(out: EntityConstruction<Sensors>) -> () {
    crate::host_set("host.onAbsoluteOrientation", create_signal());
    crate::host_set("host.onAccelerometer", create_signal());
    crate::host_set("host.onAmbientLight", create_signal());
    crate::host_set("host.onBarometer", create_signal());
    crate::host_set("host.onGravity", create_signal());
    crate::host_set("host.onGyroscope", create_signal());
    crate::host_set("host.onLinearAcceleration", create_signal());
    crate::host_set("host.onMagnetometer", create_signal());
    crate::host_set("host.onOrientation", create_signal());
    crate::host_set("host.onProximity", create_signal());
    crate::host_set("host.onQuaternion", create_signal());
}

// Source: upstream/packages/sensors/src/sensors.ts:440 (sha256:a9154aeab312c6ab82928b279ada79621cbfc92ca47e8832e2eaaff7ca7e4908)
pub fn is_sensors_supported(host_sensors: &HostSensorsCapability) -> bool {
    return {
        let __flight_callback = (host_sensors.is_motion_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:445 (sha256:236eafb01911cb9fd60d1c5c053677365151a57799c81d4cac3c13399c323292)
pub fn request_sensors_permission(host_sensors: &HostSensorsCapability) -> crate::FlightTask<bool> {
    return {
        let __flight_callback = (host_sensors.request_permission).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/sensors/src/sensors.ts:449 (sha256:384048b9eb760879aa7cd30da51dd1c625fc8ab065a0ee5e168c8682b22c2916)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            Sensors,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
