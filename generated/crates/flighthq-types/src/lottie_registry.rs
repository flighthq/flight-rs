// @generated from upstream/packages/types/src/LottieRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AdvancedBlendMode, AnimationChannel, DisplayObject, ImageResource, ImportDiagnostic,
    LottieAsset, LottieDocument, LottieImageAsset, LottieLayer, LottieMask, LottieShapeItem,
    Node2D, Path, Shape,
};

// Source: upstream/packages/types/src/LottieRegistry.ts:18 (sha256:756005cae739f01d3f8496898c8e3f854d8184b8053e2c85967fef1bf0ffb72f)
pub struct LottieLayerKind;
impl LottieLayerKind {
    pub const Image: f64 = 2.0_f64;
    pub const Null: f64 = 3.0_f64;
    pub const Precomposition: f64 = 0.0_f64;
    pub const Shape: f64 = 4.0_f64;
    pub const Solid: f64 = 1.0_f64;
    pub const Text: f64 = 5.0_f64;
}

// Source: upstream/packages/types/src/LottieRegistry.ts:27 (sha256:e5ff8e9d8d14d64793a8f8de22726979f530f2e134086069ca3d07c50f68e5b1)
// TypeScript numeric namespace LottieLayerKind is represented by its generated Rust constants.

// Source: upstream/packages/types/src/LottieRegistry.ts:29 (sha256:5d478ec4a1c1a4011d7af2ad972fe8e93ee37bc667db3bb44452e9f491a1dae4)
#[derive(Clone, Default)]
pub struct LottieShapeItemKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ellipse: String,
    pub fill: String,
    pub gradient_fill: String,
    pub gradient_stroke: String,
    pub path: String,
    pub polystar: String,
    pub rectangle: String,
    pub stroke: String,
    pub trim_path: String,
}
impl PartialEq for LottieShapeItemKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static LOTTIE_SHAPE_ITEM_KIND: std::sync::LazyLock<LottieShapeItemKindValues> =
    std::sync::LazyLock::new(|| LottieShapeItemKindValues {
        __flight_identity: std::sync::Arc::new(()),
        ellipse: "el".to_owned(),
        fill: "fl".to_owned(),
        gradient_fill: "gf".to_owned(),
        gradient_stroke: "gs".to_owned(),
        path: "sh".to_owned(),
        polystar: "sr".to_owned(),
        rectangle: "rc".to_owned(),
        stroke: "st".to_owned(),
        trim_path: "tm".to_owned(),
    });

// Source: upstream/packages/types/src/LottieRegistry.ts:41 (sha256:d306839e17829e035df33a2754c459cf34fb46c23e882ee322edf692165449b9)
pub type LottieShapeItemKind = String;

// Source: upstream/packages/types/src/LottieRegistry.ts:50 (sha256:9193683c91150366994ade264042d851498db4c2e771c86130ef6c72000339b5)
#[derive(Clone, Default)]
pub struct LottieMaskKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub additive: String,
    pub darken: String,
    pub difference: String,
    pub intersect: String,
    pub lighten: String,
    pub none: String,
    pub subtract: String,
}
impl PartialEq for LottieMaskKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static LOTTIE_MASK_KIND: std::sync::LazyLock<LottieMaskKindValues> =
    std::sync::LazyLock::new(|| LottieMaskKindValues {
        __flight_identity: std::sync::Arc::new(()),
        additive: "a".to_owned(),
        darken: "d".to_owned(),
        difference: "f".to_owned(),
        intersect: "i".to_owned(),
        lighten: "l".to_owned(),
        none: "n".to_owned(),
        subtract: "s".to_owned(),
    });

// Source: upstream/packages/types/src/LottieRegistry.ts:60 (sha256:5f6970204baebb0141ce8c3a100adb4e41ba3f1d338a8a4e84792ee74e7d201e)
pub type LottieMaskKind = String;

// Source: upstream/packages/types/src/LottieRegistry.ts:62 (sha256:d29cc295b5c1d4b844ba0299e7e06a6e416938488922051f1738dbcf4014ad46)
#[derive(Clone, Default)]
pub struct LottieAdvancedBlend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: AdvancedBlendMode,
    pub node: DisplayObject,
}
impl PartialEq for LottieAdvancedBlend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:67 (sha256:7f514937bdaaf8467512c0e0a0ac9defbb4d39b440228225570cc2cfdc8776db)
#[derive(Clone, Default)]
pub struct LottieFillPaint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Vec<f64>,
    pub kind: String,
    pub opacity: f64,
    pub winding: String,
}
impl PartialEq for LottieFillPaint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:74 (sha256:a7c0d0b8f78cee3085a75f53d54025c0e63db729db6bfbe263cb0b53e6053fa6)
#[derive(Clone, Default)]
pub struct LottieStrokePaint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub caps: String,
    pub color: Vec<f64>,
    pub dash: Vec<f64>,
    pub dash_offset: f64,
    pub joints: String,
    pub kind: String,
    pub miter_limit: f64,
    pub opacity: f64,
    pub width: f64,
}
impl PartialEq for LottieStrokePaint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:86 (sha256:3de8c5b6bf3991cecb849dc84f305bc3b21b0ec7219840a2f4163e2a28886f5a)
#[derive(Clone, Default)]
pub struct LottieGradientPaint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub caps: String,
    pub count: f64,
    pub dash: Vec<f64>,
    pub dash_offset: f64,
    pub end: Vec<f64>,
    pub joints: String,
    pub kind: String,
    pub miter_limit: f64,
    pub opacity: f64,
    pub shape: f64,
    pub start: Vec<f64>,
    pub type_: String,
    pub values: Vec<f64>,
    pub width: f64,
    pub winding: String,
}
impl PartialEq for LottieGradientPaint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:104 (sha256:788ec94cc43e4c93b4bd53e14c3b9c2ee4ece2dd420a2830b65700bd648276c4)
pub type LottiePaint = crate::FlightUnion2<
    LottieFillPaint,
    crate::FlightUnion2<LottieGradientPaint, LottieStrokePaint>,
