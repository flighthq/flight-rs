// @generated from upstream/packages/types/src/BufferedLogSink.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::LogSink;

// Source: upstream/packages/types/src/BufferedLogSink.ts:4 (sha256:1c01bbc33c3f983e60996f005f7a847e8803511149ec93c7e4b4e8c5111d3f66)
#[derive(Clone)]
pub struct BufferedLogSink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub sink: LogSink,
}
impl PartialEq for BufferedLogSink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
