// @generated from upstream/packages/haptics/src/haptics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    HapticImpactStyle, HapticNotificationType, HapticsCapabilities, HostHapticsCapability,
};

// Source: upstream/packages/haptics/src/haptics.ts:8 (sha256:60b87250e563e9c2f88175dc02fa079ee89c28a4341e0fa8f1068b4bf7f8d1d7)
pub fn cancel_device_vibration(host_haptics: &HostHapticsCapability) -> bool {
    return {
        let __flight_callback = (host_haptics.cancel).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:12 (sha256:9f84198df525f8435b79126cf603bfac890166688068b7f207a7980e13cbf99b)
pub fn get_haptics_capabilities(
    host_haptics: &HostHapticsCapability,
    out: &HapticsCapabilities,
) -> HapticsCapabilities {
    return {
        let __flight_callback = (host_haptics.capabilities).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:19 (sha256:ea59f44718e77b1a02b1fdec4854d3664010af9db49f8c1d5cb6ec9ac804feac)
pub fn is_haptics_supported(host_haptics: &HostHapticsCapability) -> bool {
    return {
        let __flight_callback = (host_haptics.is_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:25 (sha256:52777bc01a6c4cc375203f2886e97d83577f1502d7ffd5066214e09ff9524aa7)
pub fn prepare_haptics(host_haptics: &HostHapticsCapability) -> () {
    {
        let __flight_callback = (host_haptics.prepare).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:29 (sha256:e0c4e06ff049245aeb2ac63a2e33bbc1e0b07debadc3544dd82b8d09d833e335)
pub fn trigger_haptic_impact(
    host_haptics: &HostHapticsCapability,
    style: HapticImpactStyle,
    intensity: Option<f64>,
) -> bool {
    return {
        let __flight_callback = (host_haptics.impact).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (style).clone(),
            Some((intensity).unwrap_or(1.0_f64)),
        );
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:37 (sha256:649407b1b430c32f5171ff01226915915088c95e671329e2e8f27d492a75fa06)
pub fn trigger_haptic_notification(
    host_haptics: &HostHapticsCapability,
    type_: HapticNotificationType,
) -> bool {
    return {
        let __flight_callback = (host_haptics.notification).clone();
        let __flight_result = __flight_callback.lock().unwrap()((type_).clone());
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:44 (sha256:46f8a1ca1822cf5801ed772a3092fee42400f25d4e53e0be693637c59f59bd2f)
pub fn trigger_haptic_selection(host_haptics: &HostHapticsCapability) -> bool {
    return {
        let __flight_callback = (host_haptics.selection).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:48 (sha256:7a18183ca86b93438a0eac60238461d6e57c088ba618938eb01c180d18917aa8)
pub fn vibrate_device(host_haptics: &HostHapticsCapability, duration_ms: f64) -> bool {
    return {
        let __flight_callback = (host_haptics.vibrate).clone();
        let __flight_result = __flight_callback.lock().unwrap()(duration_ms);
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:52 (sha256:78bb9f0ed82af8e7ede65984c1991606fe94e7903c1f7ff0509e43d4f1d528f0)
pub fn vibrate_device_pattern(host_haptics: &HostHapticsCapability, pattern: &Vec<f64>) -> bool {
    if ((pattern.len() as f64) == 0.0_f64) {
        return false;
    }
    return {
        let __flight_callback = (host_haptics.vibrate_pattern).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*pattern).clone());
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:62 (sha256:b173256bdafdca9f265bcda4544a90d5541707687ef211116a851ec766829cd6)
pub fn vibrate_device_waveform(
    host_haptics: &HostHapticsCapability,
    timings: &Vec<f64>,
    amplitudes: &Vec<f64>,
    repeat: Option<f64>,
) -> bool {
    let repeat = repeat.unwrap_or((-1.0_f64));
    if ((timings.len() as f64) == 0.0_f64) {
        return false;
    }
    if ((host_haptics.vibrate_waveform).clone()).is_some() {
        return {
            let __flight_callback = (host_haptics.vibrate_waveform)
                .clone()
                .as_ref()
                .unwrap()
                .clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                (*timings).clone(),
                (*amplitudes).clone(),
                Some(repeat),
            );
            __flight_result
        };
    }
    return {
        let __flight_callback = (host_haptics.vibrate_pattern).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*timings).clone());
        __flight_result
    };
}
