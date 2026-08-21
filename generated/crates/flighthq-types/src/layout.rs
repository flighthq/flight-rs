// @generated from upstream/packages/types/src/Layout.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ViewportAlign;

// Source: upstream/packages/types/src/Layout.ts:3 (sha256:8fa4afdbda1b61e66bd23676c0935757f5f62bed41b55ab9f2973c81ebab842a)
pub const ANCHOR_LAYOUT_KIND: &'static str = "AnchorLayout";

// Source: upstream/packages/types/src/Layout.ts:4 (sha256:78ac9d7b604ec229238d55cc11847f49bf53512a3adc9f48970304789e3575fd)
pub const FLEX_LAYOUT_KIND: &'static str = "FlexLayout";

// Source: upstream/packages/types/src/Layout.ts:5 (sha256:46beb71aa4708c602012d34789ea2b89cd7f716b9c84703a3f27a84741888d02)
pub const GRID_LAYOUT_KIND: &'static str = "GridLayout";

// Source: upstream/packages/types/src/Layout.ts:9 (sha256:63a26878b0bb22b6fef9b109faf59b3056aabf9be2a3b2ea8844385549439a49)
#[derive(Clone, Default)]
pub struct AnchorLayoutItemStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<ViewportAlign>,
    pub bottom: Option<f64>,
    pub height: Option<f64>,
    pub left: Option<f64>,
    pub right: Option<f64>,
    pub top: Option<f64>,
    pub width: Option<f64>,
}
impl PartialEq for AnchorLayoutItemStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:19 (sha256:e07960052fe6504ce9409b6b164bed2f2f0f00dc7d41108eb8bd9f0c5e25e060)
pub type FlexLayoutAlign = String;

// Source: upstream/packages/types/src/Layout.ts:20 (sha256:99e9272dc835b0f30603fcfe6b94cf122f47653d781d7897fbcd5df93f35fec2)
pub type FlexLayoutDirection = String;

// Source: upstream/packages/types/src/Layout.ts:21 (sha256:554a2e3d9fee7a740c287eedd4eb2eb842a1491c42c34f8cbb12994a768d5f34)
pub type FlexLayoutJustify = String;

// Source: upstream/packages/types/src/Layout.ts:22 (sha256:e89c107ef4cfee7f7c95c00de9acaf64018686aea0f0bf9063f5cc91d69c6e01)
pub type FlexLayoutWrap = String;

