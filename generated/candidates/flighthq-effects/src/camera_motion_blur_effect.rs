// @generated from upstream/packages/effects/src/cameraMotionBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::CameraMotionBlurEffect;

// Source: upstream/packages/effects/src/cameraMotionBlurEffect.ts:3 (sha256:6bf830f726d4b6198235f2ce7c8733825be69a812324745b0cf7118cad872d4e)
#[derive(Clone)]
struct CreateCameraMotionBlurEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateCameraMotionBlurEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_camera_motion_blur_effect(
    options: Option<CameraMotionBlurEffect>,
) -> CameraMotionBlurEffect {
    let options = options.unwrap_or(CameraMotionBlurEffect {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        samples: None,
    });
    return CameraMotionBlurEffect {
        kind: "CameraMotionBlurEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
