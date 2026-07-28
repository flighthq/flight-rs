// @generated from upstream/packages/entity/src/entity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/entity/src/entity.ts:4 (sha256:e8e8ec56ac0d693d53dc93ebf38f390809e722b38285fd85aff1e9c1d5c98d77)
#[derive(Clone)]
struct CreateEntityRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateEntityRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_entity<Type: Clone>(mut obj: Option<Type>) -> Type {
    if (obj).is_none() {
        obj = Some(panic!(
            "cannot construct an untyped Type without a native value"
        ));
    }
    let mut entity = (obj).clone().unwrap();
    ();
    return entity;
}
