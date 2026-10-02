// @generated from upstream/packages/types/src/SpritesheetFormatDescriptor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SpritesheetData, SpritesheetFormatKind, SpritesheetParseOptions};

// Source: upstream/packages/types/src/SpritesheetFormatDescriptor.ts:11 (sha256:c852f4684a8604c766a21ceca0e6421b5aba6c24882af11f0f6ae19526811268)
#[derive(Clone)]
pub struct SpritesheetFormatEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detect: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub parse: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, SpritesheetParseOptions) -> SpritesheetData + Send + 'static>,
        >,
    >,
}
impl PartialEq for SpritesheetFormatEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpritesheetFormatDescriptor.ts:24 (sha256:4f5efaa160662ffb5da28429d094cf33638b7277287927253d6a8e9bcbc295d7)
#[derive(Clone)]
pub struct SpritesheetFormatDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry: SpritesheetFormatEntry,
    pub kind: SpritesheetFormatKind,
}
impl PartialEq for SpritesheetFormatDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpritesheetFormatDescriptor.ts:48 (sha256:a914caa772b45fca19ddf157e87eb36c7f480cbaddd7a3f795cf73cf3c803e86)
#[derive(Clone, Default)]
pub struct SpritesheetImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub formats: Option<Vec<SpritesheetFormatDescriptor>>,
}
impl PartialEq for SpritesheetImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
