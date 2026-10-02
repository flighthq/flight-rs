// @generated from upstream/packages/materials/src/explainMaterialConversion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{MaterialConversionExplanation, PhongMaterial, SpecularGlossinessPbrMaterial};

// Source: upstream/packages/materials/src/explainMaterialConversion.ts:14 (sha256:57675cedffe0b82df4e3f66b6e9eb77f0f16aa15467d6d0fd3348dcfcaa73d56)
#[derive(Clone, Default)]
struct ExplainPhongConversionRecord1 {
    __flight_identity: std::sync::Arc<()>,
    dropped_maps: Vec<String>,
    reason: String,
}
impl PartialEq for ExplainPhongConversionRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn explain_phong_conversion(source: &PhongMaterial) -> MaterialConversionExplanation {
    if ((source.specular_map).clone()).is_none() {
        return MaterialConversionExplanation {
            __flight_identity: std::sync::Arc::new(()),
            dropped_maps: vec![],
            reason: None,
        };
    }
    return MaterialConversionExplanation {
        __flight_identity: std::sync::Arc::new(()),
        dropped_maps: vec!["specularMap".to_owned()],
        reason: Some("unsupported-by-target-model".to_owned()),
    };
}

// Source: upstream/packages/materials/src/explainMaterialConversion.ts:28 (sha256:88d2749cefd0b7658f6677a1978a0f1971b756569cbe59934c30c768e7aac016)
#[derive(Clone, Default)]
struct ExplainSpecularGlossinessConversionRecord1 {
    __flight_identity: std::sync::Arc<()>,
    dropped_maps: Vec<String>,
    reason: String,
}
impl PartialEq for ExplainSpecularGlossinessConversionRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn explain_specular_glossiness_conversion(
    source: &SpecularGlossinessPbrMaterial,
) -> MaterialConversionExplanation {
    if ((source.specular_glossiness_map).clone()).is_none() {
        return MaterialConversionExplanation {
            __flight_identity: std::sync::Arc::new(()),
            dropped_maps: vec![],
            reason: None,
        };
    }
    return MaterialConversionExplanation {
        __flight_identity: std::sync::Arc::new(()),
        dropped_maps: vec!["specularGlossinessMap".to_owned()],
        reason: Some("incompatible-channel-semantics".to_owned()),
    };
}
