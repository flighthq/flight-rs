// @generated from upstream/packages/materials/src/pbrExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/materials/src/pbrExtension.ts:4 (sha256:10df7bd16ff52f13b594c62952f0cfff8991c341432ee7fa113a232500e0063a)
pub fn is_valid_pbr_uv_set(value: f64) -> bool {
    return (value == 0.0_f64) || (value == 1.0_f64);
}
