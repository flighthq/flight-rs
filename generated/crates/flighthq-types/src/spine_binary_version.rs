// @generated from upstream/packages/types/src/SpineBinaryVersion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, Skeleton2DImport};

// Source: upstream/packages/types/src/SpineBinaryVersion.ts:13 (sha256:beaccb91bce44776d5536b662552e3f32964ff55dd00b3e0b44e9fac60244ef3)
pub type SpineBinaryParser = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(Vec<u8>, Option<Vec<ImportDiagnostic>>) -> Option<Skeleton2DImport>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/SpineBinaryVersion.ts:28 (sha256:82fbf52e6cf059183db7b883614f706b463f3744ad2af04f0b7662f30a4d1f61)
pub type SpineBinaryVersionFailureReason = String;

// Source: upstream/packages/types/src/SpineBinaryVersion.ts:30 (sha256:2c4a66c54471a4c4641d35cec17aecb70fcfbdbdc61e22e20dd3771224749c9e)
#[derive(Clone, Default)]
pub struct SpineBinaryVersionFailure {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: f64,
    pub reason: SpineBinaryVersionFailureReason,
    pub v3_candidate: Option<String>,
    pub v4_candidate: Option<String>,
}
impl PartialEq for SpineBinaryVersionFailure {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
