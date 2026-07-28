// @generated from upstream/packages/node/src/hasTransform2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{HasTransform2D, HasTransform2DRuntime, Matrix};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotation: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/node/src/hasTransform2d.ts:3 (sha256:ea7bbd38750f6b9f05a70e358da517366e4b2fcf6feaf9867ca3d5fe50e8170e)
pub fn init_transform2_d_runtime_trait(
    target: &mut HasTransform2DRuntime,
    _methods: Option<FlightPartialRecord1>,
) -> () {
    target.local_matrix = None;
    target.rotation_angle = 0.0_f64;
    target.rotation_cosine = 1.0_f64;
    target.rotation_sine = 0.0_f64;
    target.world_matrix = None;
}

// Source: upstream/packages/node/src/hasTransform2d.ts:14 (sha256:aa37471eb79b63d17314926ca92e1f8466f4f70085b64a9c4ef2ec8b4c27f4c5)
pub fn init_transform2_d_trait(
    target: &mut HasTransform2D,
    obj: Option<FlightPartialRecord2>,
) -> () {
    target.pivot_x = (obj.as_ref().and_then(|value| value.pivot_x)).unwrap_or(0.0_f64);
    target.pivot_y = (obj.as_ref().and_then(|value| value.pivot_y)).unwrap_or(0.0_f64);
    target.rotation = (obj.as_ref().and_then(|value| value.rotation)).unwrap_or(0.0_f64);
    target.scale_x = (obj.as_ref().and_then(|value| value.scale_x)).unwrap_or(1.0_f64);
    target.scale_y = (obj.as_ref().and_then(|value| value.scale_y)).unwrap_or(1.0_f64);
    target.skew_x = (obj.as_ref().and_then(|value| value.skew_x)).unwrap_or(0.0_f64);
    target.skew_y = (obj.as_ref().and_then(|value| value.skew_y)).unwrap_or(0.0_f64);
    target.x = (obj.as_ref().and_then(|value| value.x)).unwrap_or(0.0_f64);
    target.y = (obj.as_ref().and_then(|value| value.y)).unwrap_or(0.0_f64);
}
