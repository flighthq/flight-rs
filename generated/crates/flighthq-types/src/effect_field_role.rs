// @generated from upstream/packages/types/src/EffectFieldRole.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/EffectFieldRole.ts:5 (sha256:c5dcad31ce0c5e182402d1514660a345747605a07c3b4838f5c23fbdd3383d13)
pub type EffectFieldRole = String;

// Source: upstream/packages/types/src/EffectFieldRole.ts:9 (sha256:f0823a98911cf009e9422a6cef4bfa6ef1b1c560c889a36a601100493234dab0)
pub type EffectKindFieldRoles = Vec<(String, EffectFieldRole)>;

// Source: upstream/packages/types/src/EffectFieldRole.ts:14 (sha256:33f6218d48c61118a6e100e2d31e3cb0b401c57fa8bf97608c57c8274d65fb8a)
pub type EffectFieldRoles = Vec<(String, EffectKindFieldRoles)>;
