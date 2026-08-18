// @generated from upstream/packages/types/src/Md5Schema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Md5Schema.ts:5 (sha256:0c07e9cd6e4ce2c9608702f2d64c18b1eeb028445df8ef5f432f2a598edb1a9f)
#[derive(Clone, Default)]
pub struct Md5Joint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub orientation_w: f64,
    pub orientation_x: f64,
    pub orientation_y: f64,
    pub orientation_z: f64,
    pub parent_index: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
}
impl PartialEq for Md5Joint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md5Schema.ts:22 (sha256:26b65f447639183fdc42d64d9d83617c2948752116c9e0ebe3c70d5cca79d41a)
#[derive(Clone, Default)]
pub struct Md5Vertex {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub count_weights: f64,
    pub start_weight: f64,
    pub u: f64,
    pub v: f64,
}
impl PartialEq for Md5Vertex {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md5Schema.ts:33 (sha256:984177bd454d61e56e9791a188f472c4289663ebbd6735267ebab967f4d7ad42)
#[derive(Clone, Default)]
pub struct Md5Weight {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bias: f64,
    pub joint_index: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
}
impl PartialEq for Md5Weight {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md5Schema.ts:45 (sha256:5e1681b04c21587cff32a69abb7b92a7fc746269eebd79842ad72a2d0cd440f0)
#[derive(Clone, Default)]
pub struct Md5Mesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub indices: Vec<f64>,
    pub shader: String,
    pub vertices: Vec<Md5Vertex>,
    pub weights: Vec<Md5Weight>,
}
impl PartialEq for Md5Mesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
