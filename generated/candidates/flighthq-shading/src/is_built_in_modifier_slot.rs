// @generated from upstream/packages/shading/src/isBuiltInModifierSlot.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/shading/src/isBuiltInModifierSlot.ts:8 (sha256:6759ed9d9b3efa23b6ea932ddc359048da052d06320f5a27e37a3cd31e555902)
pub fn is_built_in_modifier_slot(value: String) -> bool {
    return BUILT_IN_SLOTS.iter().any(|item| item == &(value).clone());
}

// Source: upstream/packages/shading/src/isBuiltInModifierSlot.ts:12 (sha256:ab68d1d80133bec947ed54dd30bfc077d0cb93da9aea9e84f6be9ae2b6eec378)
static BUILT_IN_SLOTS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| Vec::new());
