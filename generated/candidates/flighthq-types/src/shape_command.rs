// @generated from upstream/packages/types/src/ShapeCommand.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ShapeCommand.ts:4 (sha256:4296b59e30cf2f2065cbcc75e866f661ca9c7ed3d4a26124ac823be98385ffc1)
pub type CapsStyle = String;

// Source: upstream/packages/types/src/ShapeCommand.ts:6 (sha256:d5a969dd13970ec044b58790b934cd34d835432b0bff462be5ac496e19717d66)
pub type GradientType = String;

// Source: upstream/packages/types/src/ShapeCommand.ts:8 (sha256:b8c91976c5029faacc310589281c8d002d6fe3693919a47131759fc2f87edcef)
pub type PathWinding = String;

// Source: upstream/packages/types/src/ShapeCommand.ts:10 (sha256:57608fd0d901b735c548bb720897e73cc0267f71a9731b09b3851d71c5a44cb5)
pub type InterpolationMethod = String;

// Source: upstream/packages/types/src/ShapeCommand.ts:12 (sha256:0bfd1d751e9a9812454feac90111c6c615c086e0a3c63fb87d85bf75ce6c941c)
pub type JointStyle = String;

// Source: upstream/packages/types/src/ShapeCommand.ts:14 (sha256:c68453d28d184240b3c58d97977b9d6b52cdcd29bd80fc8b825183447035a22d)
pub type LineScaleMode = String;

// Source: upstream/packages/types/src/ShapeCommand.ts:16 (sha256:023e9c7b0bef77eb8c04171ba53ff3c5be45cef5f7de11f1405be050bc35699a)
pub type SpreadMethod = String;

// Source: upstream/packages/types/src/ShapeCommand.ts:19 (sha256:22fb7d1d443aab0e1b9a6696c74b267d14950c785bf7e614378a029c5dac6373)
#[derive(Clone)]
pub struct ShapeCommandRegistry {
    pub begin_bitmap_fill: Vec<crate::OpaqueHostValue>,
    pub begin_fill: Vec<f64>,
    pub begin_gradient_fill: Vec<crate::OpaqueHostValue>,
    pub cubic_curve_to: Vec<f64>,
    pub curve_to: Vec<f64>,
    pub draw_circle: Vec<f64>,
    pub draw_ellipse: Vec<f64>,
    pub draw_path: Vec<crate::OpaqueHostValue>,
    pub draw_rectangle: Vec<f64>,
    pub draw_round_rectangle: Vec<f64>,
    pub end_fill: Vec<crate::OpaqueHostValue>,
    pub line_bitmap_style: Vec<crate::OpaqueHostValue>,
    pub line_gradient_style: Vec<crate::OpaqueHostValue>,
    pub line_style: Vec<crate::OpaqueHostValue>,
    pub line_to: Vec<f64>,
    pub move_to: Vec<f64>,
}

// Source: upstream/packages/types/src/ShapeCommand.ts:79 (sha256:2c8f717c5aa0e004115771bf15c185753aea1973a179c06b8564a915549b1fd8)
pub type ShapeCommandKey = ShapeCommandRegistry;

// Source: upstream/packages/types/src/ShapeCommand.ts:88 (sha256:6493cf9e05ea30a04a520b32614a162041871a3e716e0bb68b7b187daf3124a7)
pub type ShapeCommandToken = Option<crate::OpaqueHostValue>;

// Source: upstream/packages/types/src/ShapeCommand.ts:91 (sha256:6d82672b7e69c1f5c4413f383f6b58e5333c126d0d13257d981a3ed6a5f6c8cc)
pub type ShapeCommandHitTest =
    std::sync::Arc<dyn Fn(f64, f64, Vec<ShapeCommandToken>, f64) -> bool + Send + Sync + 'static>;

// Source: upstream/packages/types/src/ShapeCommand.ts:94 (sha256:7bebf4b9846d9fc89094e47d6447a8acae36543375609cd967c4129012452dbe)
pub type ShapeHitTestCommand = crate::OpaqueHostValue;
