// @generated from upstream/packages/types/src/TilemapImportOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, TiledMap, TiledParseOptions, TiledTileset, TilemapFormatKind};

// Source: upstream/packages/types/src/TilemapImportOptions.ts:15 (sha256:fb2f71790c282cfcc4096c497cb0f8917c6cf4ebc18f4740a7fa4aae17b51ecf)
#[derive(Clone)]
pub struct TilemapFormatEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detect: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub parse: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Option<TiledParseOptions>,
                        Option<Vec<ImportDiagnostic>>,
                    ) -> Option<TiledMap>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TilemapFormatEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TilemapImportOptions.ts:21 (sha256:b2f8f7ad6690dc7c6f3eb314beb2a921991a52280785789d14c5550c4de7d8f2)
#[derive(Clone)]
pub struct TilemapFormatDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry: TilemapFormatEntry,
    pub kind: TilemapFormatKind,
}
impl PartialEq for TilemapFormatDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TilemapImportOptions.ts:27 (sha256:2a978d71a1cc20a9d391bc532e26afacea6dd9c84cb1bdabef91249352ff4e9c)
#[derive(Clone)]
pub struct TilesetFormatEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detect: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub parse: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Option<TiledParseOptions>,
                        Option<Vec<ImportDiagnostic>>,
                    ) -> Option<TiledTileset>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TilesetFormatEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TilemapImportOptions.ts:33 (sha256:e3ec79f7f203a25d6a6555ba1abe69fb29b2976c19e13b279ce1a61cbae27a41)
#[derive(Clone)]
pub struct TilesetFormatDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entry: TilesetFormatEntry,
    pub kind: TilemapFormatKind,
}
impl PartialEq for TilesetFormatDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TilemapImportOptions.ts:57 (sha256:27b3957f9f3615f355b99008be671f4555e02e24314443b1461a818664f08cb6)
#[derive(Clone, Default)]
pub struct TilemapImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub map_formats: Option<Vec<TilemapFormatDescriptor>>,
    pub tileset_formats: Option<Vec<TilesetFormatDescriptor>>,
}
impl PartialEq for TilemapImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
