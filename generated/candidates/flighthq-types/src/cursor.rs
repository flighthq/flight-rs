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
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_cursor:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<Cursor>) -> () + Send + 'static>>>,
}
impl PartialEq for CursorBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
