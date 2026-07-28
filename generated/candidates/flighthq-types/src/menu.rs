// @generated from upstream/packages/types/src/Menu.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::WellKnownMenuItemRoleValue;

// Source: upstream/packages/types/src/Menu.ts:10 (sha256:dfbdbbccb432e1c36c144fb2f1aace8a67468a363d8cc9d42ff46f7502fd8449)
pub type MenuItemType = String;

// Source: upstream/packages/types/src/Menu.ts:17 (sha256:7b2d8ece52899dd8323a11ea129b3d1a423606c124a9ecebb9edc4d05987cf04)
pub type MenuItemRole = crate::FlightUnion2<WellKnownMenuItemRoleValue, String>;

// Source: upstream/packages/types/src/Menu.ts:19 (sha256:49f11b531fed1394d5111d3dfa9d6f94036b76ec1cf83854645794700bb4419a)
#[derive(Clone)]
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
    pub submenu: Option<Vec<MenuItemTemplate>>,
}
impl PartialEq for MenuItemTemplate {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Menu.ts:34 (sha256:416b8b754887247676cd99dfa518e99708e4ec04d55ff90308da3c1004524620)
#[derive(Clone)]
pub struct MenuBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_application_menu: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<MenuItemTemplate>) -> bool + Send + 'static>>,
    >,
    pub popup_context_menu: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Vec<MenuItemTemplate>, f64, f64) -> crate::Promise<Option<String>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_select: std::sync::Arc<
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
impl PartialEq for MenuBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
