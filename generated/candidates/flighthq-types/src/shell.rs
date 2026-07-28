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
    pub beep: crate::OpaqueHostValue,
    pub move_items_to_trash: crate::OpaqueHostValue,
    pub move_to_trash: crate::OpaqueHostValue,
    pub open_external: crate::OpaqueHostValue,
    pub open_path: crate::OpaqueHostValue,
    pub open_path_result: crate::OpaqueHostValue,
    pub read_shortcut_link: crate::OpaqueHostValue,
    pub show_item_in_folder: crate::OpaqueHostValue,
    pub write_shortcut_link: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Shell.ts:25 (sha256:9bc711b953691a63bb8c3020572a290feccf58dfc79e159500015cf6fc82847b)
#[derive(Clone)]
pub struct ShellOpenExternalOptions {
    pub activate: Option<bool>,
}

// Source: upstream/packages/types/src/Shell.ts:32 (sha256:504d357a093150c5ae4b689410873225c43e1c8ee3c614a21c5d689082fb1b76)
#[derive(Clone)]
pub struct ShellOpenPathOptions {
    pub application: Option<String>,
    pub working_directory: Option<String>,
}

// Source: upstream/packages/types/src/Shell.ts:39 (sha256:4c98c49be5de57273aa9d3cc5ea9b68fd36302b9c7efca25e2d585194a7a49f8)
#[derive(Clone)]
pub struct ShellShortcutLink {
    pub target: String,
    pub app_user_model_id: Option<String>,
    pub args: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub icon_index: Option<f64>,
    pub working_directory: Option<String>,
}

// Source: upstream/packages/types/src/Shell.ts:51 (sha256:38507f893c5bbb8ac211e8e7345d46b15e1359bd9e58e014b53b898861b3c665)
pub type ShellShortcutWriteOperation = String;
