// @generated from upstream/packages/types/src/Clipboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Clipboard.ts:2 (sha256:f807ed597514439692ed56f36de242e15a76aa6a22154813ff5cb12500150119)
#[derive(Clone, Default)]
pub struct ClipboardBookmark {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: String,
    pub url: String,
}
impl PartialEq for ClipboardBookmark {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Clipboard.ts:9 (sha256:7386aa9e0d59fb35d0826be076149f4b1898c4e882d038d1f1e364735a201249)
#[derive(Clone, Default)]
pub struct ClipboardWriteItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: String,
    pub data: String,
}
impl PartialEq for ClipboardWriteItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Clipboard.ts:16 (sha256:38a8ebba2f2f8796e948edc59920d274004b7ebe661fa056990dc2fe3c9177de)
#[derive(Clone)]
pub struct HostClipboardBookmarkCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub read_bookmark: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<Option<ClipboardBookmark>> + Send + 'static>,
        >,
    >,
    pub write_bookmark: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, String) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostClipboardBookmarkCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Clipboard.ts:23 (sha256:311491561f69b736fb2c522b1ddfad56a3d796094e64ae4714d1442898e0be9f)
#[derive(Clone, Default)]
pub struct HostClipboardChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for HostClipboardChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Clipboard.ts:29 (sha256:1c7708e50baeb81beff077e5f7db5a3e16403b775ea922249784e1da3125c761)
#[derive(Clone)]
pub struct HostClipboardFormatsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_formats: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<Vec<String>> + Send + 'static>>,
    >,
    pub has_format: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub read_format: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub read_html: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub read_items: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Vec<String>) -> crate::FlightTask<Vec<(String, String)>> + Send + 'static,
            >,
        >,
    >,
    pub read_rtf: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub write_format: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, String) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
    pub write_html: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub write_items: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<ClipboardWriteItem>) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
    pub write_rtf: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for HostClipboardFormatsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Clipboard.ts:43 (sha256:de16f2fbb6c6d553b50ea2f655ad440ede02310ec1047f1aff97fe4674a369ea)
#[derive(Clone)]
pub struct HostClipboardImageCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_image: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub read_image: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub write_image: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for HostClipboardImageCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Clipboard.ts:50 (sha256:75260efc750e43d514c60d13814b86feb889e7830da5cf1d7d079e9bb77288ff)
#[derive(Clone)]
pub struct HostClipboardTextCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub has_text: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub read_text: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub write_text: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for HostClipboardTextCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
