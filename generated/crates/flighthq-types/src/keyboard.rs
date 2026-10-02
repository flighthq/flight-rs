// @generated from upstream/packages/types/src/Keyboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Keyboard.ts:3 (sha256:42d3279c2cdb20aff9d3a5d35242e92c0714de035a50fa2de7972dfe72d49e7f)
pub type SoftKeyboardResizeMode = String;

// Source: upstream/packages/types/src/Keyboard.ts:4 (sha256:ad2d01451b6eafd97a83382393efea599d0cfba42550fdd2a9b1f8c645fee583)
pub const SOFT_KEYBOARD_RESIZE_NONE_KIND: &'static str = "None";

// Source: upstream/packages/types/src/Keyboard.ts:5 (sha256:69d3297dec8fe1ac076005d02500ee846c3a7b06b44e314f1012f9b4b2a9ac9d)
pub const SOFT_KEYBOARD_RESIZE_BODY_KIND: &'static str = "Body";

// Source: upstream/packages/types/src/Keyboard.ts:6 (sha256:61cb285d309b01b744f3fc7c7c67202dd25225a54b714756e650a6aad24f131a)
pub type SoftKeyboardStyleKind = String;

// Source: upstream/packages/types/src/Keyboard.ts:7 (sha256:91265f26ace88abf4092596612cf0a84584af6337641c53aeae75be49b0095b6)
pub const SOFT_KEYBOARD_STYLE_DEFAULT_KIND: &'static str = "Default";

// Source: upstream/packages/types/src/Keyboard.ts:8 (sha256:9266face5c7ff8b54abd7086eaae3137e8c8c1a028707c1996470fd2c52e8f21)
pub const SOFT_KEYBOARD_STYLE_DARK_KIND: &'static str = "Dark";

