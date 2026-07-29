// @generated from upstream/packages/types/src/ShortcutEvent.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ShortcutEvent.ts:3 (sha256:ff34d619436b5e71e7413452a12406856bbada9a9f25733df21ec593616678ba)
#[derive(Clone, Default)]
pub struct ShortcutEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accelerator: String,
}
impl PartialEq for ShortcutEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
