// @generated from upstream/packages/types/src/TextFeature.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextFeature.ts:1 (sha256:60965055322e7fae474ca845e825ad8865f8e3e44218ee33cf42176b2c64bb40)
#[derive(Clone)]
pub struct TextFeature {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub end: Option<f64>,
    pub start: Option<f64>,
    pub tag: String,
    pub value: f64,
}
impl PartialEq for TextFeature {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextFeature.ts:7 (sha256:c47c006dfb45fa68c500211eeea84a7928fa4b86202766f5b9abb9e18f7dcfb4)
pub const TEXT_FEATURE_CAPITALS: &'static str = "c2sc";

// Source: upstream/packages/types/src/TextFeature.ts:8 (sha256:fff0f91aba819f198e5e41d58d1a5f9dd4b08d2f73eabff74e6bc19f09ebeac6)
pub const TEXT_FEATURE_CONTEXTUAL_ALTERNATES: &'static str = "calt";

// Source: upstream/packages/types/src/TextFeature.ts:9 (sha256:3afa213829570801584b30528c45f77cad7104bd5efea7ef5575b17b4504e1d0)
pub const TEXT_FEATURE_DISCRETIONARY_LIGATURES: &'static str = "dlig";

// Source: upstream/packages/types/src/TextFeature.ts:10 (sha256:e2e97f308c98c3fa1d282cd4ea9bd0afdc6651b440da79b189031cb68220054a)
pub const TEXT_FEATURE_FRACTIONS: &'static str = "frac";

// Source: upstream/packages/types/src/TextFeature.ts:11 (sha256:c362f3fc6875496218481472a23ca8a737792c0798ff20b2c0c2722f63a04fea)
pub const TEXT_FEATURE_KERNING: &'static str = "kern";

// Source: upstream/packages/types/src/TextFeature.ts:12 (sha256:b3bc1d2ba54b3b307e0c865507467d5ebbab5e462c164e720088b83152558884)
pub const TEXT_FEATURE_LIGATURES: &'static str = "liga";

// Source: upstream/packages/types/src/TextFeature.ts:13 (sha256:16b10b92ffa62d329067757522d3a9bc2330cb09476e611c1fd4c2aa6ef291e9)
pub const TEXT_FEATURE_OLD_STYLE_FIGURES: &'static str = "onum";

// Source: upstream/packages/types/src/TextFeature.ts:14 (sha256:2ce16c9b3e5f520a019d9647af0e1f8bb97bc19c6ecfe94a8b882e16f342594a)
pub const TEXT_FEATURE_SMALL_CAPS: &'static str = "smcp";

// Source: upstream/packages/types/src/TextFeature.ts:15 (sha256:64a76657026f8f33b38a0d6c69bccd5184e70f8edd500708d2514a5ec98a3d03)
pub const TEXT_FEATURE_STYLISTIC_ALTERNATES: &'static str = "salt";

// Source: upstream/packages/types/src/TextFeature.ts:16 (sha256:87a8687e09cde9528e58d512da301575698b1adea05f21d39affa82a6e1d034a)
pub const TEXT_FEATURE_SUBSCRIPT: &'static str = "subs";

// Source: upstream/packages/types/src/TextFeature.ts:17 (sha256:95cc4755dc5f78603f74fdc68a280f57ee86120b6fe867cc1d7932f50713db53)
pub const TEXT_FEATURE_SUPERSCRIPT: &'static str = "sups";

// Source: upstream/packages/types/src/TextFeature.ts:18 (sha256:f9f698e7607e4cf74749f0c6c6565725492d6e798f2038523eb1331c069fe949)
pub const TEXT_FEATURE_TABULAR_FIGURES: &'static str = "tnum";
