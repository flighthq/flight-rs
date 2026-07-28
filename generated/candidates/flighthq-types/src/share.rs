// @generated from upstream/packages/types/src/Share.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ShareFile;

// Source: upstream/packages/types/src/Share.ts:8 (sha256:899bb63cf41c77e3ac2f6eb0b8bb40a741677907ea1d9a313634713185401ef3)
#[derive(Clone)]
pub struct ShareContent {
    pub title: Option<String>,
    pub text: Option<String>,
    pub url: Option<String>,
    pub files: Option<Vec<ShareFile>>,
}

// Source: upstream/packages/types/src/Share.ts:18 (sha256:a0ca18e785ad84e681b4558039731aab27c833c90dae6585aae3e9336c25e4e9)
#[derive(Clone)]
pub struct ShareOptions {
    pub chooser_title: Option<String>,
    pub excluded_activity_types: Option<Vec<String>>,
}

// Source: upstream/packages/types/src/Share.ts:28 (sha256:4ad57cec0c278f2921a223a6bdec550daf82a2c2bafa0e67d32f8f21150b757b)
#[derive(Clone)]
pub struct ShareResult {
    pub completed: bool,
    pub activity_type: Option<String>,
    pub dismissed: bool,
}

// Source: upstream/packages/types/src/Share.ts:34 (sha256:00d172569cb2b46c400978e64d87164d0b7ec57265a4771ab5c6523dc7091f8a)
#[derive(Clone)]
pub struct ShareBackend {
    pub is_available: crate::OpaqueHostValue,
    pub can_share: crate::OpaqueHostValue,
    pub share: crate::OpaqueHostValue,
    pub share_with_result: crate::OpaqueHostValue,
}
