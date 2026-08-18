// @generated from upstream/packages/types/src/FileLogSink.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::LogSink;

// Source: upstream/packages/types/src/FileLogSink.ts:5 (sha256:b3d55d16a3a9dce68884f833205b265f9c7f38adc70fc748558df7d0c665621b)
#[derive(Clone)]
pub struct FileLogSink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub sink: LogSink,
}
impl PartialEq for FileLogSink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
