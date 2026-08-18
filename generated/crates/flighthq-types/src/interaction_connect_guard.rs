// @generated from upstream/packages/types/src/InteractionConnectGuard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{InteractionSignalName, NodeAny};

// Source: upstream/packages/types/src/InteractionConnectGuard.ts:4 (sha256:518985b1ea4c12d1a0216c2beddfa3e567aec31ac64ad7d92abeb617ee116474)
pub type InteractionConnectGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(NodeAny, InteractionSignalName) -> () + Send + 'static>>,
>;
