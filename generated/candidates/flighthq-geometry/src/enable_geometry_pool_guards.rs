// @generated from upstream/packages/geometry/src/enableGeometryPoolGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GEOMETRY_POOL_RELEASE_GUARD as geometry_pool_release_guard_constant,
    set_geometry_pool_release_guard,
};
use flighthq_log::log_once;
use flighthq_types::{LogData, LogDataProvider, LogLevel};

// Source: upstream/packages/geometry/src/enableGeometryPoolGuards.ts:6 (sha256:a3e7168c341bab86e1d462fa6f5e0ba4418ed2e5fc0e0585ee910182fe76f6b2)
pub(crate) type GeometryPoolReleaseFunction = GeometryPoolReleaseFunction;

// Source: upstream/packages/geometry/src/enableGeometryPoolGuards.ts:8 (sha256:53f3a8bf013823df90d13d3ca73d6ee3f34dfd2ed391ff40211a21fddd75bcc5)
pub fn are_geometry_pool_guards_enabled() -> bool {
    return (geometry_pool_release_guard_constant).is_some();
}

// Source: upstream/packages/geometry/src/enableGeometryPoolGuards.ts:12 (sha256:333e34cc0dba2cf47b8f9e9848139002d86fbbf8efe4ebd4f4603e6fbfc1b7ea)
pub fn disable_geometry_pool_guards() -> () {
    set_geometry_pool_release_guard(&(None));
}

// Source: upstream/packages/geometry/src/enableGeometryPoolGuards.ts:19 (sha256:0c1d94435c643df3baaadc95f7ea5f3cebf515536739800f64a36273a5341c20)
pub fn enable_geometry_pool_guards() -> () {
    set_geometry_pool_release_guard(
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: GeometryPoolReleaseFunction| -> () {
                warn_on_double_release((__flight_argument_0).clone())
            },
        )
            as Box<
                dyn FnMut(GeometryPoolReleaseFunction) -> () + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/geometry/src/enableGeometryPoolGuards.ts:23 (sha256:a345678a04d786aa06dd140c6dff010bd68446a4b7c71ba7960389a3e4ecd364)
#[derive(Clone, Default)]
struct WarnOnDoubleReleaseRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnDoubleReleaseRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_double_release(release_function: GeometryPoolReleaseFunction) -> () {
    let acquire_functions = ACQUIRE_FUNCTIONS_BY_RELEASE_FUNCTION
        .iter()
        .find(|(entry_key, _)| entry_key == &(release_function).clone())
        .map(|(_, value)| value.clone())
        .clone();
    log_once(
        format!("geometry:double-release:{}", (release_function).clone()),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("{}: this value is already in its pool, so it is being released twice. Two later matching acquire calls will hand back the same object and unrelated owners will alias each other. Every {} call pairs with exactly one {} call, and the value must not be used after release.", (release_function).clone(), ((acquire_functions).clone()).as_ref().map_or_else(|| "undefined".to_owned(), |value| value.to_string()), (release_function).clone()); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("geometry".to_owned()).clone()),
    );
}

// Source: upstream/packages/geometry/src/enableGeometryPoolGuards.ts:35 (sha256:d99af91752ea49ccc775fa0d3e227f87d25ff67f02c392dea30cca5bfb1fabb6)
static ACQUIRE_FUNCTIONS_BY_RELEASE_FUNCTION: std::sync::LazyLock<
    Vec<(GeometryPoolReleaseFunction, String)>,
> = std::sync::LazyLock::new(|| {
    let mut __flight_record = Vec::new();
    __flight_record.push((
        "releaseMatrix",
        "acquireMatrix or acquireIdentityMatrix".to_owned(),
    ));
    __flight_record.push((
        "releaseMatrix3",
        "acquireMatrix3 or acquireIdentityMatrix3".to_owned(),
    ));
    __flight_record.push((
        "releaseMatrix4",
        "acquireMatrix4 or acquireIdentityMatrix4".to_owned(),
    ));
    __flight_record.push((
        "releaseQuaternion",
        "acquireQuaternion or acquireIdentityQuaternion".to_owned(),
    ));
    __flight_record.push((
        "releaseRectangle",
        "acquireRectangle or acquireEmptyRectangle".to_owned(),
    ));
    __flight_record.push((
        "releaseVector2",
        "acquireVector2 or acquireEmptyVector2".to_owned(),
    ));
    __flight_record.push((
        "releaseVector3",
        "acquireVector3 or acquireEmptyVector3".to_owned(),
    ));
    __flight_record.push((
        "releaseVector4",
        "acquireVector4 or acquireEmptyVector4".to_owned(),
    ));
    __flight_record
});
