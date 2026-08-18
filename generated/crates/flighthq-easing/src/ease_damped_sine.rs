// @generated from upstream/packages/easing/src/easeDampedSine.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::EasingFunction;

// Source: upstream/packages/easing/src/easeDampedSine.ts:7 (sha256:c35daa3a98b38fe3376687c9eecfa3c499a4fe94dae4910a5f393f2f70ea97e4)
pub fn ease_in_damped_sine(amplitude: f64, period: f64) -> EasingFunction {
    let wavelength = to_damped_sine_wavelength(period);
    let overshoot = to_damped_sine_overshoot(amplitude);
    let phase = to_damped_sine_phase(overshoot, wavelength);
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |t: f64| -> f64 {
        if (t == 0.0_f64) || (t == 1.0_f64) {
            return t;
        }
        let time = (t - 1.0_f64);
        return (-((overshoot * (2.0_f64).powf((10.0_f64 * time)))
            * (((time - phase) * TAU) / wavelength).sin()));
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
}

// Source: upstream/packages/easing/src/easeDampedSine.ts:22 (sha256:6186f5d92098738033f1dbdc57fd2045dc08a8ce13a8681e4c6c01d7ae9817d2)
pub fn ease_in_out_damped_sine(amplitude: f64, period: f64) -> EasingFunction {
    let wavelength = to_damped_sine_wavelength(period);
    let overshoot = to_damped_sine_overshoot(amplitude);
    let phase = to_damped_sine_phase(overshoot, wavelength);
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |t: f64| -> f64 {
        if (t == 0.0_f64) || (t == 1.0_f64) {
            return t;
        }
        let time = ((t * 2.0_f64) - 1.0_f64);
        if (time < 0.0_f64) {
            return ((((-0.5_f64) * overshoot) * (2.0_f64).powf((10.0_f64 * time)))
                * (((time - phase) * TAU) / wavelength).sin());
        }
        return ((((0.5_f64 * overshoot) * (2.0_f64).powf(((-10.0_f64) * time)))
            * (((time - phase) * TAU) / wavelength).sin())
            + 1.0_f64);
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
}

// Source: upstream/packages/easing/src/easeDampedSine.ts:57 (sha256:69949b1d435dd6c33ab73db8fb2097666917c621330b196b6c49fc7972fefe29)
pub fn ease_out_damped_sine(amplitude: f64, period: f64) -> EasingFunction {
    let wavelength = to_damped_sine_wavelength(period);
    let overshoot = to_damped_sine_overshoot(amplitude);
    let phase = to_damped_sine_phase(overshoot, wavelength);
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |t: f64| -> f64 {
        if (t == 0.0_f64) || (t == 1.0_f64) {
            return t;
        }
        return (((overshoot * (2.0_f64).powf(((-10.0_f64) * t)))
            * (((t - phase) * TAU) / wavelength).sin())
            + 1.0_f64);
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
}

// Source: upstream/packages/easing/src/easeDampedSine.ts:68 (sha256:230de164f77878a7d7397a99ee30ce8b20c4d591bb3622fc41c51af417be655a)
fn to_damped_sine_phase(overshoot: f64, wavelength: f64) -> f64 {
    return ((wavelength / TAU) * (1.0_f64 / overshoot).asin());
}

// Source: upstream/packages/easing/src/easeDampedSine.ts:74 (sha256:24441e2abe750ff8740cd83bdf7a778e6f7770c9d8f5202ad51e95dff2a5c38b)
fn to_damped_sine_overshoot(amplitude: f64) -> f64 {
    return if (amplitude < 1.0_f64) {
        1.0_f64
    } else {
        amplitude
    };
}

// Source: upstream/packages/easing/src/easeDampedSine.ts:80 (sha256:48a6d966663e593d20532957e844f09f8dad1e41e37794b0bcc66c20c26538ca)
fn to_damped_sine_wavelength(period: f64) -> f64 {
    return if (period > 0.0_f64) {
        period
    } else {
        DEFAULT_DAMPED_SINE_PERIOD
    };
}

// Source: upstream/packages/easing/src/easeDampedSine.ts:84 (sha256:65338d509e734abd927922e89262f56065c4827ad36f380ff4af389c7d25ce60)
const TAU: f64 = 6.283185307179586_f64;

// Source: upstream/packages/easing/src/easeDampedSine.ts:85 (sha256:3d8b0e60d9a897a01e55ae581bcb5b69e83c03967d0918bda63119d397189221)
const DEFAULT_DAMPED_SINE_PERIOD: f64 = 0.4_f64;
