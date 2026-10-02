// @generated from upstream/packages/types/src/PathSegment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct PathSegmentRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
}
impl PartialEq for PathSegmentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct PathSegmentRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub control_x1: f64,
    pub control_y1: f64,
    pub control_x2: f64,
    pub control_y2: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for PathSegmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct PathSegmentRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub control_x: f64,
    pub control_y: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for PathSegmentRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct PathSegmentRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for PathSegmentRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct PathSegmentRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for PathSegmentRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PathSegment.ts:12 (sha256:526b0a559687296765dc670d71a0b87853bc581e0b17a0c4c1283df9f9c905fa)
pub type PathSegment = crate::FlightUnion2<
    PathSegmentRecord5,
    crate::FlightUnion2<
        PathSegmentRecord4,
        crate::FlightUnion2<
            PathSegmentRecord3,
            crate::FlightUnion2<PathSegmentRecord2, PathSegmentRecord1>,
        >,
    >,
>;
