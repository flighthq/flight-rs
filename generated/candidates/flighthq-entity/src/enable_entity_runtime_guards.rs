// @generated from upstream/packages/entity/src/enableEntityRuntimeGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{set_entity_runtime_guard_mode, set_entity_runtime_write_guard};
use flighthq_log::log_once;
use flighthq_types::{EntityRuntime, EntityRuntimeWriteSlot, LogData, LogDataProvider, LogLevel};

// Source: upstream/packages/entity/src/enableEntityRuntimeGuards.ts:8 (sha256:393c63a6f6465e8a34c630f1f2be78fc1e6407ddb74cc5d9ab24fb6839dc2018)
pub fn disable_entity_runtime_guards() -> () {
    set_entity_runtime_guard_mode(false);
    set_entity_runtime_write_guard(&(None));
}

// Source: upstream/packages/entity/src/enableEntityRuntimeGuards.ts:22 (sha256:02ab9035b30f44af00104f7a07807b9755b3096a10d65fd6a48b81d6d5546e66)
pub fn enable_entity_runtime_guards() -> () {
    set_entity_runtime_guard_mode(true);
    set_entity_runtime_write_guard(&(warn_on_direct_write));
}

// Source: upstream/packages/entity/src/enableEntityRuntimeGuards.ts:27 (sha256:5a6691d7fc9356afe927a52b4a187e87eb00ce7edc335675998f9bb41db09b60)
#[derive(Clone, Default)]
struct WarnOnDirectWriteRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnDirectWriteRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_direct_write(slot: EntityRuntimeWriteSlot) -> () {
    if (slot == "binding-slot") {
        log_once(
            "entity:direct-binding-write".to_owned(),
            LogLevel::Warn,
            &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                String,
                Vec<(String, crate::FlightValue)>,
            >::B({
                let mut __flight_record = Vec::new();
                __flight_record.push(("message".to_owned(), { let __flight_portable_source = "EntityRuntime.binding was written directly. Use attachEntityBinding or detachEntityBinding, which keep the binding and the runtime consistent; the write was allowed but is not tracked.".to_owned(); crate::FlightValue::String((&__flight_portable_source).clone()) }));
                __flight_record
            }))),
            Some(("entity".to_owned()).clone()),
        );
        return;
    }
    log_once(
        "entity:direct-runtime-write".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = "An entity's runtime slot was written directly. Use attachEntityBinding, which allocates the slot for you; the write was allowed, but bypassing it is how a runtime ends up on the wrong entity.".to_owned(); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("entity".to_owned()).clone()),
    );
}
