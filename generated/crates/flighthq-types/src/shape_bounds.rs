// @generated from upstream/packages/types/src/ShapeBounds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CapsStyle, JointStyle, Shape, ShapeCommandToken};

// Source: upstream/packages/types/src/ShapeBounds.ts:4 (sha256:f2ca496237f0642dc6330d7b742019a7b59b9a4340ce8883c8b0f1728c884faf)
pub type ShapeBoundsMode = String;

// Source: upstream/packages/types/src/ShapeBounds.ts:8 (sha256:07a3f5441ec4dfabda3bdf9d21434d80c5bfc8030da8727140b1dfd1674347ff)
#[derive(Clone)]
pub struct ShapeCommandArgumentCursor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub length: f64,
    pub get_argument: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> Option<ShapeCommandToken> + Send + 'static>>,
    >,
}
impl PartialEq for ShapeCommandArgumentCursor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeBounds.ts:15 (sha256:816537ac79f4a27deace04ebd04cca7fd4ad415fb36a761577f3f9d5e912e72f)
#[derive(Clone)]
pub struct ShapeBoundsContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub close_path: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub cubic_curve_to: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64, f64, f64, f64, f64) -> () + Send + 'static>>,
    >,
    pub curve_to:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64, f64) -> () + Send + 'static>>>,
    pub draw_circle:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> () + Send + 'static>>>,
    pub draw_ellipse:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64, f64) -> () + Send + 'static>>>,
    pub draw_rectangle:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64, f64, f64) -> () + Send + 'static>>>,
    pub expand_point:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
    pub flush_path: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub line_to: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
    pub move_to: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
    pub set_stroke_style: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, CapsStyle, JointStyle, f64) -> () + Send + 'static>>,
    >,
}
impl PartialEq for ShapeBoundsContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeBounds.ts:36 (sha256:549dc6ce1c4066055a30b73a0370ba6859cd46621726e8721f2bcb4dc83e2d60)
pub type ShapeBoundsCommandHandler = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(ShapeBoundsContext, ShapeCommandArgumentCursor) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/ShapeBounds.ts:43 (sha256:d5779fc44f043aa3a8a3926a093db87cf0d91e7888c2472726a188b256542f99)
#[derive(Clone)]
pub struct ShapeBoundsCommand<K> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: K,
    pub fill_bounds: Option<ShapeBoundsCommandHandler>,
    pub stroke_bounds: Option<ShapeBoundsCommandHandler>,
}
impl<K> PartialEq for ShapeBoundsCommand<K> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeBounds.ts:49 (sha256:88fb377a6d4d253414a4aa9ebd31c3f92d1ec68f8b80dfd0f3e4defe926c90f1)
#[derive(Clone, Default)]
pub struct ShapeBoundsExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub complete: bool,
    pub missing_command_keys: Vec<String>,
    pub mode: ShapeBoundsMode,
}
impl PartialEq for ShapeBoundsExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeBounds.ts:55 (sha256:5177c92ede303074e409131fde86f60c5b629ea79bfb0918795cc096caa9e81f)
pub type ShapeBoundsGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Shape, ShapeBoundsMode, String) -> () + Send + 'static>>,
>;
