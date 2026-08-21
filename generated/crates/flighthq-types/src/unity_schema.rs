// @generated from upstream/packages/types/src/UnitySchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, ParticleEmitterConfig};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub pixels_per_unit: Option<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:12 (sha256:b05e1dc336a56b7f55f9d7424c99ca7e0dc686d290d84f9317dcda9a6853151b)
pub type UnityParticleShapeType = String;

// Source: upstream/packages/types/src/UnitySchema.ts:22 (sha256:ec7f1527bf28e072fd70a4757732f1532f8e1d3bb575b469eecbfcc8989b4e1c)
#[derive(Clone, Default)]
pub struct UnityColor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
impl PartialEq for UnityColor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:30 (sha256:4e28b4d4ac812a63f93d5681fa39e0774a1efc4599a5047cc32be8f4514a4f85)
#[derive(Clone, Default)]
pub struct UnityMinMaxValue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: String,
    pub constant: Option<f64>,
    pub constant_min: Option<f64>,
    pub constant_max: Option<f64>,
}
impl PartialEq for UnityMinMaxValue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:37 (sha256:a57160b741fb13bb0630d480b9caf4869fdf48e83e2032f29abbd85082db9b96)
#[derive(Clone, Default)]
pub struct UnityBurst {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub count: f64,
    pub cycle_count: f64,
    pub repeat_interval: f64,
}
impl PartialEq for UnityBurst {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:44 (sha256:dff2e75a544468f3d1831a99ec319ff013d0cc770c004c4a80c9cd97dc6158da)
#[derive(Clone, Default)]
pub struct UnityEmission {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rate_over_time: UnityMinMaxValue,
    pub bursts: Vec<UnityBurst>,
}
impl PartialEq for UnityEmission {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:49 (sha256:53eda305c1dcfff684b132bed035cf2b050f7adc2e53a431f8073fb742fdc869)
#[derive(Clone, Default)]
pub struct UnityShapeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq for UnityShapeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct UnityShape {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: bool,
    pub shape_type: UnityParticleShapeType,
    pub radius: f64,
    pub angle: f64,
    pub scale: UnityShapeRecord1,
}
impl PartialEq for UnityShape {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:61 (sha256:026a71f42f86fcf7e898f02093bc6372cdf706a983e2f3f3f43502daec0fc0ef)
#[derive(Clone, Default)]
pub struct UnityGradientColorKeyRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl PartialEq for UnityGradientColorKeyRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct UnityGradientColorKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub color: UnityGradientColorKeyRecord1,
}
impl PartialEq for UnityGradientColorKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:67 (sha256:dfd5b616b7cdd37b1dc49445c2e8eae27c0990ab193f76aad5f6d5953cceb53b)
#[derive(Clone, Default)]
pub struct UnityGradientAlphaKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub alpha: f64,
}
impl PartialEq for UnityGradientAlphaKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:73 (sha256:b065f6adb9e25f59422b8b2ba46fb9255ceb25c9737fff2e8b9e51b5e93ebeca)
#[derive(Clone, Default)]
pub struct UnityGradient {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color_keys: Vec<UnityGradientColorKey>,
    pub alpha_keys: Vec<UnityGradientAlphaKey>,
}
impl PartialEq for UnityGradient {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:79 (sha256:535025fccf50bacf2a480b3ff78c95f23f2cff1642c405154bec9cfaf688ecd8)
#[derive(Clone, Default)]
pub struct UnityCurveKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub time: f64,
    pub value: f64,
}
impl PartialEq for UnityCurveKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:85 (sha256:5f714665c9f460395efed3a29b351907e225c88b0343540a751c2e2961b8d416)
#[derive(Clone, Default)]
pub struct UnityAnimationCurve {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub keys: Vec<UnityCurveKey>,
}
impl PartialEq for UnityAnimationCurve {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:89 (sha256:f2a34d6164f297ac67e2c7a98c0e7141986fa1a5e2dfec6355b31e73bf6e278a)
#[derive(Clone, Default)]
pub struct UnityColorOverLifetime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: bool,
    pub color_start: UnityColor,
    pub color_end: UnityColor,
    pub gradient: Option<UnityGradient>,
}
impl PartialEq for UnityColorOverLifetime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:99 (sha256:b0cb393f6e34d091b201992c4b2f32fb35a6eff1da32bc8d3267488b0407da67)
#[derive(Clone, Default)]
pub struct UnitySizeOverLifetime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: bool,
    pub size_start: f64,
    pub size_end: f64,
    pub curve: Option<UnityAnimationCurve>,
}
impl PartialEq for UnitySizeOverLifetime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:108 (sha256:5b5e082baae32d759dd9e19eadb2ba66c1e5317c650085fc784d88d7ce063e97)
#[derive(Clone, Default)]
pub struct UnityRotationOverLifetime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: bool,
    pub angular_velocity: UnityMinMaxValue,
}
impl PartialEq for UnityRotationOverLifetime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:115 (sha256:ede587b33b81b2fc9afecf42090541b5920f7fe28a614c0cbdb8b11f49a7a2eb)
#[derive(Clone, Default)]
pub struct UnityParticleDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub duration: f64,
    pub looping: bool,
    pub prewarm: bool,
    pub max_particles: f64,
    pub start_lifetime: UnityMinMaxValue,
    pub start_speed: UnityMinMaxValue,
    pub start_size: UnityMinMaxValue,
    pub start_rotation: UnityMinMaxValue,
    pub start_color: UnityColor,
    pub gravity_modifier: f64,
    pub physics_gravity: f64,
    pub emission: UnityEmission,
    pub shape: UnityShape,
    pub color_over_lifetime: UnityColorOverLifetime,
    pub size_over_lifetime: UnitySizeOverLifetime,
    pub rotation_over_lifetime: UnityRotationOverLifetime,
}
impl PartialEq for UnityParticleDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:135 (sha256:133666b113fec0daa93dc082c34735e90bda8eef5a59b746fbc2afd38e135139)
#[derive(Clone, Default)]
pub struct UnityParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pixels_per_unit: Option<f64>,
}
impl PartialEq for UnityParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:141 (sha256:08a8784afc31699d9ffd2082f30c2d51a652119f672b73e608eeada425531f13)
#[derive(Clone, Default)]
pub struct UnityParsed {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub config: ParticleEmitterConfig,
    pub document: UnityParticleDocument,
    pub diagnostics: Vec<ImportDiagnostic>,
}
impl PartialEq for UnityParsed {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/UnitySchema.ts:149 (sha256:20bd3e0ccb60e42d6dfd5380b8444598829ee9fbc9dc2bc870b356c82c0d51db)
#[derive(Clone, Default)]
pub struct UnitySerializeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pixels_per_unit: Option<f64>,
}
impl PartialEq for UnitySerializeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
