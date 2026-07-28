// @generated from upstream/packages/effects/src/ssrEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SsrEffect;

// Source: upstream/packages/effects/src/ssrEffect.ts:3 (sha256:30717c7fd0026c3b42818f92458ad6af6bfa29454faf42e1fced442ccd6580f6)
#[derive(Clone)]
struct CreateSsrEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSsrEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_ssr_effect(options: Option<SsrEffect>) -> SsrEffect {
    let options = options.unwrap_or(SsrEffect {
        __flight_identity: std::sync::Arc::new(()),
        max_distance: None,
        resolution: None,
        steps: None,
    });
    return SsrEffect {
        kind: "SsrEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
