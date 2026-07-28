// @generated from upstream/packages/types/src/Protocol.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Protocol.ts:5 (sha256:ba077607db282b1c9f6ca90eea8515cebe470d4f5183c598f62a10cbc8ebb69a)
#[derive(Clone)]
pub struct ParsedProtocolUrl {
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub query: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/Protocol.ts:13 (sha256:2d288a88ead2d63a4a3e411b5af491389dcf8c94533a2e10283bbccb3130eeb3)
#[derive(Clone)]
pub struct ProtocolHandler {
    pub on_open_url: Signal,
}

// Source: upstream/packages/types/src/Protocol.ts:19 (sha256:86af5604996e632e93cc2ec18f11e0d76a6c0f3f1fd33e62174b2646b1e89d79)
#[derive(Clone)]
pub struct ProtocolBackend {
    pub register: crate::OpaqueHostValue,
    pub unregister: crate::OpaqueHostValue,
    pub is_registered: crate::OpaqueHostValue,
    pub get_registered_schemes: crate::OpaqueHostValue,
    pub set_as_default: crate::OpaqueHostValue,
    pub is_default: crate::OpaqueHostValue,
    pub remove_as_default: crate::OpaqueHostValue,
    pub get_launch_url: crate::OpaqueHostValue,
    pub drain_pending_urls: crate::OpaqueHostValue,
    pub subscribe: crate::OpaqueHostValue,
}
