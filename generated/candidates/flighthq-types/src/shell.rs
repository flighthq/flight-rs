// @generated from upstream/packages/types/src/Shell.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Shell.ts:6 (sha256:366a754393cf76c7a942f3c5e569c518fc1c288cc24b43046b4c94f586ecce52)
#[derive(Clone)]
pub struct ShellBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub beep: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub move_items_to_trash: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<String>) -> crate::Promise<Vec<bool>> + Send + 'static>>,
    >,
    pub move_to_trash: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub open_external: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Option<ShellOpenExternalOptions>) -> crate::Promise<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub open_path: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Option<ShellOpenPathOptions>) -> crate::Promise<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub open_path_result: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Option<ShellOpenPathOptions>) -> crate::Promise<String>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_shortcut_link: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::Promise<Option<ShellShortcutLink>> + Send + 'static>,
        >,
    >,
    pub show_item_in_folder: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>>,
    >,
    pub write_shortcut_link: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        ShellShortcutLink,
                        Option<ShellShortcutWriteOperation>,
                    ) -> crate::Promise<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ShellBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:25 (sha256:9bc711b953691a63bb8c3020572a290feccf58dfc79e159500015cf6fc82847b)
#[derive(Clone, Default)]
pub struct ShellOpenExternalOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub activate: Option<bool>,
}
impl PartialEq for ShellOpenExternalOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:32 (sha256:504d357a093150c5ae4b689410873225c43e1c8ee3c614a21c5d689082fb1b76)
#[derive(Clone, Default)]
pub struct ShellOpenPathOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub application: Option<String>,
    pub working_directory: Option<String>,
}
impl PartialEq for ShellOpenPathOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:39 (sha256:4c98c49be5de57273aa9d3cc5ea9b68fd36302b9c7efca25e2d585194a7a49f8)
#[derive(Clone, Default)]
pub struct ShellShortcutLink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub target: String,
    pub app_user_model_id: Option<String>,
    pub args: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub icon_index: Option<f64>,
    pub working_directory: Option<String>,
}
impl PartialEq for ShellShortcutLink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shell.ts:51 (sha256:38507f893c5bbb8ac211e8e7345d46b15e1359bd9e58e014b53b898861b3c665)
pub type ShellShortcutWriteOperation = String;
