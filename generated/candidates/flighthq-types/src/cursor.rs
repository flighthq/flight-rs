// @generated from upstream/packages/types/src/Cursor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Cursor.ts:7 (sha256:4353882b4b070fe5d1fc0a7d8f65c65ddbd9d2cf332815a4848fc10597764380)
pub type Cursor = String;

// Source: upstream/packages/types/src/Cursor.ts:51 (sha256:348f7f02cc56aed2a81350c7fc1635f72cb24ec15939bdb9d98c3fb10ee42464)
#[derive(Clone)]
pub struct CursorBackend {
    pub set_cursor: crate::OpaqueHostValue,
}
