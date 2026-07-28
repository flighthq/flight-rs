// @generated from upstream/packages/types/src/SceneResourceRef.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ResourceResolutionState;

// Source: upstream/packages/types/src/SceneResourceRef.ts:19 (sha256:6d9879b086fd261aa3e612640eba3775cf3616966fd7549f026399e7a9f6a6ab)
// TypeScript value namespace SceneResourceRefKind is represented by its generated Rust type.

// Source: upstream/packages/types/src/SceneResourceRef.ts:24 (sha256:3cce1b6a163bfbc391e5e8fe64c21753dee06b2f1644a5fe0e5ca17c1a21dbfc)
pub type SceneResourceRefKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/SceneResourceRef.ts:26 (sha256:4ee0d3100445a232f371383b0c8814a4733c92a5b104b7161b3cbbbb1f3f36a7)
#[derive(Clone)]
struct SceneResourceRefBase {
    pub mime_type: Option<String>,
    pub state: ResourceResolutionState,
}

// Source: upstream/packages/types/src/SceneResourceRef.ts:37 (sha256:7146c1414efc8e29e20296d4a6a42c86d0166ae7fb08651b22f41988985d4dad)
#[derive(Clone)]
pub struct EmbeddedSceneResourceRef {
    pub mime_type: Option<String>,
    pub state: ResourceResolutionState,
    pub kind: String,
    pub bytes: Vec<u8>,
}

// Source: upstream/packages/types/src/SceneResourceRef.ts:45 (sha256:140b7e85692e88f5951041afa2025e9a7949ffd119cba949a3ad0d634095ac6b)
#[derive(Clone)]
pub struct ExternalSceneResourceRef {
    pub mime_type: Option<String>,
    pub state: ResourceResolutionState,
    pub kind: String,
    pub uri: String,
    pub base_path: Option<String>,
}

// Source: upstream/packages/types/src/SceneResourceRef.ts:51 (sha256:dedd1225e56402a32de0c081a99f78c7719d2d4a50a106e7006fd74affd289e6)
pub type SceneResourceRef = crate::OpaqueHostValue;
