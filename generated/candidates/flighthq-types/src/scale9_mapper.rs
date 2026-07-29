// @generated from upstream/packages/types/src/Scale9Mapper.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Scale9Mapper.ts:1 (sha256:5a9581f0af3de876beb9c556f8569ac04582be180012182873a0ff8b6af5a882)
#[derive(Clone)]
pub struct Scale9Mapper {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub map_x: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
    pub map_y: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
}
impl PartialEq for Scale9Mapper {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
