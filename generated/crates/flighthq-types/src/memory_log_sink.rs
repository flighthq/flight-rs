// @generated from upstream/packages/types/src/MemoryLogSink.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::LogSink;

// Source: upstream/packages/types/src/MemoryLogSink.ts:5 (sha256:096b0a3018661b193e0d08749cae51967121c2f5ac3e9c4e4d3017b0fa7cb53a)
#[derive(Clone)]
pub struct MemoryLogSink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub sink: LogSink,
}
impl PartialEq for MemoryLogSink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