// Source: upstream/packages/types/src/Keyboard.ts:10 (sha256:0d37ab980102fd6c29da9c33e3ff69749aa8fc3fceed59fed424bf51c17c3ca4)
#[derive(Clone, Default)]
pub struct SoftKeyboardInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub visible: bool,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
}
impl PartialEq for SoftKeyboardInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:17 (sha256:2570847ccc83abcfda2129d907c117c104272439a99cf9c890947af88f800a44)
#[derive(Clone)]
pub struct SoftKeyboard {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_show:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_hide: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_resize:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
}
impl PartialEq for SoftKeyboard {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:23 (sha256:d3b435f047ab69a9851c6fd1ffce417b9a26941027b386c9e5db0438f1cebd8e)
pub type SoftKeyboardVisibilityResult = String;

// Source: upstream/packages/types/src/Keyboard.ts:24 (sha256:606467addaba613f4a311711d80b7fa31678108f7e206bd7d9726678b66a3fdd)
pub const SOFT_KEYBOARD_VISIBILITY_OK_KIND: &'static str = "ok";

// Source: upstream/packages/types/src/Keyboard.ts:25 (sha256:ebdd2fd1343f4a68251200fb7cf10041a555413a62bfe88f23f15878cd7c454f)
pub const SOFT_KEYBOARD_VISIBILITY_OPERATION_FAILED_KIND: &'static str = "operation-failed";

// Source: upstream/packages/types/src/Keyboard.ts:27 (sha256:2e1ae8be0ba955b3758701bd487cec095cec3503072b226bafab27480808061d)
pub type SoftKeyboardSetterResult = String;

// Source: upstream/packages/types/src/Keyboard.ts:28 (sha256:38a7c996c4c06156a84b9339b4d96a3419e8e85fd19b42bce121663ce0be8621)
pub const SOFT_KEYBOARD_SETTER_OK_KIND: &'static str = "ok";

// Source: upstream/packages/types/src/Keyboard.ts:29 (sha256:b25cfb21f22e1440833202636b94da30748f6c56e5c4748b5483ec55c67ea23d)
pub const SOFT_KEYBOARD_SETTER_OPERATION_UNAVAILABLE_KIND: &'static str = "operation-unavailable";

// Source: upstream/packages/types/src/Keyboard.ts:30 (sha256:24d0e95cb8bc6aeb9ca2dd4e2a27c9ab991a1ad3cad09aa93247da5e62279b37)
pub const SOFT_KEYBOARD_SETTER_OPERATION_FAILED_KIND: &'static str = "operation-failed";

// Source: upstream/packages/types/src/Keyboard.ts:32 (sha256:c16f159abb531eaa732a2698fa745de4bdee5192be82005d39c863562ce20a5e)
pub type SoftKeyboardAttachResult = String;

// Source: upstream/packages/types/src/Keyboard.ts:33 (sha256:6d8d5fb567fafbed10f2cc249922719249a9d77b65079a3ed4573b4725140c66)
pub const SOFT_KEYBOARD_ATTACH_OK_KIND: &'static str = "ok";

// Source: upstream/packages/types/src/Keyboard.ts:34 (sha256:9721f4df9eefee5ffea81591f84a7bb2a28a27e9f37ae6c9615795d3ba071a3e)
pub const SOFT_KEYBOARD_ATTACH_ACQUISITION_FAILED_KIND: &'static str = "acquisition-failed";

// Source: upstream/packages/types/src/Keyboard.ts:36 (sha256:dbe223e42dff710978096be8bdefeae9feb12c420e129e9f0a50675b09f54f19)
#[derive(Clone)]
pub struct HostSoftKeyboardInfoCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_info: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SoftKeyboardInfo) -> SoftKeyboardInfo + Send + 'static>>,
    >,
}
impl PartialEq for HostSoftKeyboardInfoCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:40 (sha256:fbd09342c1e20ea7caaa7258a96f81d09e54791df878ddfd34a45e9fa63822a0)
#[derive(Clone, Default)]
pub struct SoftKeyboardChangeSubscription {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub result: SoftKeyboardAttachResult,
    pub unsubscribe:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for SoftKeyboardChangeSubscription {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:45 (sha256:2393bc24a36d06de06f5bdd7d7d4bcc46821f0f6c3b83713888961e4db751aa0)
#[derive(Clone)]
pub struct HostSoftKeyboardChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> crate::FlightTask<SoftKeyboardChangeSubscription>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostSoftKeyboardChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:49 (sha256:bb01c4070396b34f2be620a9e93a14ccdd56d0b0ef50801aa275457a5748068b)
#[derive(Clone)]
pub struct HostSoftKeyboardVisibilityCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub show: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<SoftKeyboardVisibilityResult> + Send + 'static>,
        >,
    >,
    pub hide: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<SoftKeyboardVisibilityResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostSoftKeyboardVisibilityCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:54 (sha256:b1d2322fe77b637786af12c5b9b16e0f8f75abeb55f18d7107abb2dea03829be)
#[derive(Clone)]
pub struct HostSoftKeyboardResizeModeWriteCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_resize_mode: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(SoftKeyboardResizeMode) -> crate::FlightTask<SoftKeyboardSetterResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostSoftKeyboardResizeModeWriteCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:58 (sha256:68b6ef5b520105bdb12f23896fc9dc4e03028527c601a9089173f9e59c5fb4e5)
#[derive(Clone)]
pub struct HostSoftKeyboardStyleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_style: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(SoftKeyboardStyleKind) -> crate::FlightTask<SoftKeyboardSetterResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostSoftKeyboardStyleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:62 (sha256:5336c24ff754fc0a1ff4f20ddd618939195432aafbf0e08307dc4313141f0d4e)
#[derive(Clone)]
pub struct HostSoftKeyboardAccessoryBarCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_accessory_bar_visible: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(bool) -> crate::FlightTask<SoftKeyboardSetterResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostSoftKeyboardAccessoryBarCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Keyboard.ts:66 (sha256:b1aab0ffa261857f5454bfa82ee4aa281c7b119342f43c16a244c1276fdd9a2b)
#[derive(Clone)]
pub struct HostSoftKeyboardScrollAssistCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_scroll_assist_enabled: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(bool) -> crate::FlightTask<SoftKeyboardSetterResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostSoftKeyboardScrollAssistCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
