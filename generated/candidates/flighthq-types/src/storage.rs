// @generated from upstream/packages/types/src/Storage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Storage.ts:7 (sha256:c33a09b6448a34364744537c372f980cfed7536946d2cd2a3bc2fb1186e03165)
#[derive(Clone)]
pub struct StorageBackend {
    pub get_item: crate::OpaqueHostValue,
    pub set_item: crate::OpaqueHostValue,
    pub remove_item: crate::OpaqueHostValue,
    pub clear: crate::OpaqueHostValue,
    pub keys: crate::OpaqueHostValue,
    pub byte_size: Option<crate::OpaqueHostValue>,
    pub subscribe_changes: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/Storage.ts:22 (sha256:02a0e1a37682e44dd655893df32418d5d3b9e6f907cfd7590a289256a8b7bae8)
#[derive(Clone)]
pub struct StorageChange {
    pub key: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
}

// Source: upstream/packages/types/src/Storage.ts:31 (sha256:96fa8963461679463dc0bfa2f65819759acab569e05f544e2a3527e40a2ea2cd)
#[derive(Clone)]
pub struct StorageMigration {
    pub migrate: crate::OpaqueHostValue,
    pub version: f64,
}

// Source: upstream/packages/types/src/Storage.ts:37 (sha256:7a96a1d7dd351cf083f20376c7f9e44aa41c631acd3d360ed8b32b29b141895d)
#[derive(Clone)]
pub struct StorageNamespace {
    pub prefix: String,
}

// Source: upstream/packages/types/src/Storage.ts:42 (sha256:1d2171ddf6a3843fac1e2d252f811586bcc5cea0a4a63ff26dca808185bf3ad4)
#[derive(Clone)]
pub struct StorageQuota {
    pub available: f64,
    pub used: f64,
}

// Source: upstream/packages/types/src/Storage.ts:48 (sha256:1fec9342f58e0a9e66498a7f10e0a57c0e229ef09ca01942053260aef72aa6c7)
#[derive(Clone)]
pub struct StorageSignals {
    pub on_change: Signal,
}
