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

// Source: upstream/packages/types/src/Clipboard.ts:17 (sha256:ea92fef6d9d21df1a4e21bd00f296493760a7f632ed3d61cd48d7e0564b8c36f)
#[derive(Clone)]
pub struct ClipboardBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub read_text: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<String> + Send + 'static>>,
    >,
    pub write_text: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub read_html: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<String> + Send + 'static>>,
    >,
    pub write_html: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub has_text:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<bool> + Send + 'static>>>,
    pub read_image: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<String> + Send + 'static>>,
    >,
    pub write_image: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub has_image:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<bool> + Send + 'static>>>,
    pub read_rtf: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<String> + Send + 'static>>,
    >,
    pub write_rtf: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub read_bookmark: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::Promise<Option<ClipboardBookmark>> + Send + 'static>,
        >,
    >,
    pub write_bookmark: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String, String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub read_format: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<String> + Send + 'static>>,
    >,
    pub write_format: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String, String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub has_format: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub get_formats: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<Vec<String>> + Send + 'static>>,
    >,
    pub read_items: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<String>) -> crate::Promise<Vec<(String, String)>> + Send + 'static>,
        >,
    >,
    pub write_items: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<ClipboardWriteItem>) -> crate::Promise<bool> + Send + 'static>,
        >,
    >,
    pub read_files: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<Vec<String>> + Send + 'static>>,
    >,
    pub write_files: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<String>) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub clear:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<bool> + Send + 'static>>>,
    pub get_change_count:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub subscribe_clipboard_change: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ClipboardBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
