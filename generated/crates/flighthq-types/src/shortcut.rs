// @generated from upstream/packages/types/src/Shortcut.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Shortcut.ts:6 (sha256:b9df7d16c8855374dfa5e55d916aa9c1392625dd0f070ff9421d5d322fbd3160)
#[derive(Clone)]
pub struct ShortcutBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_registered:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub is_registered:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub register: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_all_enabled:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_enabled:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, bool) -> bool + Send + 'static>>>,
    pub unregister:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub unregister_all: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for ShortcutBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
