// @generated from upstream/packages/node/src/hasTransform2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{HasTransform2D, HasTransform2DRuntime};

// Source: upstream/packages/node/src/hasTransform2d.ts:3 (sha256:ea7bbd38750f6b9f05a70e358da517366e4b2fcf6feaf9867ca3d5fe50e8170e)
pub fn init_transform2_d_runtime_trait(
    target: &mut HasTransform2DRuntime,
    _methods: Option<HasTransform2DRuntime>,
) -> () {
    target.local_matrix = None;
    target.rotation_angle = 0.0_f64;
    target.rotation_cosine = 1.0_f64;
    target.rotation_sine = 0.0_f64;
    target.world_matrix = None;
}

// Source: upstream/packages/node/src/hasTransform2d.ts:14 (sha256:aa37471eb79b63d17314926ca92e1f8466f4f70085b64a9c4ef2ec8b4c27f4c5)
pub fn init_transform2_d_trait(target: &mut HasTransform2D, obj: Option<HasTransform2D>) -> () {
    target.pivot_x = (obj.as_ref().map(|value| value.pivot_x)).unwrap_or(0.0_f64);
    target.pivot_y = (obj.as_ref().map(|value| value.pivot_y)).unwrap_or(0.0_f64);
    target.rotation = (obj.as_ref().map(|value| value.rotation)).unwrap_or(0.0_f64);
    target.scale_x = (obj.as_ref().map(|value| value.scale_x)).unwrap_or(1.0_f64);
    target.scale_y = (obj.as_ref().map(|value| value.scale_y)).unwrap_or(1.0_f64);
    target.skew_x = (obj.as_ref().map(|value| value.skew_x)).unwrap_or(0.0_f64);
    target.skew_y = (obj.as_ref().map(|value| value.skew_y)).unwrap_or(0.0_f64);
    target.x = (obj.as_ref().map(|value| value.x)).unwrap_or(0.0_f64);
    target.y = (obj.as_ref().map(|value| value.y)).unwrap_or(0.0_f64);
}
