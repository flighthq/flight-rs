// @generated from upstream/packages/types/src/SceneLights.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AmbientLight, DirectionalLight, HemisphereLight, PointLight, SpotLight};

// Source: upstream/packages/types/src/SceneLights.ts:16 (sha256:f5a941b32883e5ab9c05cc0ff252b4a844ac0e2c703c38a00e780617d0bff2e4)
#[derive(Clone)]
pub struct SceneLights {
    pub ambient: Option<AmbientLight>,
    pub directional: Option<DirectionalLight>,
    pub hemisphere: Option<Vec<HemisphereLight>>,
    pub point: Option<Vec<PointLight>>,
    pub spot: Option<Vec<SpotLight>>,
}
