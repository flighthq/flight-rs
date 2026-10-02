// @generated from upstream/packages/types/src/SvgRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ImportDiagnostic, Matrix, Node2D, PathWinding, Rectangle, SpreadMethod,
    SvgDocumentImportOptions, XmlElement,
};

// Source: upstream/packages/types/src/SvgRegistry.ts:9 (sha256:839f9259c83107b51ad5bcda0ccd16e0f2da6bb1f02c716ab5c17706dda24c5f)
#[derive(Clone, Default)]
pub struct SvgElementKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub container: String,
    pub geometry: String,
    pub image: String,
    pub text: String,
    pub use_: String,
}
impl PartialEq for SvgElementKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SVG_ELEMENT_KIND: std::sync::LazyLock<SvgElementKindValues> =
    std::sync::LazyLock::new(|| SvgElementKindValues {
        __flight_identity: std::sync::Arc::new(()),
        container: "container".to_owned(),
        geometry: "geometry".to_owned(),
        image: "image".to_owned(),
        text: "text".to_owned(),
        use_: "use".to_owned(),
    });

// Source: upstream/packages/types/src/SvgRegistry.ts:17 (sha256:7fded72b3d07466e72b6d4660288dc42d82b3c17eb41b594c3ffbcb344c9bc7d)
pub type SvgElementKind = String;

// Source: upstream/packages/types/src/SvgRegistry.ts:29 (sha256:01037ad526d61fce02c1a9347d890de5256e6f79b2bec0865e2f10b8ca05dae5)
#[derive(Clone, Default)]
pub struct SvgClipKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub path: String,
}
impl PartialEq for SvgClipKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SVG_CLIP_KIND: std::sync::LazyLock<SvgClipKindValues> =
    std::sync::LazyLock::new(|| SvgClipKindValues {
        __flight_identity: std::sync::Arc::new(()),
        path: "clip-path".to_owned(),
    });

// Source: upstream/packages/types/src/SvgRegistry.ts:33 (sha256:6b961de04ddd0b5d34d8087bbadaa50d74e69ca1fc3087713c1eaef6e4b4e164)
pub type SvgClipKind = String;

// Source: upstream/packages/types/src/SvgRegistry.ts:35 (sha256:2d9a4f89677ea364b29d2e640d56acf90a3156db545684935d056819369a7bde)
#[derive(Clone, Default)]
pub struct SvgClipContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub element: XmlElement,
    pub import: SvgImportContext,
    pub target: Node2D,
}
impl PartialEq for SvgClipContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:41 (sha256:1853c687fa70f06026a7cc06e1ff1d13e36a0b56c20eaec3763b4fb104894f66)
pub type SvgClipHandler =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(SvgClipContext) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/SvgRegistry.ts:43 (sha256:40e30e9cd720396b26203dfb8686290be783f9cb1698724a20758df1778a1252)
#[derive(Clone)]
pub struct SvgClipHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: SvgClipHandler,
    pub kind: SvgClipKind,
}
impl PartialEq for SvgClipHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:48 (sha256:d903e8dd1a023dc2fc4ea575b56f8c5e0f9bfc574a1317665bb928dda73b09bc)
#[derive(Clone, Default)]
pub struct SvgColor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub rgb: f64,
}
impl PartialEq for SvgColor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:53 (sha256:6cd449d70b59804a577f1d9d202ae98000046c575fb4e057766c75e6f88d3294)
#[derive(Clone, Default)]
pub struct SvgCssRule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub declarations: Vec<(String, String)>,
    pub order: f64,
    pub selector: String,
    pub specificity: f64,
}
impl PartialEq for SvgCssRule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:60 (sha256:7dc55df62470e29bde13358ed574283cd6f3275a94547c87cdc1ec7d4d60d85f)
#[derive(Clone, Default)]
pub struct SvgElementContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub element: XmlElement,
    pub import: SvgImportContext,
    pub parent_style: SvgStyle,
}
impl PartialEq for SvgElementContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:66 (sha256:d2e518c05c2416afa1c1bd0679092358a3ca5435319a269553949e98d1a635ea)
pub type SvgElementHandler = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(SvgElementContext) -> Option<Node2D> + Send + 'static>>,
>;

// Source: upstream/packages/types/src/SvgRegistry.ts:68 (sha256:03d18b4fbe797112a5cdd0ab9d4710b02dd20c81573816f47bb55f7cef22ab72)
#[derive(Clone)]
pub struct SvgElementHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: SvgElementHandler,
    pub kind: SvgElementKind,
}
impl PartialEq for SvgElementHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:73 (sha256:af1039a2d3fbe22674f29208a7a5174a034452b58930414ff5f12fd0a4c15b59)
#[derive(Clone, Default)]
pub struct SvgGradient {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cx: f64,
    pub cy: f64,
    pub fx: f64,
    pub fy: f64,
    pub kind: String,
    pub radius: f64,
    pub spread_method: SpreadMethod,
    pub stops: Vec<SvgGradientStop>,
    pub transform: Option<Matrix>,
    pub units: String,
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
}
impl PartialEq for SvgGradient {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:90 (sha256:ae6674dccc9688e3f58e209e685049ebc64678d0c6f0a9b46247691096d968c1)
#[derive(Clone, Default)]
pub struct SvgGradientStop {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: SvgColor,
    pub offset: f64,
}
impl PartialEq for SvgGradientStop {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:95 (sha256:fdb5355e4bd7216d5d179b17f3df9f39ab75c6d964133af3b2a33f4cc31bc347)
#[derive(Clone, Default)]
pub struct SvgImportContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub css_rules: Vec<SvgCssRule>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub elements_by_id: Vec<(String, XmlElement)>,
    pub gradients_by_id: Vec<(String, SvgGradient)>,
    pub object_bounding_boxes: Vec<(Node2D, Rectangle)>,
    pub options: Option<SvgDocumentImportOptions>,
    pub parent_by_element: Vec<(XmlElement, Option<XmlElement>)>,
    pub registry: SvgRegistry,
    pub reported_unsupported_elements: Vec<XmlElement>,
    pub resolved_definition_styles: Vec<(XmlElement, SvgStyle)>,
    pub resolving_clip_uses: Vec<String>,
    pub resolving_clips: Vec<String>,
    pub resolving_gradients: Vec<String>,
    pub resolving_uses: Vec<String>,
}
impl PartialEq for SvgImportContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:112 (sha256:2ff91bf128ff1a549012e55b778b53ccfa01f2e675e5c05a5e6bea56f712882f)
#[derive(Clone, Default)]
pub struct SvgRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip_handlers: Vec<SvgClipHandlerEntry>,
    pub element_handlers: Vec<SvgElementHandlerEntry>,
}
impl PartialEq for SvgRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SvgRegistry.ts:117 (sha256:e1e9699c37f50e623b06d669a58b34fd44ea14e7a0eb22fe6d8c5537e244a8a9)
#[derive(Clone, Default)]
pub struct SvgStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip_rule: PathWinding,
    pub color: String,
    pub display: String,
    pub fill: String,
    pub fill_opacity: f64,
    pub fill_rule: PathWinding,
    pub filter: String,
    pub font_family: String,
    pub font_size: f64,
    pub font_style: String,
    pub font_weight: String,
    pub opacity: f64,
    pub stroke: String,
    pub stroke_dasharray: String,
    pub stroke_dashoffset: f64,
    pub stroke_linecap: String,
    pub stroke_linejoin: String,
    pub stroke_miterlimit: f64,
    pub stroke_opacity: f64,
    pub stroke_width: f64,
    pub text_anchor: String,
    pub visibility: String,
}
impl PartialEq for SvgStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
