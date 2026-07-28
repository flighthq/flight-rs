// @generated from upstream/packages/types/src/Material.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/Material.ts:17 (sha256:a4c315d8130136ab5ad5c38e12d1cebca5388d52af388fbef9c000e2da59083d)
#[derive(Clone)]
pub struct Material {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
}
impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Material.ts:28 (sha256:2d9bc41a7f5dd0cf2749211f70bf48ac24a704328b0b4dcdf6886be10129a32a)
pub type MaterialLike = Material;

// Source: upstream/packages/types/src/Material.ts:33 (sha256:b9465a69e946e60e8c2ea2640de9071ff5e1c9af91cf72bebbadcf2518aeb9d3)
pub type MaterialData = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Material.ts:37 (sha256:d81a93c087ae25e6048244b67b4d7d4d4c8b49c603e0cff912eb30098d79d9cf)
pub const DEFAULT_MATERIAL_KIND: &'static str = "DefaultMaterial";
