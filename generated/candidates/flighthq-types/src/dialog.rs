// @generated from upstream/packages/types/src/Dialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ApplicationWindow;

// Source: upstream/packages/types/src/Dialog.ts:4 (sha256:2da479e7ae1fd3e75c80d5433ee0ec659267e48f3e6c530727b2681c3a23eb86)
#[derive(Clone)]
pub struct FileDialogFilter {
    pub name: String,
    pub extensions: Vec<String>,
    pub mime_types: Option<Vec<String>>,
}

// Source: upstream/packages/types/src/Dialog.ts:16 (sha256:53c050ed33cc396f7b2b0a42259ba0d150ab85aaabfb793de56538c274ec258e)
#[derive(Clone)]
pub struct FileDialogHandle {
    pub kind: String,
    pub name: String,
    pub path: Option<String>,
}

// Source: upstream/packages/types/src/Dialog.ts:25 (sha256:d3b0f6b588195b67c2b586bb671f6bc578fab0f42962dbc84d5da66be79d68af)
pub type FileDialogStartIn = String;

// Source: upstream/packages/types/src/Dialog.ts:37 (sha256:3801250869a4c1c55a3128890cefb7369d0f25c402cc60f287db4f6360d29fed)
#[derive(Clone)]
pub struct OpenFileDialogOptions {
    pub title: Option<String>,
    pub multiple: Option<bool>,
    pub directory: Option<bool>,
    pub filters: Option<Vec<FileDialogFilter>>,
    pub default_path: Option<String>,
    pub start_in: Option<FileDialogStartIn>,
    pub parent_window: Option<ApplicationWindow>,
}

// Source: upstream/packages/types/src/Dialog.ts:51 (sha256:0745db5662ca681a7ad9ac1b1d3642dd232e8361479cc01426fef026c727999f)
#[derive(Clone)]
pub struct OpenDirectoryDialogOptions {
    pub title: Option<String>,
    pub multiple: Option<bool>,
    pub start_in: Option<FileDialogStartIn>,
    pub parent_window: Option<ApplicationWindow>,
}

// Source: upstream/packages/types/src/Dialog.ts:60 (sha256:470f1114705823515bcb88354fdc64b924b2c0e854fcdfa2d4375d73f34086c5)
#[derive(Clone)]
pub struct SaveFileDialogOptions {
    pub title: Option<String>,
    pub default_path: Option<String>,
    pub default_name: Option<String>,
    pub filters: Option<Vec<FileDialogFilter>>,
    pub start_in: Option<FileDialogStartIn>,
    pub parent_window: Option<ApplicationWindow>,
}

// Source: upstream/packages/types/src/Dialog.ts:73 (sha256:c35a79c2218c54c750521f58b14f64f0341ef515c4d2b68056657a667155fcff)
#[derive(Clone)]
pub struct PromptDialogOptions {
    pub title: Option<String>,
    pub message: String,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub parent_window: Option<ApplicationWindow>,
}

// Source: upstream/packages/types/src/Dialog.ts:82 (sha256:9869481c0caaab6137630affbecc226ba834f0bc836e73934890ca1036ec47f6)
pub type MessageDialogKind = String;

// Source: upstream/packages/types/src/Dialog.ts:84 (sha256:8e024b60092946fed8e1d754f27c433fca083f92173e92cf2e0a976b8aa5305b)
#[derive(Clone)]
pub struct MessageDialogOptions {
    pub title: Option<String>,
    pub message: String,
    pub detail: Option<String>,
    pub buttons: Option<Vec<String>>,
    pub kind: Option<MessageDialogKind>,
    pub checkbox_label: Option<String>,
    pub checkbox_checked: Option<bool>,
    pub default_id: Option<f64>,
    pub cancel_id: Option<f64>,
    pub parent_window: Option<ApplicationWindow>,
}

// Source: upstream/packages/types/src/Dialog.ts:104 (sha256:5955c19ad7714113252dfbab0942d980da86468e0febf98aaf04b3bfc2ce69c6)
#[derive(Clone)]
pub struct MessageDialogResult {
    pub button_index: f64,
    pub cancelled: bool,
    pub checkbox_checked: bool,
}

// Source: upstream/packages/types/src/Dialog.ts:113 (sha256:9183d35eb502e85bccf20fef2ad532cabbc56844f97f7973c96ddd5d2e1d9762)
#[derive(Clone)]
pub struct DialogBackend {
    pub confirm: crate::OpaqueHostValue,
    pub message: crate::OpaqueHostValue,
    pub open_directory: crate::OpaqueHostValue,
    pub open_file: crate::OpaqueHostValue,
    pub prompt: crate::OpaqueHostValue,
    pub save_file: crate::OpaqueHostValue,
}
