// @generated from upstream/packages/types/src/HasBoundsRectangle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, NodeData, Rectangle};

// Source: upstream/packages/types/src/HasBoundsRectangle.ts:6 (sha256:0ecfccab9df1edf64e1fa4ad53d28a8ceedc0b71d2ef87d53af34349fd0640b4)
#[derive(Clone)]
pub struct HasBoundsRectangle {}

// Source: upstream/packages/types/src/HasBoundsRectangle.ts:8 (sha256:a19ae57a4710bd355d13a39e58afc3849abd973976923d88c704c6d487ab6e26)
#[derive(Clone)]
pub struct HasBoundsRectangleRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub compute_local_bounds_rectangle:
        std::sync::Arc<dyn Fn(Rectangle, BoundsNodeAny) -> () + Send + Sync + 'static>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
}

// Source: upstream/packages/types/src/HasBoundsRectangle.ts:15 (sha256:8fddb53399dce104a81ccb3922daaad01147fa43545a5ab9705329b6ee7c44f3)
#[derive(Clone)]
pub struct BoundsNode {
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
}

// Source: upstream/packages/types/src/HasBoundsRectangle.ts:16 (sha256:d5830d1d56f8a58a70a0fca0439a5f76e222af81496c1a33fa0321d3c45855c6)
#[derive(Clone)]
pub struct BoundsNodeAny {}

// Source: upstream/packages/types/src/HasBoundsRectangle.ts:18 (sha256:fd7d412c542ae8c3de6675e94131f1b2b5d5387ab2f73f83b3cb7b4f4fa7affe)
#[derive(Clone)]
pub struct Spatial2DNode {
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}
