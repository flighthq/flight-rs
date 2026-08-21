// @generated from upstream/packages/skeleton2d/src/skeleton2dGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    Skeleton2DCoercedInterpolation, Skeleton2DCoercedInterpolationGuard,
    Skeleton2DDeformLengthGuard, Skeleton2DDeformLengthMismatch,
};

// Source: upstream/packages/skeleton2d/src/skeleton2dGuards.ts:13 (sha256:e888448b87a2a41e22c77f48d261ace5018483273315dd5d4ef1415164635673)
#[derive(Clone, Default)]
struct ReportSkeleton2DCoercedInterpolationSynthesizedRecord1618767717 {
    __flight_identity: std::sync::Arc<()>,
    applied: String,
    stated: String,
    subject: String,
}
impl PartialEq for ReportSkeleton2DCoercedInterpolationSynthesizedRecord1618767717 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn report_skeleton2_d_coerced_interpolation(
    subject: String,
    stated: String,
    applied: String,
) -> () {
    if ((*_COERCED_INTERPOLATION_GUARD.lock().unwrap()).clone()).is_none() {
        return;
    }
    {
        let __flight_callback = ((*_COERCED_INTERPOLATION_GUARD.lock().unwrap())
            .as_ref()
            .unwrap())
        .clone();
        __flight_callback.lock().unwrap()({
            let __flight_source =
                &(ReportSkeleton2DCoercedInterpolationSynthesizedRecord1618767717 {
                    __flight_identity: std::sync::Arc::new(()),
                    applied: (applied).clone(),
                    stated: (stated).clone(),
                    subject: (subject).clone(),
                });
            Skeleton2DCoercedInterpolation {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                applied: (__flight_source.applied).clone(),
                stated: (__flight_source.stated).clone(),
                subject: (__flight_source.subject).clone(),
            }
        })
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dGuards.ts:18 (sha256:70aec7295514b990c10893b48e41650bfa8673586e5795a084b7a26a38dea0a9)
#[derive(Clone, Default)]
struct ReportSkeleton2DDeformLengthMismatchSynthesizedRecord2386502204 {
    __flight_identity: std::sync::Arc<()>,
    addressed: f64,
    offsets: f64,
    subject: String,
}
impl PartialEq for ReportSkeleton2DDeformLengthMismatchSynthesizedRecord2386502204 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn report_skeleton2_d_deform_length_mismatch(
    subject: String,
    offsets: f64,
    addressed: f64,
) -> () {
    if ((*_DEFORM_LENGTH_GUARD.lock().unwrap()).clone()).is_none() {
        return;
    }
    {
        let __flight_callback = ((*_DEFORM_LENGTH_GUARD.lock().unwrap()).as_ref().unwrap()).clone();
        __flight_callback.lock().unwrap()({
            let __flight_source =
                &(ReportSkeleton2DDeformLengthMismatchSynthesizedRecord2386502204 {
                    __flight_identity: std::sync::Arc::new(()),
                    addressed: addressed,
                    offsets: offsets,
                    subject: (subject).clone(),
                });
            Skeleton2DDeformLengthMismatch {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                addressed: __flight_source.addressed,
                offsets: __flight_source.offsets,
                subject: (__flight_source.subject).clone(),
            }
        })
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dGuards.ts:23 (sha256:e2b66c189a50c4818208be66581ed01cf53cb30b057bea2417211c63d4492173)
pub fn set_skeleton2_d_coerced_interpolation_guard(
    guard: &Option<Skeleton2DCoercedInterpolationGuard>,
) -> () {
    (*_COERCED_INTERPOLATION_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/skeleton2d/src/skeleton2dGuards.ts:27 (sha256:5f8f4ec244eadbef596a7919b9e5e3f05d3ea3a355845bbfade1b5f4e41dd4d4)
pub fn set_skeleton2_d_deform_length_guard(guard: &Option<Skeleton2DDeformLengthGuard>) -> () {
    (*_DEFORM_LENGTH_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/skeleton2d/src/skeleton2dGuards.ts:31 (sha256:4b25d8ed103e0b5679531e62505ffd685e2229c2c678103fa4f020bbddc2c0c0)
static _COERCED_INTERPOLATION_GUARD: std::sync::LazyLock<
    std::sync::Mutex<Option<Skeleton2DCoercedInterpolationGuard>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/skeleton2d/src/skeleton2dGuards.ts:32 (sha256:12648479bc223c44ee9889c99e898a758ab33b21107cff5cc111e9c981fe4ace)
static _DEFORM_LENGTH_GUARD: std::sync::LazyLock<
    std::sync::Mutex<Option<Skeleton2DDeformLengthGuard>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
