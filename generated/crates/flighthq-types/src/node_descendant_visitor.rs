// @generated from upstream/packages/types/src/NodeDescendantVisitor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Node;

// Source: upstream/packages/types/src/NodeDescendantVisitor.ts:6 (sha256:dd122c8a9c875b75b8dbe545aec34f25cf218ae86ec58957b96414297ce56b30)
pub type NodeDescendantVisitor =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node) -> bool + Send + 'static>>>;
