// @generated from upstream/packages/types/src/RenderCacheAdapterSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/RenderCacheAdapterSignals.ts:3 (sha256:5b436c2cfc60c1f85dcc93720d519d0d14ba6ff6bda8db8c3f1b82473b07c237)
#[derive(Clone)]
pub struct RenderCacheAdapterSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_prepare:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for RenderCacheAdapterSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
