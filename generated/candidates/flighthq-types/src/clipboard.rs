// @generated from upstream/packages/types/src/Clipboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Clipboard.ts:2 (sha256:f807ed597514439692ed56f36de242e15a76aa6a22154813ff5cb12500150119)
#[derive(Clone)]
pub struct ClipboardBookmark {
    pub title: String,
    pub url: String,
}

// Source: upstream/packages/types/src/Clipboard.ts:9 (sha256:7386aa9e0d59fb35d0826be076149f4b1898c4e882d038d1f1e364735a201249)
#[derive(Clone)]
pub struct ClipboardWriteItem {
    pub format: String,
    pub data: String,
}

// Source: upstream/packages/types/src/Clipboard.ts:17 (sha256:ea92fef6d9d21df1a4e21bd00f296493760a7f632ed3d61cd48d7e0564b8c36f)
#[derive(Clone)]
pub struct ClipboardBackend {
    pub read_text: crate::OpaqueHostValue,
    pub write_text: crate::OpaqueHostValue,
    pub read_html: crate::OpaqueHostValue,
    pub write_html: crate::OpaqueHostValue,
    pub has_text: crate::OpaqueHostValue,
    pub read_image: crate::OpaqueHostValue,
    pub write_image: crate::OpaqueHostValue,
    pub has_image: crate::OpaqueHostValue,
    pub read_rtf: crate::OpaqueHostValue,
    pub write_rtf: crate::OpaqueHostValue,
    pub read_bookmark: crate::OpaqueHostValue,
    pub write_bookmark: crate::OpaqueHostValue,
    pub read_format: crate::OpaqueHostValue,
    pub write_format: crate::OpaqueHostValue,
    pub has_format: crate::OpaqueHostValue,
    pub get_formats: crate::OpaqueHostValue,
    pub read_items: crate::OpaqueHostValue,
    pub write_items: crate::OpaqueHostValue,
    pub read_files: crate::OpaqueHostValue,
    pub write_files: crate::OpaqueHostValue,
    pub clear: crate::OpaqueHostValue,
    pub get_change_count: crate::OpaqueHostValue,
    pub subscribe_clipboard_change: crate::OpaqueHostValue,
}
