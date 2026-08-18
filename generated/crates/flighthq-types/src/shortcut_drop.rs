// @generated from upstream/packages/types/src/ShortcutDrop.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AcceleratorParseError;

// Source: upstream/packages/types/src/ShortcutDrop.ts:8 (sha256:580bc6f22b1faec209a65e24a04d5f8201426d1dd871ab8fa8c5d3e2c81602fd)
#[derive(Clone, Default)]
pub struct ShortcutDrop {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: ShortcutOperation,
    pub accelerator: String,
    pub reason: ShortcutDropReason,
    pub parse_error: Option<AcceleratorParseError>,
}
impl PartialEq for ShortcutDrop {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShortcutDrop.ts:22 (sha256:45668f473eba182aa93152acaf9f5569d1c3aed9053da08d80a7da78df760230)
pub type ShortcutDropGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ShortcutDrop) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/ShortcutDrop.ts:24 (sha256:647ed59a35ec34b0e92bebaff05cb6c5c2cd948196e1a09c7105808cf3833934)
pub type ShortcutDropReason = String;

// Source: upstream/packages/types/src/ShortcutDrop.ts:26 (sha256:8762e26a301a834795c4b223f8428a7906e947f4eb030164e60e6f9763c19df4)
pub type ShortcutOperation = String;
