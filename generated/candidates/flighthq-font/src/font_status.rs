// @generated from upstream/packages/font/src/fontStatus.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/font/src/fontStatus.ts:3 (sha256:29a12b8ef2d69b9488ac7691478a95a9b958ff01a07082fa7bc38e20b03026cd)
pub fn is_font_loaded(family: String, style: Option<String>) -> bool {
    return crate::host_value::<bool>("host.check");
}

// Source: upstream/packages/font/src/fontStatus.ts:7 (sha256:a9a632e81b2b401d128497ca7e36252f671e86a84e7380d7ffa2915b10717660)
pub fn when_fonts_ready() -> crate::Promise<crate::OpaqueHostValue> {
    Default::default()
}
