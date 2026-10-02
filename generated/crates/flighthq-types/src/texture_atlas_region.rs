// @generated from upstream/packages/types/src/TextureAtlasRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/TextureAtlasRegion.ts:3 (sha256:509f146a94fe3a22d6a43cfc510a16f1af89313f1ede489e87600a6866776fea)
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextureAtlasRotation(pub u32);

impl TextureAtlasRotation {
    #[allow(non_upper_case_globals)]
    pub const Counterclockwise90: Self = Self(4294967295_u32);

    #[allow(non_upper_case_globals)]
    pub const None: Self = Self(0_u32);

    #[allow(non_upper_case_globals)]
    pub const Clockwise90: Self = Self(1_u32);
}

impl std::ops::BitAnd for TextureAtlasRotation {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for TextureAtlasRotation {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for TextureAtlasRotation {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for TextureAtlasRotation {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl PartialEq<f64> for TextureAtlasRotation {
    fn eq(&self, rhs: &f64) -> bool {
        self.0 as f64 == *rhs
    }
}

// Source: upstream/packages/types/src/TextureAtlasRegion.ts:9 (sha256:06ce530291e77c78576f6fd188b5b54e173666175d6ba61d1b301848cbaf9498)
#[derive(Clone)]
pub struct TextureAtlasRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub height: f64,
    pub id: f64,
    pub name: Option<String>,
    pub original_height: Option<f64>,
    pub original_width: Option<f64>,
    pub page_name: Option<String>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotation: TextureAtlasRotation,
    pub source_x: f64,
    pub source_y: f64,
    pub trimmed: bool,
    pub x: f64,
    pub y: f64,
    pub width: f64,
}
impl PartialEq for TextureAtlasRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TextureAtlasRegion {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/TextureAtlasRegion.ts:35 (sha256:9863842621d051ad75d93d3a933cbd0c0dac801afd48d9f6d7bb6a7094f9e34d)
pub type TextureAtlasRegionLike = TextureAtlasRegion;
