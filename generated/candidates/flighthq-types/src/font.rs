// @generated from upstream/packages/types/src/Font.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Font.ts:3 (sha256:87ada5ff249ddbcff03022e2e64cf96b1cfb4946188ecab2931c2f205b4a7203)
#[derive(Clone)]
pub struct Font {
    pub name: String,
}

// Source: upstream/packages/types/src/Font.ts:7 (sha256:50b0b605c4a59647c487994d45d99039aa098de090df70939b4e58a2635be502)
#[derive(Clone)]
pub struct FontUrl {
    pub format: Option<String>,
    pub url: String,
}
