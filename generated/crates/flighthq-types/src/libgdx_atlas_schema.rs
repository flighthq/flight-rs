// @generated from upstream/packages/types/src/LibgdxAtlasSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/LibgdxAtlasSchema.ts:6 (sha256:22db67e940d060324913253356f098a7dd82a0e7a9f5bc92a79e28b6ec01ccdc)
#[derive(Clone, Default)]
pub struct LibgdxAtlasDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pages: Vec<LibgdxAtlasPage>,
}
impl PartialEq for LibgdxAtlasDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxAtlasSchema.ts:10 (sha256:606eda33a79d4274dfa72a88b09fae37f8e9ef68d059a9c623d8bb824f9e79d7)
#[derive(Clone, Default)]
pub struct LibgdxAtlasPage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub filter_mag: String,
    pub filter_min: String,
    pub format: String,
    pub image_file: String,
    pub regions: Vec<LibgdxAtlasRegion>,
    pub repeat: String,
    pub size: Vec<f64>,
}
impl PartialEq for LibgdxAtlasPage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxAtlasSchema.ts:27 (sha256:743a7fe9e1d8bbc41889180a4437b96802a34e91dcee2cbc3c7a020f6a849d61)
#[derive(Clone, Default)]
pub struct LibgdxAtlasRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index: f64,
    pub name: String,
    pub offset: Vec<f64>,
    pub orig: Vec<f64>,
    pub orig_size: Vec<f64>,
    pub rotate: bool,
    pub size: Vec<f64>,
    pub xy: Vec<f64>,
}
impl PartialEq for LibgdxAtlasRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LibgdxAtlasSchema.ts:46 (sha256:d17e42d33d123685687a764165cce1819d9a4e3bb36164f637f22bf8b4d4691a)
#[derive(Clone, Default)]
pub struct LibgdxAtlasParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame_duration: Option<f64>,
}
impl PartialEq for LibgdxAtlasParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
