// @generated from upstream/packages/types/src/RenderProxy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ColorTransform, Kind, Material, MaterialData, Renderable, Renderer, RendererData,
};

// Source: upstream/packages/types/src/RenderProxy.ts:9 (sha256:a83fce0a440eb87068d75b87cbb845c0ec6b4938505e891b25a907b5cceb1e82)
#[derive(Clone)]
pub struct RenderProxy {
    pub source: Renderable,
    pub kind: Kind,
    pub next: Option<Box<RenderProxy>>,
    pub alpha: f64,
    pub appearance_frame_id: f64,
    pub blend_mode: Option<BlendMode>,
    pub color_transform: Option<ColorTransform>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
    pub last_appearance_id: f64,
    pub last_local_content_id: f64,
    pub last_local_transform_id: f64,
    pub name: Option<String>,
    pub renderer: Option<Renderer>,
    pub renderer_data: Option<RendererData>,
    pub renderer_data_source: Option<Renderable>,
    pub renderer_map_id: f64,
    pub transform_frame_id: f64,
    pub visible: bool,
}
