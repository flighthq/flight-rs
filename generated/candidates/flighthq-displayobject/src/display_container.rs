// @generated from upstream/packages/displayobject/src/displayContainer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_types::{
    DISPLAY_OBJECT_KIND as display_object_kind_constant, DisplayContainer, DisplayContainerRuntime,
};

// Source: upstream/packages/displayobject/src/displayContainer.ts:6 (sha256:1400add81cdb09f32044940a3e7160c9c8bdf103c12acdb5e22e8e8b74d6f7d9)
pub fn create_display_container(obj: Option<DisplayContainer>) -> DisplayContainer {
    return create_display_object_generic(
        (display_object_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(undefined),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_display_container_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/displayobject/src/displayContainer.ts:15 (sha256:03ea635558047bbf20d579c36a8a9aa3ccb3466e55af8411f9dd57a1ab2c7061)
pub fn create_display_container_runtime() -> DisplayContainerRuntime {
    return create_display_object_runtime(None);
}

// Source: upstream/packages/displayobject/src/displayContainer.ts:19 (sha256:c0fe0804ae1ed3e8e3c68e3f4a3491c7ddb47ff624d086b52e33a1201063658c)
pub fn get_display_container_runtime(source: &DisplayContainer) -> DisplayContainerRuntime {
    return get_display_object_runtime(source);
}
