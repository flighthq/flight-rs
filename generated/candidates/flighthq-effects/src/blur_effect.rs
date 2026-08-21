// @generated from upstream/packages/effects/src/blurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_gaussian_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{BlurEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord2484562537 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
}
impl PartialEq for FlightOmitRecord2484562537 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/blurEffect.ts:8 (sha256:c201dc944b5cc997ed7759922a210418e3312ca5fa5dc98527912545d52f144e)
#[derive(Clone, Default)]
struct CreateBlurEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBlurEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_blur_effect(options: Option<FlightOmitRecord2484562537>) -> BlurEffect {
    let options = options.unwrap_or(FlightOmitRecord2484562537 {
        __flight_identity: std::sync::Arc::new(()),
        blur_x: None,
        blur_y: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        BlurEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BlurEffect".to_owned(),
            blur_x: __flight_spread_1.blur_x,
            blur_y: __flight_spread_1.blur_y,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/blurEffect.ts:12 (sha256:52002f3974ed6d4621c069d3f89b228a66053fe95bcdd80e843ea6bff4f6a3da)
pub fn get_blur_effect_padding(effect: &BlurEffect) -> RenderEffectPadding {
    return get_gaussian_render_effect_padding(
        (effect.blur_x).clone().unwrap_or(4.0_f64),
        (effect.blur_y).clone().unwrap_or(4.0_f64),
    );
}

// Source: upstream/packages/effects/src/blurEffect.ts:16 (sha256:512b7ab945b89b20abea58fc37a91c23babab7880fad581a27d352bfa309b75a)
pub fn register_blur_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "BlurEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_blur_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/blurEffect.ts:20 (sha256:3b2bf6bdbde328ddff0de85ff6dd8e1d0d0b464b9f091f0a27ac1d130fcaf875)
fn resolve_blur_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_blur_effect_padding(&{
        let __flight_source = &((*effect).clone());
        BlurEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            blur_x: __flight_source.blur_x,
            blur_y: __flight_source.blur_y,
            ..Default::default()
        }
    });
}
