// @generated from upstream/packages/types/src/BitmapReadback.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapReadback.ts:5 (sha256:651aed3a009ea43863d84fccbd5c0a607e6186e76651e1d95e99197a4ab0b666)
#[derive(Clone, Default)]
pub struct BitmapReadbackExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub readable: bool,
    pub reason: BitmapReadbackBlockReason,
}
impl PartialEq for BitmapReadbackExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapReadback.ts:20 (sha256:3fd2efe803cbac6644d4c5a5ac54dc286fc519cc708e87c2ed2926cda30ea987)
pub type BitmapReadbackBlockReason = String;
