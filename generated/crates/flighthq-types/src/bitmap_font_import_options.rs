// @generated from upstream/packages/types/src/BitmapFontImportOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BitmapFont, BitmapFontFormatKind, BitmapFontParseOptions, ImportDiagnostic};

// Source: upstream/packages/types/src/BitmapFontImportOptions.ts:14 (sha256:f108a93acf89464e452853edc57a523eb8cba08a27d0b4ceda9d18a1e5157767)
#[derive(Clone)]
pub struct BitmapFontFormatEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detect: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<u8>) -> bool + Send + 'static>>>,
    pub parse: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Vec<u8>,
                        Option<BitmapFontParseOptions>,
                        Option<Vec<ImportDiagnostic>>,
                    ) -> Option<BitmapFont>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for BitmapFontFormatEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFontImportOptions.ts:29 (sha256:2a2aafc6d8f0b4ff76ce4fa2b284231c0535918ec439a01f549cf4bf54e0643a)
#[derive(Clone)]
pub struct BitmapFontFormatDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry: BitmapFontFormatEntry,
    pub kind: BitmapFontFormatKind,
}
impl PartialEq for BitmapFontFormatDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFontImportOptions.ts:51 (sha256:d230cd4f61020633eafc74ad4621d9a007e305455945195308208e649ed006f3)
#[derive(Clone, Default)]
pub struct BitmapFontImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub formats: Option<Vec<BitmapFontFormatDescriptor>>,
}
impl PartialEq for BitmapFontImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
