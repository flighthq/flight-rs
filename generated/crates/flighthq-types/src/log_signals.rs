// @generated from upstream/packages/types/src/LogSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{LogEntry, Signal};

// Source: upstream/packages/types/src/LogSignals.ts:6 (sha256:0130bcc5462c56bde339e89e376c8e1f575ecef2a420cc5d1402236d8286f524)
#[derive(Clone)]
pub struct LogSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_log_entry:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LogEntry) -> () + Send + 'static>>>>,
    pub on_log_error:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LogEntry) -> () + Send + 'static>>>>,
}
impl PartialEq for LogSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
