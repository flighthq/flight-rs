// @generated from upstream/packages/adjustments/src/hueSaturationAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ColorTransformFunction, HueSaturationAdjustment};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub hue: Option<f64>,
    pub saturation: Option<f64>,
    pub lightness: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/hueSaturationAdjustment.ts:6 (sha256:dcd80a35c51eb9dbe51668399d612c676ac8470c7d2fc9d62f3d682c2d443f13)
#[derive(Clone, Default)]
struct CreateHueSaturationAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateHueSaturationAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_hue_saturation_adjustment(
    options: Option<FlightOmitRecord1>,
) -> HueSaturationAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        hue: None,
        saturation: None,
        lightness: None,
    });
    let hue = ((options.hue).unwrap_or(0.0_f64) / 360.0_f64);
    let saturation = (options.saturation).unwrap_or(1.0_f64);
    let lightness = (options.lightness).unwrap_or(0.0_f64);
    let mut transform: ColorTransformFunction = std::sync::Arc::new(std::sync::Mutex::new(
        Box::new(move |mut out: Vec<f64>, r: f64, g: f64, b: f64| -> () {
            let mx = ((r).max(g)).max(b);
            let mn = ((r).min(g)).min(b);
            let mut h = 0.0_f64;
            let mut s = 0.0_f64;
            let l = ((mx + mn) * 0.5_f64);
            let d = (mx - mn);
            if (d > 0.0001_f64) {
                s = if (l < 0.5_f64) {
                    (d / (mx + mn))
                } else {
                    (d / ((2.0_f64 - mx) - mn))
                };
                if (mx == r) {
                    h = (((g - b) / d) + if (g < b) { 6.0_f64 } else { 0.0_f64 });
                } else {
                    if (mx == g) {
                        h = (((b - r) / d) + 2.0_f64);
                    } else {
                        h = (((r - g) / d) + 4.0_f64);
                    }
                }
                h /= 6.0_f64;
            }
            h = fract((h + hue));
            s = clamp01((s * saturation));
            let ln = clamp01((l + lightness));
            if (s <= 0.0_f64) {
                {
                    let __flight_index = (0.0_f64) as usize;
                    let __flight_value = ln;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (1.0_f64) as usize;
                    let __flight_value = ln;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (2.0_f64) as usize;
                    let __flight_value = ln;
                    if __flight_index == out.len() {
                        out.push(__flight_value);
                    } else {
                        out[__flight_index] = __flight_value;
                    }
                };
                return;
            }
            let q = if (ln < 0.5_f64) {
                (ln * (1.0_f64 + s))
            } else {
                ((ln + s) - (ln * s))
            };
            let p = ((2.0_f64 * ln) - q);
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = hue2rgb(p, q, (h + (1.0_f64 / 3.0_f64)));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = hue2rgb(p, q, h);
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = hue2rgb(p, q, (h - (1.0_f64 / 3.0_f64)));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
        }) as Box<dyn FnMut(Vec<f64>, f64, f64, f64) -> () + Send + 'static>,
    ));
    return {
        let __flight_spread_1 = options;
        HueSaturationAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "HueSaturationAdjustment".to_owned(),
            transform: (transform).clone(),
            hue: __flight_spread_1.hue,
            saturation: __flight_spread_1.saturation,
            lightness: __flight_spread_1.lightness,
        }
    };
}

// Source: upstream/packages/adjustments/src/hueSaturationAdjustment.ts:44 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}

// Source: upstream/packages/adjustments/src/hueSaturationAdjustment.ts:48 (sha256:ed1a4ba70c5e14d50071a86ebdf4bf7e097b170e2b7e279bea8eafe8e3cd4758)
fn fract(v: f64) -> f64 {
    return (v - (v).floor());
}

// Source: upstream/packages/adjustments/src/hueSaturationAdjustment.ts:52 (sha256:968e7a1d17cf2398f4ebb2dce57eb7aa533e0b7f44c3bb1f5b687b0eaf90c0ee)
fn hue2rgb(p: f64, q: f64, t_raw: f64) -> f64 {
    let mut t = t_raw;
    if (t < 0.0_f64) {
        t += 1.0_f64;
    }
    if (t > 1.0_f64) {
        t -= 1.0_f64;
    }
    if (t < (1.0_f64 / 6.0_f64)) {
        return (p + (((q - p) * 6.0_f64) * t));
    }
    if (t < (1.0_f64 / 2.0_f64)) {
        return q;
    }
    if (t < (2.0_f64 / 3.0_f64)) {
        return (p + (((q - p) * ((2.0_f64 / 3.0_f64) - t)) * 6.0_f64));
    }
    return p;
}
