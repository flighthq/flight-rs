// @generated from upstream/packages/entity/src/binding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_entity_runtime, get_entity_runtime};
use flighthq_types::Entity;

// Source: upstream/packages/entity/src/binding.ts:6 (sha256:47a09091ba08bf14ad23b4933af457ef86e2292786efa10cd66f2056e652b580)
pub fn attach_entity_binding(entity: &mut Entity, binding: crate::OpaqueHostValue) -> () {
    if flighthq_types::FlightEntity::__flight_entity_runtime(entity)
        .lock()
        .unwrap()
        .is_none()
    {
        *flighthq_types::FlightEntity::__flight_entity_runtime(entity)
            .lock()
            .unwrap() = Some(create_entity_runtime());
    }
    {
        let __flight_runtime = ({
            let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(entity)
                .lock()
                .unwrap()
                .clone()
                .expect("entity runtime was read before initialization");
            __flight_runtime
        });
        let __flight_value = Some((binding).clone());
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.binding = __flight_value;
    };
}

// Source: upstream/packages/entity/src/binding.ts:13 (sha256:2797af123d40539c338319f7719810aa65e7085c4ba77f834b002d62d9f66197)
pub fn get_entity_binding(source: &Entity) -> Option<crate::OpaqueHostValue> {
    let runtime = get_entity_runtime(source);
    return (runtime.inner.lock().unwrap().binding).clone();
}
