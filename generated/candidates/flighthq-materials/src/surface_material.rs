// @generated from upstream/packages/materials/src/surfaceMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_material;
use flighthq_types::{
    AlphaType, BLEND_MODE as blend_mode_constant, Kind, MaterialAlphaMode, SurfaceMaterial,
};

// Source: upstream/packages/materials/src/surfaceMaterial.ts:10 (sha256:62794450cefb8dd0245c91e8c65ad4cd160b2aeea9fa2618a6d233b71799737f)
pub fn create_surface_material(kind: Kind) -> SurfaceMaterial {
    let mut material = create_material((kind).clone());
    material.alpha_cutoff = DEFAULT_ALPHA_CUTOFF;
    material.alpha_mode = (DEFAULT_ALPHA_MODE).clone();
    material.alpha_type = (DEFAULT_ALPHA_TYPE).clone();
    material.blend_mode = blend_mode_constant.normal;
    material.double_sided = DEFAULT_DOUBLE_SIDED;
    return (material).clone();
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:23 (sha256:056d19166494396768e87c911a424b8a00bdeb02c45ec857e5e0b1e6bd1ec0e5)
pub fn get_material_alpha_mode(source: &SurfaceMaterial) -> MaterialAlphaMode {
    return (source.alpha_mode).clone();
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:29 (sha256:2a288ca0ec2e4e0b51b77f1aaceb482ee83c97a7c87d24554aa04f2f08f50271)
pub fn is_material_blended(source: &SurfaceMaterial) -> bool {
    return ((source.alpha_mode).clone() == "blend");
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:35 (sha256:d51465c8dbe13f734257a14268dbb2ffa2bd07d6ae37960068a1ecff2f4965de)
pub fn is_material_masked(source: &SurfaceMaterial) -> bool {
    return ((source.alpha_mode).clone() == "mask");
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:41 (sha256:dde4b9a6a811600ada6bbedf2fc665c56dd5b3828b81a48b5c5193142928b2f7)
pub fn is_material_opaque(source: &SurfaceMaterial) -> bool {
    return ((source.alpha_mode).clone() == "opaque");
}

// Source: upstream/packages/materials/src/surfaceMaterial.ts:45 (sha256:34e90c18bc19a9bc5f2510d91751864751ac7980e8301885b96e60f58aef72a0)
const DEFAULT_ALPHA_CUTOFF: f64 = 0.5_f64;

// Source: upstream/packages/materials/src/surfaceMaterial.ts:46 (sha256:401716d5b49444f6ce555a73a1459772cc9cab019086d5c70ea11ac13e42b06e)
const DEFAULT_ALPHA_MODE: MaterialAlphaMode = "opaque".to_owned();

// Source: upstream/packages/materials/src/surfaceMaterial.ts:47 (sha256:46e9b783476b033f39a58893eb73ff20671c4452b09ba8179ea734b5efa36c76)
const DEFAULT_ALPHA_TYPE: AlphaType = "straight".to_owned();

// Source: upstream/packages/materials/src/surfaceMaterial.ts:48 (sha256:59abc25abd5a8cbe5fa6822845633843c57e374dba44d086d0760fb0d39cd176)
const DEFAULT_DOUBLE_SIDED: bool = false;
