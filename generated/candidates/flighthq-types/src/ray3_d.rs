// @generated from upstream/packages/types/src/Ray3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3;

// Source: upstream/packages/types/src/Ray3D.ts:6 (sha256:2f17c1e22289ae4dfddeb7a4e65e66444a584a087a9bf46bf1fc9f71608132bf)
#[derive(Clone)]
pub struct Ray3D {
    pub direction: Vector3,
    pub origin: Vector3,
}

// Source: upstream/packages/types/src/Ray3D.ts:11 (sha256:e6ee73e975b2735820cab918774fb28604bcadb9e1d10cf323d8f5475cd2e802)
pub type Ray3DLike = Ray3D;