// Source: upstream/packages/types/src/Layout.ts:24 (sha256:46e8abb038a23cff12aea9800be207d60137ecdd8be33be6c8f6af833a153eb9)
#[derive(Clone, Default)]
pub struct FlexLayoutContainerStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<FlexLayoutAlign>,
    pub direction: Option<FlexLayoutDirection>,
    pub gap: Option<f64>,
    pub justify: Option<FlexLayoutJustify>,
    pub padding_bottom: Option<f64>,
    pub padding_left: Option<f64>,
    pub padding_right: Option<f64>,
    pub padding_top: Option<f64>,
    pub wrap: Option<FlexLayoutWrap>,
}
impl PartialEq for FlexLayoutContainerStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:36 (sha256:5f78f38895f44208c6b9992633e77f10e15cd707cbd579918a67aa00701daa26)
#[derive(Clone, Default)]
pub struct FlexLayoutItemStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align_self: Option<crate::FlightUnion2<FlexLayoutAlign, String>>,
    pub basis: Option<crate::FlightUnion2<f64, String>>,
    pub grow: Option<f64>,
    pub shrink: Option<f64>,
}
impl PartialEq for FlexLayoutItemStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:43 (sha256:bd4c212d95fae6345541d2b9cf0d215ae231bb0064585c3057e7332e7042a5b9)
#[derive(Clone, Default)]
pub struct GridLayoutTrack {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub size: Option<f64>,
    pub fraction: Option<f64>,
}
impl PartialEq for GridLayoutTrack {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:48 (sha256:78050e9ba3748198db8d9131d8715a72bf672fea63c11235c39e823659d4cdf7)
#[derive(Clone, Default)]
pub struct GridLayoutContainerStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub column_gap: Option<f64>,
    pub columns: Vec<GridLayoutTrack>,
    pub padding_bottom: Option<f64>,
    pub padding_left: Option<f64>,
    pub padding_right: Option<f64>,
    pub padding_top: Option<f64>,
    pub row_gap: Option<f64>,
    pub rows: Vec<GridLayoutTrack>,
}
impl PartialEq for GridLayoutContainerStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:61 (sha256:2a53a77d506c3e24d49ddec40f9177f91887dcb95e9ec7f71474223753bca498)
#[derive(Clone, Default)]
pub struct GridLayoutItemStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub column: Option<f64>,
    pub column_span: Option<f64>,
    pub row: Option<f64>,
    pub row_span: Option<f64>,
}
impl PartialEq for GridLayoutItemStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:71 (sha256:dcb64afdbc4634db6a19bccd9b239a2d46272227ea1bcc4547bb450d8e95b91f)
#[derive(Clone)]
pub struct LayoutNode<ContainerStyle, ItemStyle> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub container_style: Option<ContainerStyle>,
    pub item_style: Option<ItemStyle>,
    pub kind: String,
    pub parent_index: f64,
}
impl<ContainerStyle, ItemStyle> Default for LayoutNode<ContainerStyle, ItemStyle> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            container_style: Default::default(),
            item_style: Default::default(),
            kind: Default::default(),
            parent_index: Default::default(),
        }
    }
}
impl<ContainerStyle, ItemStyle> PartialEq for LayoutNode<ContainerStyle, ItemStyle> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:78 (sha256:490d3123a670ddb1d15cd2cfd73da271b75c8e85cc7f1fe029718b2079329e7a)
#[derive(Clone, Default)]
pub struct LayoutTree {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub nodes: Vec<LayoutNode<crate::OpaqueHostValue, crate::OpaqueHostValue>>,
}
impl PartialEq for LayoutTree {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:82 (sha256:0821630a80a90e2af20ba65739a0ad0d9511311efa26dcd0cd33e28e257c2594)
#[derive(Clone, Default)]
pub struct LayoutResolutionFailureKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub intrinsic_sizes_too_small: String,
    pub invalid_container_style: String,
    pub invalid_hierarchy: String,
    pub invalid_item_style: String,
    pub output_too_small: String,
    pub unregistered_kind: String,
}
impl PartialEq for LayoutResolutionFailureKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static LAYOUT_RESOLUTION_FAILURE_KIND: std::sync::LazyLock<LayoutResolutionFailureKindValues> =
    std::sync::LazyLock::new(|| LayoutResolutionFailureKindValues {
        __flight_identity: std::sync::Arc::new(()),
        intrinsic_sizes_too_small: "IntrinsicSizesTooSmall".to_owned(),
        invalid_container_style: "InvalidContainerStyle".to_owned(),
        invalid_hierarchy: "InvalidHierarchy".to_owned(),
        invalid_item_style: "InvalidItemStyle".to_owned(),
        output_too_small: "OutputTooSmall".to_owned(),
        unregistered_kind: "UnregisteredKind".to_owned(),
    });

// Source: upstream/packages/types/src/Layout.ts:91 (sha256:38be69efdbc392021430c9e4352ffca57b72b0b7eea161b096692b2d7832f620)
pub type LayoutResolutionFailureKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Layout.ts:94 (sha256:074364b39a9696c6d18fa06a3cb02c2e638a943edf36025e32dda967c7ce4493)
#[derive(Clone, Default)]
pub struct LayoutResolutionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub actual_length: f64,
    pub kind: LayoutResolutionFailureKind,
    pub node_index: f64,
    pub parent_index: f64,
    pub required_length: f64,
    pub resolver_kind: Option<String>,
}
impl PartialEq for LayoutResolutionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Layout.ts:106 (sha256:76784cecd1937e4644466648d9bff88a1b2647c1ab971700e8fe17bdcdd065ee)
pub type LayoutResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    Vec<f32>,
                    LayoutTree,
                    Vec<f64>,
                    f64,
                    f64,
                ) -> Option<LayoutResolutionFailureKind>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/Layout.ts:114 (sha256:329847512bdbaad747ddcc9f54274c1ea393181260744e680cd0bcd7e6fcaed9)
pub type LayoutResolutionGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(LayoutResolutionExplanation) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Layout.ts:116 (sha256:7990b91753d362be27f86906395a45a7c19aae3b4001e7681dc88f6e8ca61d39)
#[derive(Clone, Default)]
pub struct LayoutState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub guard: Option<LayoutResolutionGuard>,
    pub last_failure_actual_length: f64,
    pub last_failure_kind: Option<LayoutResolutionFailureKind>,
    pub last_failure_node_index: f64,
    pub last_failure_parent_index: f64,
    pub last_failure_required_length: f64,
    pub last_failure_resolver_kind: Option<String>,
    pub resolvers: Vec<(String, LayoutResolver)>,
}
impl PartialEq for LayoutState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
