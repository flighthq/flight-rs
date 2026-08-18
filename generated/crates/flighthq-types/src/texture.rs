// @generated from upstream/packages/types/src/Texture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Sampler, TextureSource, Vector2, VoxelGrid};

// Source: upstream/packages/types/src/Texture.ts:11 (sha256:782486f0609aa06395ceefd5b4b2405fbe316447c2459f7669158fed78003f2a)
pub type TextureColorSpace = String;

// Source: upstream/packages/types/src/Texture.ts:15 (sha256:b491f554b0d4fe8af6364d44a54b73dac1846e114c0993daa22a53b8ba8eed7f)
pub type TextureSourceCubeFaces = Vec<Option<TextureSource>>;

// Source: upstream/packages/types/src/Texture.ts:26 (sha256:2c6fc189c0d2a0889bf687ce4d63c610dabba379e8a0dde28140271920065add)
#[derive(Clone, Default)]
struct TextureCommon {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
    pub color_space: TextureColorSpace,
    pub sampler: Sampler,
    pub version: f64,
}
impl PartialEq for TextureCommon {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TextureCommon {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Texture.ts:34 (sha256:6fc460454838402f2f491be7c207087b5dbd2f51429db1282a2c6ec7e4b78cd3)
#[derive(Clone, Default)]
pub struct Texture2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
    pub color_space: TextureColorSpace,
    pub sampler: Sampler,
    pub version: f64,
    pub dimension: String,
    pub source: Option<TextureSource>,
}
impl PartialEq for Texture2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Texture2D {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Texture.ts:39 (sha256:f5003ea82cb68d7113381de962ee3e9874c5faf487888269dc3dbb2daf823873)
#[derive(Clone, Default)]
pub struct TextureRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
    pub color_space: TextureColorSpace,
    pub sampler: Sampler,
    pub version: f64,
    pub dimension: String,
    pub sources: TextureSourceCubeFaces,
}
impl PartialEq for TextureRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TextureRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
    pub color_space: TextureColorSpace,
    pub sampler: Sampler,
    pub version: f64,
    pub dimension: String,
    pub source: Option<VoxelGrid>,
}
impl PartialEq for TextureRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TextureRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
    pub color_space: TextureColorSpace,
    pub sampler: Sampler,
    pub version: f64,
    pub dimension: String,
    pub sources: Vec<Option<TextureSource>>,
}
impl PartialEq for TextureRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub type Texture = crate::FlightUnion2<
    Texture2D,
    crate::FlightUnion2<TextureRecord3, crate::FlightUnion2<TextureRecord2, TextureRecord1>>,
>;

// Source: upstream/packages/types/src/Texture.ts:54 (sha256:82fef5fba8ef920384fda4dd0c982cf3554b42d79bd7f3e35df4593e165da3fa)
type TextureLikeFrom = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Texture.ts:56 (sha256:4dceca97a01125d9bb5c81dbdfb122e9c39b3dbc1de22dc38bbf23d42e7e00ac)
pub type TextureLike = TextureLikeFrom;
