// @generated from upstream/packages/types/src/GlCompressedTextureSupport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlCompressedTextureSupport.ts:12 (sha256:30c0a57e67d5b5bc1d6eed78fef6f716c2a5411ea717c77fb8eb0e232653ccbc)
#[derive(Clone)]
pub struct GlCompressedTextureSupport {
    pub astc: bool,
    pub bptc: bool,
    pub etc: bool,
    pub pvrtc: bool,
    pub rgtc: bool,
    pub s3tc: bool,
}
