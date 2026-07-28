// @generated from upstream/packages/haptics/src/haptics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    HapticImpactStyle, HapticNotificationType, HapticsBackend, HapticsCapabilities,
};

// Source: upstream/packages/haptics/src/haptics.ts:4 (sha256:84152e23e5b954723bbcb60f44671d2507fdd1a16d2f91095fcfb6d47ef7c4c1)
pub fn cancel_device_vibration() -> bool {
    return {
        let __flight_callback = (get_haptics_backend().cancel).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:11 (sha256:7adac207276236de617bbfbcad1fa2cd3c3ec5921f6d7aeb253bed36a4384b31)
pub fn create_web_haptics_backend() -> HapticsBackend {
    return HapticsBackend {
        __flight_identity: std::sync::Arc::new(()),
        cancel: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> bool {
            return web_vibrate(&(crate::FlightUnion2::<f64, Vec<f64>>::A(0.0_f64)));
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        capabilities: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut out: HapticsCapabilities| -> HapticsCapabilities {
                let supported = ("undefined" != "undefined") && ("function" == "function");
                out.amplitude_control = false;
                out.custom_events = false;
                out.intensity = false;
                out.patterns = supported;
                out.supported = supported;
                return out;
            },
        )
            as Box<dyn FnMut(HapticsCapabilities) -> HapticsCapabilities + Send + 'static>)),
        impact: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |style: HapticImpactStyle, intensity: Option<f64>| -> bool {
                let base = if (style == "heavy") || (style == "rigid") {
                    30.0_f64
                } else {
                    if (style == "medium") {
                        20.0_f64
                    } else {
                        if (style == "soft") {
                            25.0_f64
                        } else {
                            10.0_f64
                        }
                    }
                };
                let ms = if (intensity).is_some() {
                    (base * (0.0_f64).max((1.0_f64).min(*(intensity.as_ref().unwrap())))).round()
                } else {
                    base
                };
                return web_vibrate(&(crate::FlightUnion2::<f64, Vec<f64>>::A(ms)));
            },
        )
            as Box<dyn FnMut(HapticImpactStyle, Option<f64>) -> bool + Send + 'static>)),
        is_supported: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> bool {
            return ("undefined" != "undefined") && ("function" == "function");
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        notification: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |type_: HapticNotificationType| -> bool {
                let pattern = if (type_ == "error") {
                    vec![20.0_f64, 60.0_f64, 20.0_f64]
                } else {
                    if (type_ == "warning") {
                        vec![20.0_f64, 60.0_f64, 20.0_f64, 60.0_f64]
                    } else {
                        vec![15.0_f64, 50.0_f64, 15.0_f64]
                    }
                };
                return web_vibrate(&(crate::FlightUnion2::<f64, Vec<f64>>::B((pattern).clone())));
            },
        )
            as Box<dyn FnMut(HapticNotificationType) -> bool + Send + 'static>)),
        prepare: Some(std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>,
        ))),
        selection: std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || -> bool {
            return web_vibrate(&(crate::FlightUnion2::<f64, Vec<f64>>::A(5.0_f64)));
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        vibrate: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |duration_ms: f64| -> bool {
                return web_vibrate(&(crate::FlightUnion2::<f64, Vec<f64>>::A(duration_ms)));
            },
        )
            as Box<dyn FnMut(f64) -> bool + Send + 'static>)),
        vibrate_pattern: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |pattern: Vec<f64>| -> bool {
                if ((pattern.len() as f64) == 0.0_f64) {
                    return false;
                }
                return web_vibrate(&(crate::FlightUnion2::<f64, Vec<f64>>::B((pattern).clone())));
            },
        )
            as Box<dyn FnMut(Vec<f64>) -> bool + Send + 'static>)),
        vibrate_waveform: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |timings: Vec<f64>, _amplitudes: Vec<f64>, repeat: Option<f64>| -> bool {
                if ((timings.len() as f64) == 0.0_f64) {
                    return false;
                }
                {
                    repeat;
                    ()
                };
                return web_vibrate(&(crate::FlightUnion2::<f64, Vec<f64>>::B((timings).clone())));
            },
        )
            as Box<dyn FnMut(Vec<f64>, Vec<f64>, Option<f64>) -> bool + Send + 'static>))),
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:60 (sha256:b31bc520204eb4935c051b97650d7b4931c9b54ce529c047eaf7248235599aa0)
pub fn get_haptics_backend() -> HapticsBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_haptics_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/haptics/src/haptics.ts:66 (sha256:7a1382de8efb35fdedeb9ddb3e3b3d89f503fd19a24e25f7e234d828f31590e3)
pub fn get_haptics_capabilities(out: &HapticsCapabilities) -> HapticsCapabilities {
    return {
        let __flight_callback = (get_haptics_backend().capabilities).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:71 (sha256:eac1e1afd7a1b095493bd83a89ba06898f533e064a0970f75d25f8796f3e4931)
pub fn is_haptics_supported() -> bool {
    return {
        let __flight_callback = (get_haptics_backend().is_supported).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:77 (sha256:8bb03dd0cfb79650e3b1c60e44f9501c08df44b0682b0dd70e1a9cb6e67789b7)
pub fn prepare_haptics() -> () {
    {
        let __flight_callback = (get_haptics_backend().prepare).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:82 (sha256:5b029e6ffe5bb885e98ff5d9d9e68040918bed8361f712abf23ba92c226c2669)
pub fn set_haptics_backend(backend: Option<HapticsBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/haptics/src/haptics.ts:88 (sha256:1981b4ac71601a2daaeee7eeb02db8909e9d77cbb11faf0f69efde1ae55ccadd)
pub fn trigger_haptic_impact(style: HapticImpactStyle, intensity: Option<f64>) -> bool {
    return {
        let __flight_callback = (get_haptics_backend().impact).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (style).clone(),
            Some((intensity).unwrap_or(1.0_f64)),
        );
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:96 (sha256:bbee84beb93c89b6d3be02433b6db63c0ea93dc3afc481eaa232d579b61065ec)
pub fn trigger_haptic_notification(type_: HapticNotificationType) -> bool {
    return {
        let __flight_callback = (get_haptics_backend().notification).clone();
        let __flight_result = __flight_callback.lock().unwrap()((type_).clone());
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:101 (sha256:918b3de1a372c69599b61c12bdecc5b2ed8c3c404e9d118036a12e900da93619)
pub fn trigger_haptic_selection() -> bool {
    return {
        let __flight_callback = (get_haptics_backend().selection).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:106 (sha256:29f8b43fdb7119804dbd444671d5c2a9f288f0f5b1ca735e726a2f8b14d28899)
pub fn vibrate_device(duration_ms: f64) -> bool {
    return {
        let __flight_callback = (get_haptics_backend().vibrate).clone();
        let __flight_result = __flight_callback.lock().unwrap()(duration_ms);
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:112 (sha256:78e511d652effb19fd81003753fad3d142dbddb06778e9dfe1fa7d797beb7f87)
pub fn vibrate_device_pattern(pattern: &Vec<f64>) -> bool {
    if ((pattern.len() as f64) == 0.0_f64) {
        return false;
    }
    return {
        let __flight_callback = (get_haptics_backend().vibrate_pattern).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*pattern).clone());
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:122 (sha256:0a606c6c489a16491b1088a62aa076623f29627c7d00f963f2a00fb133c179b9)
pub fn vibrate_device_waveform(
    timings: &Vec<f64>,
    amplitudes: &Vec<f64>,
    repeat: Option<f64>,
) -> bool {
    let repeat = repeat.unwrap_or((-1.0_f64));
    let backend = get_haptics_backend();
    if ((timings.len() as f64) == 0.0_f64) {
        return false;
    }
    if ((backend.vibrate_waveform).clone()).is_some() {
        return {
            let __flight_callback = backend.vibrate_waveform.as_ref().unwrap().clone();
            let __flight_result = __flight_callback.lock().unwrap()(
                (*timings).clone(),
                (*amplitudes).clone(),
                Some(repeat),
            );
            __flight_result
        };
    }
    return {
        let __flight_callback = (backend.vibrate_pattern).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*timings).clone());
        __flight_result
    };
}

// Source: upstream/packages/haptics/src/haptics.ts:135 (sha256:923dca4c11a999716491d937ad32dd8a0a9d85b21abfa7215b77557af7c1965d)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<HapticsBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/haptics/src/haptics.ts:137 (sha256:b58f932e4ebe817afe07e6b6284c3c03487e0d7255d1458f6b81c353a0347c5a)
fn web_vibrate(pattern: &crate::FlightUnion2<f64, Vec<f64>>) -> bool {
    {
        return false;
    }
}
