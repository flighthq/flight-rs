// @generated from upstream/packages/types/src/Net.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Net.ts:14 (sha256:eb039680a4dddd5e6a6f4ab25bab4750fd8b6e4058ce3d46dc4abf3da6e796f8)
pub type NetMethod = String;

// Source: upstream/packages/types/src/Net.ts:18 (sha256:eb4e529d4803349ca0e0fb3d73bd4a7a6530ca9a942e140629ad81bc795abd18)
pub type NetResponseType = String;

// Source: upstream/packages/types/src/Net.ts:22 (sha256:48d2fd7febbca35c1e57558573aa3d2586f3588e4d63fd53765b2405c4332f49)
pub type NetCredentials = String;

// Source: upstream/packages/types/src/Net.ts:26 (sha256:3004a874b806a5369ba27c32bba3ba16805ff15905fab87782b0e3f2ed6583ea)
pub type NetRedirect = String;

// Source: upstream/packages/types/src/Net.ts:31 (sha256:1ca3dd765bb3e660c1884ccb0867d8a083385852cbd2ae4af2231c30242843cf)
pub type NetBody = Option<crate::FlightUnion2<String, crate::FlightUnion2<Vec<u8>, Vec<u8>>>>;

// Source: upstream/packages/types/src/Net.ts:36 (sha256:59733359f481dfa821da4ce87b07778f91fc1337a52446c722f1a5611a54d2e4)
pub type NetResponseBody = Option<
    crate::FlightUnion2<
        String,
        crate::FlightUnion2<
            crate::FlightValue,
            crate::FlightUnion2<Vec<u8>, crate::OpaqueHostValue>,
        >,
    >,
>;

// Source: upstream/packages/types/src/Net.ts:40 (sha256:be5f077631722591406184fa65398af48876fb3c6e8c82d3a3da4cc352c434e7)
#[derive(Clone, Default)]
pub struct NetRequest {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub method: NetMethod,
    pub url: String,
    pub headers: Option<Vec<(String, String)>>,
    pub body: Option<NetBody>,
    pub response_type: Option<NetResponseType>,
    pub timeout_ms: Option<f64>,
    pub credentials: Option<NetCredentials>,
    pub redirect: Option<NetRedirect>,
}
impl PartialEq for NetRequest {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Net.ts:54 (sha256:d840f17b5a2203b069d8f46eaba1eebd3ce5c55ffc9ad7c3b62c835a25bbea26)
#[derive(Clone, Default)]
pub struct NetGuardNotice {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub operation: String,
    pub reason: String,
    pub request: NetRequest,
}
impl PartialEq for NetGuardNotice {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Net.ts:60 (sha256:4930dccb24d81259ef9c93ccd68815e0d0f0c68ded3a9a7b8044f3f44b506acc)
pub type NetGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(NetGuardNotice) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/Net.ts:65 (sha256:9f072cdbc05e1b6f6f23c2f526fb60b394c28f9e5d14f3963903e565423c1d32)
#[derive(Clone, Default)]
pub struct NetResponse {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: f64,
    pub status_text: String,
    pub ok: bool,
    pub headers: Vec<(String, String)>,
    pub body: NetResponseBody,
    pub url: String,
}
impl PartialEq for NetResponse {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Net.ts:78 (sha256:53dffe299d668940c8c9b804c0ed55d003cb3a98e693fb4f463c2cd2253f3cc8)
#[derive(Clone, Default)]
pub struct NetResponseExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub status: f64,
    pub status_text: String,
    pub url: String,
}
impl PartialEq for NetResponseExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Net.ts:89 (sha256:d44c2fab2668dd3124b15bf357de97a81465504c11e7245ada6850a71efa8c76)
#[derive(Clone, Default)]
pub struct NetProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub phase: String,
    pub loaded: f64,
    pub total: f64,
}
impl PartialEq for NetProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Net.ts:97 (sha256:eb3efc65ebc51bd352306b2bce221f8fb7d60e77e6b7ffda364ee0a9ec7268db)
#[derive(Clone, Default)]
pub struct NetRequestOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub progress: Option<
        Signal<
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(NetProgress) -> () + Send + 'static>>>,
        >,
    >,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for NetRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Net.ts:104 (sha256:f960336ed8bdd91555ed1b96148ad4f2a892ea786d876a7be087769381f04161)
#[derive(Clone)]
pub struct HostNetCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub send_net_request: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(NetRequest, Option<NetRequestOptions>) -> crate::FlightTask<NetResponse>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostNetCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
