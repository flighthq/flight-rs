// @generated from upstream/packages/types/src/FlightDocumentFieldSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FlightDocumentFieldSchema.ts:1 (sha256:8c33bffe01b6dbb7860266d81327dc445838168ad97f9c950d4299cffe543757)
pub type FlightDocumentScalar = Option<crate::FlightUnion2<bool, crate::FlightUnion2<f64, String>>>;

// Source: upstream/packages/types/src/FlightDocumentFieldSchema.ts:3 (sha256:c8fb694b18676f3843444f275940e6c7bd632fbb1de1dc23f99d7f51a6c7ea44)
#[derive(Clone)]
pub struct FlightDocumentValue(
    pub  crate::FlightUnion2<
        FlightDocumentScalar,
        crate::FlightUnion2<Vec<FlightDocumentValue>, FlightDocumentFields>,
    >,
);

// Source: upstream/packages/types/src/FlightDocumentFieldSchema.ts:7 (sha256:7ea9d9912f971e7d967f0446bb0d587d8d06a32bf3618ce38dbea69e202d94da)
#[derive(Clone, Default)]
pub struct FlightDocumentFields {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightDocumentFields {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentFieldSchema.ts:11 (sha256:ef88c562c4fbdc6d8273914cc7458fd4e208cd3ddbe6d7cb431b8c9258b00626)
pub type FlightDocumentFieldValidator =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(FlightDocumentValue) -> bool + Send + 'static>>>;

// Source: upstream/packages/types/src/FlightDocumentFieldSchema.ts:15 (sha256:0df011808bd2e9a98fcad26c027cd3b49b9d718ad42f4bc152718ae5f84f5cf3)
#[derive(Clone)]
pub struct FlightDocumentFieldSchema {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub default_value: Option<FlightDocumentValue>,
    pub name: String,
    pub required: bool,
    pub validate: FlightDocumentFieldValidator,
}
impl PartialEq for FlightDocumentFieldSchema {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