>;

// Source: upstream/packages/types/src/LottieRegistry.ts:115 (sha256:13f48d8e08b9adfda01f9ae402860446fa75ba8ba20946960bf18936fa19d3f2)
pub type LottieShapePainter =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Shape, Vec<Path>) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/LottieRegistry.ts:127 (sha256:47f4c3511e71a57ecd4a614d72626f91a41936080f4103971f283f72a2bb7928)
pub type LottieShapeModifier =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<Path>) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/LottieRegistry.ts:129 (sha256:eafb30e8019c3aabf5b0bff2a6fa0d99f2b553e50e135ec8513646470b8b6865)
#[derive(Clone, Default)]
pub struct LottieImportContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advanced_blends: Vec<LottieAdvancedBlend>,
    pub assets: Vec<(String, LottieAsset)>,
    pub channels: Vec<AnimationChannel>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub document: LottieDocument,
    pub frame_offset: f64,
    pub frame_scale: f64,
    pub registry: LottieRegistry,
    pub resolve_image_resource: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(LottieImageAsset) -> Option<ImageResource> + Send + 'static>,
            >,
        >,
    >,
    pub resolving_precompositions: Vec<String>,
}
impl PartialEq for LottieImportContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:142 (sha256:1ccd2f495e475af9d34153b112ed8d5e19071629dd1fbbb685802531e4eb4795)
#[derive(Clone, Default)]
pub struct LottieLayerContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub container: DisplayObject,
    pub import: LottieImportContext,
    pub layer: LottieLayer,
}
impl PartialEq for LottieLayerContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:148 (sha256:0afd801ff242fb5227a8075183177a7d3f562af953a31d6b034bc4f2c7d332eb)
pub type LottieLayerHandler =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LottieLayerContext) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/LottieRegistry.ts:150 (sha256:7759263e067cbb85f04ed1a49a4a12331cea93a6699a0eef3fc6558f198cc0ee)
#[derive(Clone)]
pub struct LottieLayerHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: LottieLayerHandler,
    pub kind: f64,
}
impl PartialEq for LottieLayerHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:155 (sha256:f044e291feee725a7e74f755722908e8b51f9cec0f098a6ec4881c418a007130)
#[derive(Clone, Default)]
pub struct LottieMaskContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub import: LottieImportContext,
    pub masks: Vec<LottieMask>,
    pub target: Node2D,
}
impl PartialEq for LottieMaskContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:162 (sha256:ab4785447520852de9d8fe78cf1523cc1e4cd190dedd63975167ea08b5dbfffb)
pub type LottieMaskHandler =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LottieMaskContext) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/LottieRegistry.ts:164 (sha256:c6228ef2722d476e49f0267f5e2a749bf8743cbc11b906b7be1236fa836b9e68)
#[derive(Clone)]
pub struct LottieMaskHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: LottieMaskHandler,
    pub kind: LottieMaskKind,
}
impl PartialEq for LottieMaskHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:170 (sha256:0a65d005b91b7a4f9277045efef3b6b3caa8efaca076228f23218f7af48921cf)
#[derive(Clone)]
pub struct LottieMutableAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub lottie_apply: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::FlightUnion2<Vec<f64>, Vec<f32>>, f64) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for LottieMutableAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:174 (sha256:fb7d6e144c55b1e927210f4468278038d5ba7344c809a6d97f79d42cb5dfcf16)
#[derive(Clone)]
pub struct LottieShapeItemContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub claims: Vec<String>,
    pub import: LottieImportContext,
    pub item: LottieShapeItem,
    pub modifiers: Vec<LottieShapeModifier>,
    pub painters: Vec<LottieShapePainter>,
    pub paths: Vec<Path>,
    pub rerender: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub shape: Shape,
}
impl PartialEq for LottieShapeItemContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:193 (sha256:d9c419e40b6a5420d403a61bca641727efc20af12a5820c316edcbed66bc2b8b)
pub type LottieShapeItemHandler =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(LottieShapeItemContext) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/LottieRegistry.ts:195 (sha256:f25a64a532d3af5f71a9e85c0c834af85b9bfe30c2f4e1704e71d8c9c171c606)
#[derive(Clone)]
pub struct LottieShapeItemHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: LottieShapeItemHandler,
    pub kind: LottieShapeItemKind,
}
impl PartialEq for LottieShapeItemHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieRegistry.ts:200 (sha256:9efd2b1ffe81d001f13d24eaebd38d9ebbf2e19c89d8b4d9adab80d39b962bce)
#[derive(Clone, Default)]
pub struct LottieRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub layer_handlers: Vec<LottieLayerHandlerEntry>,
    pub mask_handlers: Vec<LottieMaskHandlerEntry>,
    pub shape_item_handlers: Vec<LottieShapeItemHandlerEntry>,
}
impl PartialEq for LottieRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
