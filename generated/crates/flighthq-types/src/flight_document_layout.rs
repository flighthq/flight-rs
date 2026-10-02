// @generated from upstream/packages/types/src/FlightDocumentLayout.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FlightDocumentFields, LayoutNode, NodeAny};

// Source: upstream/packages/types/src/FlightDocumentLayout.ts:7 (sha256:b56a9fb7feaa526ad16eb387f015ca8ecc48038844f6325562e63742662342d9)
pub type FlightDocumentLayoutNode = LayoutNode<FlightDocumentFields, FlightDocumentFields>;

// Source: upstream/packages/types/src/FlightDocumentLayout.ts:9 (sha256:8b774e6659062569e1899f8e4fc11f1596972d4995bb99097ed44745b4e36112)
#[derive(Clone, Default)]
pub struct FlightDocumentLayoutTree {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub nodes: Vec<FlightDocumentLayoutNode>,
}
impl PartialEq for FlightDocumentLayoutTree {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentLayout.ts:15 (sha256:80580c09e620fecd6bff8b5a344899391386a92edeb8f00d56e5cfd46d1ba894)
#[derive(Clone, Default)]
pub struct FlightDocumentLayoutDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub targets: Vec<String>,
    pub tree: FlightDocumentLayoutTree,
}
impl PartialEq for FlightDocumentLayoutDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentLayout.ts:22 (sha256:f0b0d62a5221ea7611bff3802e06dd464d940497cd632e9497a990e3417d09c3)
#[derive(Clone)]
pub struct FlightDocumentLayoutBinding<N = NodeAny> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub targets: Vec<N>,
    pub tree: FlightDocumentLayoutTree,
}
impl<N> Default for FlightDocumentLayoutBinding<N> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            targets: Default::default(),
            tree: Default::default(),
        }
    }
}
impl<N> PartialEq for FlightDocumentLayoutBinding<N> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
