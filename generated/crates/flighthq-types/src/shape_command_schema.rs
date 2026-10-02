// @generated from upstream/packages/types/src/ShapeCommandSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ShapeCommandKey;

// Source: upstream/packages/types/src/ShapeCommandSchema.ts:7 (sha256:ff6a09362d831161e77adec3e562521fa98dee3fde4371b3356219885993f8f3)
#[derive(Clone)]
pub struct ShapeCommandSchema<K = ShapeCommandKey> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub arguments: Vec<ShapeCommandSchemaArgument>,
    pub key: K,
    pub required_argument_count: f64,
}
impl<K> PartialEq for ShapeCommandSchema<K> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeCommandSchema.ts:13 (sha256:abeb969da37eb7bf991bc3eafe7bcb75d8b99ec2db047d6b366d382f475fb39a)
#[derive(Clone, Default)]
pub struct ShapeCommandSchemaArgument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub type_: ShapeCommandSchemaArgumentType,
}
impl PartialEq for ShapeCommandSchemaArgument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeCommandSchema.ts:18 (sha256:c0b04d1490b882169f6a1100d88144370c28437a8af29bbaa18c227f26a0e2ac)
pub type ShapeCommandSchemaArgumentType = String;
