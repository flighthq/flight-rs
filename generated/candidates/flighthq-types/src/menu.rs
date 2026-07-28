// @generated from upstream/packages/types/src/Menu.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Menu.ts:10 (sha256:dfbdbbccb432e1c36c144fb2f1aace8a67468a363d8cc9d42ff46f7502fd8449)
pub type MenuItemType = String;

// Source: upstream/packages/types/src/Menu.ts:17 (sha256:7b2d8ece52899dd8323a11ea129b3d1a423606c124a9ecebb9edc4d05987cf04)
pub type MenuItemRole = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Menu.ts:19 (sha256:49f11b531fed1394d5111d3dfa9d6f94036b76ec1cf83854645794700bb4419a)
#[derive(Clone)]
pub struct MenuItemTemplate {
    pub id: Option<String>,
    pub label: Option<String>,
    pub type_: Option<MenuItemType>,
    pub role: Option<MenuItemRole>,
    pub accelerator: Option<String>,
    pub enabled: Option<bool>,
    pub checked: Option<bool>,
    pub submenu: Option<Vec<MenuItemTemplate>>,
}

// Source: upstream/packages/types/src/Menu.ts:34 (sha256:416b8b754887247676cd99dfa518e99708e4ec04d55ff90308da3c1004524620)
#[derive(Clone)]
pub struct MenuBackend {
    pub set_application_menu: crate::OpaqueHostValue,
    pub popup_context_menu: crate::OpaqueHostValue,
    pub subscribe_select: crate::OpaqueHostValue,
}
