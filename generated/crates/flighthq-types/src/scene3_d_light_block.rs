// @generated from upstream/packages/types/src/Scene3DLightBlock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:16 (sha256:4f02cd2e116d99ce5b2af2c64e24ae32f9498383db2c912a82071baa98a33344)
#[derive(Clone, Default)]
pub struct Scene3DLightBlock {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ambient_count: f64,
    pub data: Vec<f32>,
    pub directional_count: f64,
    pub hemisphere_count: f64,
    pub point_count: f64,
    pub spot_count: f64,
    pub version: f64,
}
impl PartialEq for Scene3DLightBlock {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:32 (sha256:1329bc3822561fa5a7881b8cc5bb49e1b8d65e7184080f1d25d0002bec76349b)
pub const MAX_FORWARD_LIGHTS: f64 = 4.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:39 (sha256:51b776b059233292603d7e02f6594c1f35d3b240ee68b1524a63775393eaf6f3)
pub const SCENE_LIGHT_DIRECTIONAL_DIRECTION_OFFSET: f64 = 0.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:40 (sha256:d087de760baf851b0b3eb88d03fb0687fb49d42ec4f732bbdd4eb0af9595d028)
pub const SCENE_LIGHT_DIRECTIONAL_RADIANCE_OFFSET: f64 = 4.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:41 (sha256:cdbf5747cfb3440bc6c300600b41fe369eee70834937516f831636acb7b87a89)
pub const SCENE_LIGHT_AMBIENT_RADIANCE_OFFSET: f64 = 8.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:42 (sha256:ed08d335bc51a88b5034875c844b59962d373a3334e166d1cbd0dafc664ae907)
pub const SCENE_LIGHT_HEAD_FLOATS: f64 = 12.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:46 (sha256:93f10edc5a7d4f023dc6c477557758f1706d7ac56ac7521a13734126e3f7cd75)
pub const SCENE_LIGHT_POINT_OFFSET: f64 = 12.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:47 (sha256:35c3fd1a0ce0b48ded0d5c4a843882b89b7005fac4665e04709dffa144bcefe8)
pub const SCENE_LIGHT_POINT_STRIDE: f64 = 8.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:51 (sha256:d68f3d5714b04bd718497a2faa3dcdbc2c74b7177ca92dc4fac5d7a7e872e734)
pub const SCENE_LIGHT_SPOT_OFFSET: f64 = 44.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:52 (sha256:b448c007c525aee750fad837154424e181073b9aea34251202da177be6aff9d5)
pub const SCENE_LIGHT_SPOT_STRIDE: f64 = 16.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:56 (sha256:8a31b77ace3b6f3c0835c87350c85ad7c2b2cb2b560fab15efd6df8b2472139c)
pub const SCENE_LIGHT_HEMISPHERE_OFFSET: f64 = 108.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:57 (sha256:b25c93c0c203a96c218b8caee0c92e0643d8f950f1e766ae0973752053f1f1f1)
pub const SCENE_LIGHT_HEMISPHERE_STRIDE: f64 = 12.0_f64;

// Source: upstream/packages/types/src/Scene3DLightBlock.ts:60 (sha256:dd588163efc7a9f68396db76287e8def927abd40b5854d3313195f0de425e35a)
pub const SCENE_LIGHT_BLOCK_FLOATS: f64 = 156.0_f64;
