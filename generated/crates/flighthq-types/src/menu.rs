// @generated from upstream/packages/types/src/Menu.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{HostMenuCapabilities, WellKnownMenuItemRoleValue};

#[derive(Clone)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Menu.ts:11 (sha256:dfbdbbccb432e1c36c144fb2f1aace8a67468a363d8cc9d42ff46f7502fd8449)
pub type MenuItemType = String;

// Source: upstream/packages/types/src/Menu.ts:18 (sha256:7b2d8ece52899dd8323a11ea129b3d1a423606c124a9ecebb9edc4d05987cf04)
pub type MenuItemRole = crate::FlightUnion2<WellKnownMenuItemRoleValue, String>;

// Source: upstream/packages/types/src/Menu.ts:20 (sha256:e21b9e7a11fef01e2c657165cbe9d2b6bd967ade2ce23641b0b431c272bdfa8b)
#[derive(Clone, Default)]
pub struct MenuItemTemplate {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: Option<String>,
    pub label: Option<String>,
    pub type_: Option<MenuItemType>,
    pub role: Option<MenuItemRole>,
    pub accelerator: Option<String>,
    pub enabled: Option<bool>,
    pub checked: Option<bool>,
    pub visible: Option<bool>,
    pub sublabel: Option<String>,
    pub tool_tip: Option<String>,
    pub submenu: Option<Vec<MenuItemTemplate>>,
}
impl PartialEq for MenuItemTemplate {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Menu.ts:55 (sha256:a4785e4dc1c6fb18b71613f188f2f655ee45a294d13e1340e36e34f57f6c3f8f)
#[derive(Clone)]
pub struct HostAppMenuCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroy: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub set_app_menu: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<MenuItemTemplate>) -> bool + Send + 'static>>,
    >,
}
impl PartialEq for HostAppMenuCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Menu.ts:63 (sha256:9110653bb300fdb38c18b499764c5bd0399a239a6195578d782eeaed2ab5cf5c)
#[derive(Clone)]
pub struct HostMenuHighlightCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostMenuHighlightCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Menu.ts:68 (sha256:415410f446fa63d16e445694f1fb1798ebb057345e0ba1213e105ccc914e02f1)
#[derive(Clone)]
pub struct HostMenuPopupCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub popup: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Vec<MenuItemTemplate>, f64, f64) -> crate::FlightTask<Option<String>>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostMenuPopupCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Menu.ts:74 (sha256:564c8122551bb36061704f84af2e7af51c78605898b0a7a82f6dfc96b2fb5a09)
#[derive(Clone)]
pub struct HostMenuSelectCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostMenuSelectCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Menu.ts:78 (sha256:c54f2ced1e99ae09330f56b63cd6ae4e1a7a86f607df99d5f72d0f31471162b1)
pub type ElectronMenuCapabilities = HostMenuCapabilities;

// Source: upstream/packages/types/src/Menu.ts:80 (sha256:9ca77113086dd3bbe473cfb3e57181dbd5c2e3ffcf4da56255b0bbe8cb2f0edd)
pub type TauriMenuCapabilities = HostMenuCapabilities;
