// @generated from upstream/packages/types/src/TiledGid.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TiledGid.ts:5 (sha256:24fed34412a32b4a4ec7eb62f8605d0827907769a2c1dc6e641efe2b97808e4e)
#[derive(Clone, Default)]
pub struct TiledGid {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tile_id: f64,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub flip_diagonal: bool,
}
impl PartialEq for TiledGid {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
