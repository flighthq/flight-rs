// @generated from upstream/packages/scene3d-gl/src/glMeshFragmentTail.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/scene3d-gl/src/glMeshFragmentTail.ts:21 (sha256:2abda4bdede24cdf25e8dfdf6d5744995a919c6866ee73b2e07b23c08ad5e4f4)
pub const GL_MESH_FRAGMENT_TAIL_UNIFORMS: &'static str =
    "uniform float u_objectAlpha;\nuniform float u_alphaIsCoverage;";

// Source: upstream/packages/scene3d-gl/src/glMeshFragmentTail.ts:24 (sha256:220c7a91da9c8b750fbd5c94defb42ca5adaade848fa1181fd8611081441a183)
pub const GL_MESH_FRAGMENT_TAIL: &'static str = "  fragColor.a = mix(1.0, fragColor.a, u_alphaIsCoverage) * u_objectAlpha;\n  fragColor.rgb *= fragColor.a;";
