// @generated from upstream/packages/types/src/PixiParseResult.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, ParticleEmitterConfig};

// Source: upstream/packages/types/src/PixiParseResult.ts:4 (sha256:12362b4678837043a0bc033f1888d79d24b4b252cdfb3eb84a27511a29947993)
#[derive(Clone, Default)]
pub struct PixiParseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub diagnostics: Vec<ImportDiagnostic>,
}
impl PartialEq for PixiParseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PixiParseResult.ts:11 (sha256:9a517c27630cc9bb09f4147f906f627204e2f1aa61076c25f69b9e654b23dd2f)
pub type PixiParsed = PixiParseResult;
