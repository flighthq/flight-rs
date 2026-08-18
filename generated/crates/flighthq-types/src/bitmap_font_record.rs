// @generated from upstream/packages/types/src/BitmapFontRecord.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BitmapFontEncoding;

// Source: upstream/packages/types/src/BitmapFontRecord.ts:7 (sha256:182ceaddee1847336c2926bdbcbd220df1fad5ad725ec3b0cc21f45dd02da713)
#[derive(Clone, Default)]
pub struct BitmapFontCharRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub id: f64,
    pub page: f64,
    pub width: f64,
    pub x: f64,
    pub xadvance: f64,
    pub xoffset: f64,
    pub y: f64,
    pub yoffset: f64,
}
impl PartialEq for BitmapFontCharRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFontRecord.ts:21 (sha256:4f116fa26e71d8730c5448413eb7005c1f0f4539e88c6d0da93c80091aa64f8d)
#[derive(Clone, Default)]
pub struct BitmapFontKerningRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub amount: f64,
    pub first: f64,
    pub second: f64,
}
impl PartialEq for BitmapFontKerningRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFontRecord.ts:29 (sha256:90e36dee9bbb4651c12cb4465b07b9039fbe398c4ca8975e052c6179e700355c)
#[derive(Clone, Default)]
pub struct BitmapFontPageRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub file: String,
    pub id: f64,
}
impl PartialEq for BitmapFontPageRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFontRecord.ts:39 (sha256:ae30b6e490fa58d911ccf7d6ad417734cdf4bcb81f934892511ccb503ffd7231)
#[derive(Clone, Default)]
pub struct BitmapFontRecord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub base: f64,
    pub chars: Vec<BitmapFontCharRecord>,
    pub encoding: BitmapFontEncoding,
    pub kernings: Vec<BitmapFontKerningRecord>,
    pub line_height: f64,
    pub pages: Vec<BitmapFontPageRecord>,
}
impl PartialEq for BitmapFontRecord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
