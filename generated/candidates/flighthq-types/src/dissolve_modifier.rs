// @generated from upstream/packages/types/src/DissolveModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;

// Source: upstream/packages/types/src/DissolveModifier.ts:11 (sha256:b4447f68b4d80c5a7fc46ba4dfaedef76ea959785551545cb6cb49842f894138)
#[derive(Clone)]
pub struct DissolveModifier {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub slot: String,
    pub threshold: f64,
    pub edge_color: f64,
    pub edge_width: Option<f64>,
    pub map: Option<Texture>,
    pub scale: Option<f64>,
}
impl PartialEq for DissolveModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DissolveModifier.ts:21 (sha256:3bf663d2eb41c8257b420e470dff1f42d74906ed878b971bb68245fb58583590)
pub const DISSOLVE_MODIFIER_KIND: &'static str = "DissolveModifier";
