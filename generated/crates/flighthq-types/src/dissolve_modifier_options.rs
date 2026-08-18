// @generated from upstream/packages/types/src/DissolveModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;

// Source: upstream/packages/types/src/DissolveModifierOptions.ts:3 (sha256:877e24a08322880ba5714aa0116f27f119d937d476e1922d21694b5d5bb03c36)
#[derive(Clone, Default)]
pub struct DissolveModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub threshold: f64,
    pub edge_color: Option<f64>,
    pub edge_width: Option<f64>,
    pub map: Option<Texture>,
    pub scale: Option<f64>,
}
impl PartialEq for DissolveModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
