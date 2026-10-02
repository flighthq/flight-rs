// @generated from upstream/packages/sensors/src/index.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    attach_sensors, compute_euler_from_quaternion, compute_gravity_from_orientation,
    compute_quaternion_from_orientation_reading, compute_rotation_matrix_from_quaternion,
    compute_screen_relative_orientation, compute_world_acceleration_from_device_acceleration,
    create_sensors, detach_sensors, dispose_sensors, get_sensors_permission_state,
    has_accelerometer, has_ambient_light_sensor, has_barometer, has_gravity_sensor, has_gyroscope,
    has_linear_acceleration_sensor, has_magnetometer, has_orientation_sensor, has_proximity_sensor,
    is_sensors_supported, request_sensors_permission,
};
