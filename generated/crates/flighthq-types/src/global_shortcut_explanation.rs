// @generated from upstream/packages/types/src/GlobalShortcutExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Accelerator, AcceleratorParseError};

// Source: upstream/packages/types/src/GlobalShortcutExplanation.ts:9 (sha256:35d185b1d1c953a13d662659bb39539173e8bfc03b3b429160a3fd50faa53f31)
#[derive(Clone, Default)]
pub struct GlobalShortcutExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accelerator: String,
    pub normalized: Option<Accelerator>,
    pub parse_error: Option<AcceleratorParseError>,
    pub has_native_backend: bool,
    pub registered: bool,
    pub reason: GlobalShortcutBlockReason,
}
impl PartialEq for GlobalShortcutExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlobalShortcutExplanation.ts:30 (sha256:2993797dce30249fe333103993575bc483c350d984f4f551e80b1ae59cbce75d)
pub type GlobalShortcutBlockReason = String;
