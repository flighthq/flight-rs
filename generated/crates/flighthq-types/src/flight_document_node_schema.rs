// @generated from upstream/packages/types/src/FlightDocumentNodeSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FlightDocumentFieldSchema, FlightDocumentFields, Kind, NodeAny};

// Source: upstream/packages/types/src/FlightDocumentNodeSchema.ts:8 (sha256:01db29125a0cc0fbea2232220866a3fcd91dc78ff974b97bfa97d0e6f571c70d)
pub type FlightDocumentResourceLookup = Vec<(String, crate::FlightValue)>;

// Source: upstream/packages/types/src/FlightDocumentNodeSchema.ts:10 (sha256:00c362c2d55f90cba9659691714eb5549cc6c71bd5e327bd7e785656a17d7c3f)
pub type FlightDocumentNodeFactory = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(FlightDocumentFields, FlightDocumentResourceLookup) -> Option<NodeAny>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/FlightDocumentNodeSchema.ts:15 (sha256:a4d92e1690cc72d3656e97b937003ab722d1dc6bf09a722c888c008a7c8faa98)
pub type FlightDocumentNodeFieldWriter = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(FlightDocumentFields, NodeAny, FlightDocumentResourceLookup) -> bool
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/FlightDocumentNodeSchema.ts:23 (sha256:b165a5225d9e207761ab5712a44bc950debbc82c9e5c29cbabc000f33023a080)
#[derive(Clone)]
pub struct FlightDocumentNodeSchema {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_node: FlightDocumentNodeFactory,
    pub fields: Vec<FlightDocumentFieldSchema>,
    pub kind: Kind,
    pub write_node_fields: FlightDocumentNodeFieldWriter,
}
impl PartialEq for FlightDocumentNodeSchema {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
