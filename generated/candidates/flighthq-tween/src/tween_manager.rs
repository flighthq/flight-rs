// @generated from upstream/packages/tween/src/tweenManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_easing::ease_out_exponential;
use flighthq_types::TweenManager;
pub use flighthq_types::TweenManagerOptions;

// Source: upstream/packages/tween/src/tweenManager.ts:6 (sha256:f283117ad4266ac2cd40a1bc1f2fdba1bf6cb513f0a9981569ba72fa05d34711)
pub fn create_tween_manager(options: Option<TweenManagerOptions>) -> TweenManager {
    return TweenManager {
        __flight_identity: std::sync::Arc::new(()),
        __brand: "TweenManager".to_owned(),
        default_ease: (options
            .as_ref()
            .and_then(|value| (value.default_ease).clone()))
        .unwrap_or(ease_out_exponential),
        tweens: Vec::new(),
    };
}

// Source: upstream/packages/tween/src/tweenManager.ts:14 (sha256:01ed2c4077a68d9d42692046e2f78ee057dda2bde2cf56f5db7a73f06799ffdb)
pub static DEFAULT_MANAGER: std::sync::LazyLock<TweenManager> =
    std::sync::LazyLock::new(|| create_tween_manager(None));
