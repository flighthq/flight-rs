// @generated from upstream/packages/node/src/hasTransform3d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{create_quaternion, create_vector3};
pub use flighthq_types::{HasTransform3D, HasTransform3DRuntime};

// Source: upstream/packages/node/src/hasTransform3d.ts:6 (sha256:19a90e49d8328bdb6522d2f46fe13be639882b6ca0d64d5fc52879e708ef519b)
pub fn init_transform3_d_runtime_trait(target: &mut HasTransform3DRuntime) -> () {
    target.local_matrix4 = None;
    target.local_matrix4_detached = false;
    target.world_matrix4 = None;
}

// Source: upstream/packages/node/src/hasTransform3d.ts:12 (sha256:6f88394f57fb4ec88319424b0a9f314dcebfbec88b83c6712003263a9243c043)
pub fn init_transform3_d_trait(target: &mut HasTransform3D, obj: Option<HasTransform3D>) -> () {
    target.rotation = (obj.as_ref().map(|value| (value.rotation).clone()))
        .unwrap_or(create_quaternion(None, None, None, None));
    target.scale = (obj.as_ref().map(|value| (value.scale).clone())).unwrap_or(create_vector3(
        Some(1.0_f64),
        Some(1.0_f64),
        Some(1.0_f64),
    ));
    target.position = (obj.as_ref().map(|value| (value.position).clone()))
        .unwrap_or(create_vector3(None, None, None));
}
