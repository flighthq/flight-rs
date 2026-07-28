// @generated from upstream/packages/effects/src/convolutionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ConvolutionEffect;

// Source: upstream/packages/effects/src/convolutionEffect.ts:3 (sha256:da09ff4b411a344966534cbd0fa54eea272b20411116ef4f0c27b20f6052566c)
pub fn create_convolution_effect(options: &ConvolutionEffect) -> ConvolutionEffect {
    return ConvolutionEffect {
        kind: "ConvolutionEffect".to_owned(),
        ..((*options).clone()).clone()
    };
}
