// @generated from upstream/packages/types/src/InputKeyRepeatOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InputKeyRepeatOptions.ts:1 (sha256:71b47163b5d905d89ebc9a2ba1b7112cc7795fda6fad731934248845b5d86e1d)
#[derive(Clone, Default)]
pub struct InputKeyRepeatOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub delay: f64,
    pub interval: f64,
}
impl PartialEq for InputKeyRepeatOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
