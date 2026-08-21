// @generated from upstream/packages/skeleton2d/src/enableSkeleton2DGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{set_skeleton2_d_coerced_interpolation_guard, set_skeleton2_d_deform_length_guard};
use flighthq_log::log_once;
use flighthq_types::{
    LogData, LogDataProvider, LogLevel, Skeleton2DCoercedInterpolation,
    Skeleton2DDeformLengthMismatch,
};

// Source: upstream/packages/skeleton2d/src/enableSkeleton2DGuards.ts:8 (sha256:0d568aea37cba50435b92df08956e835fde05c696b30e8656367f1b1e700cc53)
pub fn disable_skeleton2_d_guards() -> () {
    set_skeleton2_d_coerced_interpolation_guard(&(None));
    set_skeleton2_d_deform_length_guard(&(None));
}

// Source: upstream/packages/skeleton2d/src/enableSkeleton2DGuards.ts:32 (sha256:ac8fc578c534491a847e4f1c1a8b26828f6ca4b954620c46b4bc614a836ae33f)
pub fn enable_skeleton2_d_guards() -> () {
    set_skeleton2_d_coerced_interpolation_guard(
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Skeleton2DCoercedInterpolation| -> () {
                warn_on_coerced_interpolation(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(Skeleton2DCoercedInterpolation) -> () + Send + 'static,
            >)))),
    );
    set_skeleton2_d_deform_length_guard(
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Skeleton2DDeformLengthMismatch| -> () {
                warn_on_deform_length_mismatch(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(Skeleton2DDeformLengthMismatch) -> () + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/skeleton2d/src/enableSkeleton2DGuards.ts:37 (sha256:dd2ab561861068fbed5e7475a34a659a3f4ef3da451e4572b0a48db4e565e458)
#[derive(Clone, Default)]
struct WarnOnCoercedInterpolationRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnCoercedInterpolationRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_coerced_interpolation(report: &Skeleton2DCoercedInterpolation) -> () {
    log_once(
        format!(
            "skeleton2d:coerced-interpolation:{}",
            (report.subject).clone()
        ),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("A {} channel states '{}' interpolation but is walked as '{}'. The value it carries cannot be blended — an attachment index between two table entries names art nobody authored, and a draw order between two orderings gives fractional sort keys — so the step is forced and the stated easing has no effect. Author the track as '{}' to say what actually happens, or drive a blendable property instead.", (report.subject).clone(), (report.stated).clone(), (report.applied).clone(), (report.applied).clone()); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("skeleton2d".to_owned()).clone()),
    );
}

// Source: upstream/packages/skeleton2d/src/enableSkeleton2DGuards.ts:48 (sha256:669a7b96815ad39d0499521eb80f62db4b171635750bc4bac6a703984c081a1a)
#[derive(Clone, Default)]
struct WarnOnDeformLengthMismatchRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnDeformLengthMismatchRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_deform_length_mismatch(report: &Skeleton2DDeformLengthMismatch) -> () {
    log_once(
        format!("skeleton2d:deform-length:{}", (report.subject).clone()),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("Deform offsets on '{}' carry {} values but the vertex stream they address needs {}. The offsets are ignored rather than partly applied: a stream of the wrong length has no correct prefix, so consuming what fits would deform some vertices and silently leave the rest in their setup pose. Re-export the attachment, or check that the deform timeline belongs to this attachment.", (report.subject).clone(), report.offsets, report.addressed); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("skeleton2d".to_owned()).clone()),
    );
}
