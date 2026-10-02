// @generated from upstream/packages/types/src/MethodsOf.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MethodsOf.ts:1 (sha256:7e818ae9f3aaddaeaf026579b5665352ecbe165c9c01f7b8ed656ce823d2b819)
pub struct MethodsOf<T>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (T,)>,
);
impl<T> Clone for MethodsOf<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}
