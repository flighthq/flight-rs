// @generated from upstream/packages/entity/src/runtime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Entity, EntityRuntime};

// Source: upstream/packages/entity/src/runtime.ts:4 (sha256:7ac291f0aeebe33601241673478f870e62300f9f5b580a8708e4deee6bb4b586)
pub fn create_entity_runtime() -> EntityRuntime {
    return EntityRuntime {
        __flight_identity: std::sync::Arc::new(()),
        binding: None,
    };
}

// Source: upstream/packages/entity/src/runtime.ts:10 (sha256:94d31433c6818ceed765bb6e598a09fdd4cc3f9d142409afeac22983b329d5ed)
pub fn get_entity_runtime(source: &Entity) -> EntityRuntime {
    return panic!("entity runtime storage requires the generated native entity trait");
}

// Source: upstream/packages/entity/src/runtime.ts:17 (sha256:e9b0a17ebde42710d744cc13606857b292cd0fb62aaf84c1496fe8a18da75bd4)
pub fn has_entity_runtime(source: &Entity) -> bool {
    return (None::<crate::OpaqueHostValue>).is_some();
}
