// @generated from upstream/packages/types/src/NonEntityCreateResult.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/NonEntityCreateResult.ts:15 (sha256:6107ce7f1ad714e05b8f28dc13553c5465939926e6474937318c529909d49710)
pub struct NonEntityCreateResult<Type, Kind>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (Type, Kind)>,
);
impl<Type, Kind> Clone for NonEntityCreateResult<Type, Kind> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}
