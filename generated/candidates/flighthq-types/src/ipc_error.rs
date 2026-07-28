// @generated from upstream/packages/types/src/IpcError.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/IpcError.ts:2 (sha256:e708f01a5a4598faeb015e5ac675cced78abefb96080fb1fe0c55fcc3a4bbd1d)
pub type IpcErrorCode = String;

// Source: upstream/packages/types/src/IpcError.ts:9 (sha256:a834bd76744e4543841e55760be9ae588904ca076dfb9b013d692ed19da465d9)
#[derive(Clone)]
pub struct IpcError {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub code: IpcErrorCode,
    pub message: String,
    pub channel: String,
}
impl PartialEq for IpcError {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
