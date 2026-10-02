// @generated from upstream/packages/types/src/HierarchyNode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node, NodeAny, NodeTraits};

// Source: upstream/packages/types/src/HierarchyNode.ts:8 (sha256:fd5b082cbc4963c16f70b0ac642a5c37bae250d16ca81573ce34b25e3c21bb45)
pub struct HierarchyNode<Traits = NodeTraits>(
    pub Node,
    pub core::marker::PhantomData<fn() -> (Traits,)>,
);
impl<Traits> Clone for HierarchyNode<Traits> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}

// Source: upstream/packages/types/src/HierarchyNode.ts:11 (sha256:cf435b753357f635375b1ba9ca1568e047b1c293c1c062ed1cc748a57b053483)
pub type HierarchyNodeAny = NodeAny;
