// @generated from upstream/packages/camera/src/enableCameraGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{explain_camera3_d_view, set_camera2_d_visible_bounds_guard, set_camera3_d_view_guard};
use flighthq_log::log_once;
use flighthq_types::{Camera2D, Camera3D, LogData, LogDataProvider, LogLevel};

// Source: upstream/packages/camera/src/enableCameraGuards.ts:9 (sha256:57ae37aebf06c70c5ae881001bf53f8a85e3efd14a05af6106c7748b34b004ba)
pub fn are_camera_guards_enabled() -> bool {
    return CAMERA_GUARDS_ENABLED.load(std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/camera/src/enableCameraGuards.ts:13 (sha256:feada37ff56d6855585e3e7c2f8044b4f805168e0083990c99322d1dae5ea35f)
pub fn disable_camera_guards() -> () {
    set_camera3_d_view_guard(&(None));
    set_camera2_d_visible_bounds_guard(&(None));
    CAMERA_GUARDS_ENABLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/camera/src/enableCameraGuards.ts:22 (sha256:2398881b7bb36ac0fbb1a41bbc3b65b141260571268604ffca6eb30ba68f6d62)
pub fn enable_camera_guards() -> () {
    set_camera3_d_view_guard(
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Camera3D| -> () {
                warn_on_non_orthonormal_camera3_d_view(&__flight_argument_0)
            },
        )
            as Box<dyn FnMut(Camera3D) -> () + Send + 'static>)))),
    );
    set_camera2_d_visible_bounds_guard(
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Camera2D| -> () {
                warn_on_degenerate_camera2_d_visible_bounds(&__flight_argument_0)
            },
        )
            as Box<dyn FnMut(Camera2D) -> () + Send + 'static>)))),
    );
    CAMERA_GUARDS_ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/camera/src/enableCameraGuards.ts:31 (sha256:88677ba24ebd987947dbc9d4a13805bbdda253b206fd7ef4ee2a136e7a52833a)
fn warn_on_degenerate_camera2_d_visible_bounds(camera: &Camera2D) -> () {
    log_once(
        format!("camera:degenerate-visible-bounds:{}", camera.zoom),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = "getCamera2DVisibleBounds: the view matrix has no inverse, so the visible rectangle is unbounded and nothing is culled — a zoom of 0 is the usual cause; set a non-zero zoom.".to_owned(); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record.push(("zoom".to_owned(), {
                let __flight_portable_source = camera.zoom;
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            }));
            __flight_record
        }))),
        None,
    );
}

// Source: upstream/packages/camera/src/enableCameraGuards.ts:42 (sha256:09bfd0c81f6562cae6aa72ce995168d9a6c2afef4e86ba62ab6447e0f7d3c40d)
fn warn_on_non_orthonormal_camera3_d_view(camera: &Camera3D) -> () {
    let explanation = explain_camera3_d_view(camera);
    if explanation.is_orthonormal {
        return;
    }
    log_once(
        "camera:non-orthonormal-view".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = "setCamera3DViewMatrix4FromMatrix4: the view matrix is not orthonormal, which its consumers rely on — a scaled matrix is the usual cause. Reflections are fine; scale is not. Call explainCamera3DView(camera) for the measured deviations.".to_owned(); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record.push(("scaleDeviation".to_owned(), {
                let __flight_portable_source = explanation.scale_deviation;
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            }));
            __flight_record.push(("shearDeviation".to_owned(), {
                let __flight_portable_source = explanation.shear_deviation;
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            }));
            __flight_record
        }))),
        None,
    );
}

// Source: upstream/packages/camera/src/enableCameraGuards.ts:53 (sha256:1e7fdcc3929f64171a398bc4aaffa761999d869e62b88d5f7f2b9154dac6072d)
static CAMERA_GUARDS_ENABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
