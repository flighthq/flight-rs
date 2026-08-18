// @generated from upstream/packages/types/src/TiledParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{TiledTilesetRef, TilemapData};

// Source: upstream/packages/types/src/TiledParseOptions.ts:7 (sha256:d5c484c50fb69b7acc29beb410109230c971f5a0635d9caccc7f687b1987c2d7)
pub type TiledCompression = String;

// Source: upstream/packages/types/src/TiledParseOptions.ts:13 (sha256:02d13b65552c0945c3e358c4cf33a2384d357924b4e0d2d16916056edb76bce4)
pub type TiledInflate = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Vec<u8>, TiledCompression) -> Option<Vec<u8>> + Send + 'static>>,
>;

// Source: upstream/packages/types/src/TiledParseOptions.ts:16 (sha256:89538f442435bd003e3abd5845f273d62794e3bd6134653ad51d3763715bd023)
#[derive(Clone, Default)]
pub struct TiledParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub inflate: Option<TiledInflate>,
}
impl PartialEq for TiledParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TiledParseOptions.ts:23 (sha256:ea67f9233041186cfe6c70cd371c7a50de1544da23ff92552f0036c92cd787a1)
pub type TiledTilesetResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(TiledTilesetRef) -> Option<TilemapData> + Send + 'static>>,
>;
