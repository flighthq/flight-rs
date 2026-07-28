// @generated from upstream/packages/signals/src/throttle.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{connect_signal, disconnect_signal};
use flighthq_types::Signal;

// Source: upstream/packages/signals/src/throttle.ts:10 (sha256:9ff1029d5c3091e092ff0d59362f51d66eed27c820470f270b0a384f0b0e01b6)
#[derive(Clone)]
pub struct SignalThrottleOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub leading: Option<bool>,
    pub trailing: Option<bool>,
}
impl PartialEq for SignalThrottleOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/signals/src/throttle.ts:33 (sha256:3c572a4d71215f45d79f857e4fd661d328f47f70a50cbf19f6c5ad140d453e78)
pub fn connect_signal_at_frame_rate(
    mut source: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    >,
    fps: f64,
    slot: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    let period = (1000.0_f64 / fps);
    let elapsed: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let mut handler: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut elapsed = elapsed.clone();
            let slot = slot.clone();
            move |delta: f64| -> () {
                (*elapsed.lock().unwrap()) += delta;
                if ((*elapsed.lock().unwrap()).clone() >= period) {
                    ((slot).clone()).lock().unwrap()((*elapsed.lock().unwrap()).clone());
                    (*elapsed.lock().unwrap()) %= period;
                }
            }
        })
            as Box<dyn FnMut(f64) -> () + Send + 'static>));
    connect_signal(&mut source, (handler).clone(), None);
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let handler = handler.clone();
        let mut source = source.clone();
        move || -> () { disconnect_signal(&mut source, (handler).clone()) }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
}

// Source: upstream/packages/signals/src/throttle.ts:63 (sha256:8db3fa3451b5f328f628a3d5552b18f0402c1c1106209ac5a525be1b0c006231)
pub fn connect_signal_debounced<T: crate::FlightCallback>(
    mut source: Signal<T>,
    delay_ms: f64,
    slot: T,
    options: Option<SignalThrottleOptions>,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    let leading = (options.as_ref().and_then(|value| value.leading)).unwrap_or(false);
    let trailing = (options.as_ref().and_then(|value| value.trailing)).unwrap_or(true);
    let timer: std::sync::Arc<std::sync::Mutex<Option<crate::FlightTimeout>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let last_args: std::sync::Arc<std::sync::Mutex<Option<<T as crate::FlightCallback>::Args>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let leading_fired: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut clear_timer: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut timer = timer.clone();
            move || -> () {
                if ((*timer.lock().unwrap()).clone()).is_some() {
                    if let Some(__flight_timer) = ((*timer.lock().unwrap()).clone()).clone() {
                        crate::clear_timeout(__flight_timer);
                    };
                    (*timer.lock().unwrap()) = None;
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    let handler = T::flight_from_tuple_callback({
        let clear_timer = clear_timer.clone();
        let mut last_args = last_args.clone();
        let mut leading_fired = leading_fired.clone();
        let slot = slot.clone();
        let mut timer = timer.clone();
        move |args: <T as crate::FlightCallback>::Args| -> () {
            (*last_args.lock().unwrap()) = Some((args).clone());
            if ((leading && ((*timer.lock().unwrap()).clone()).is_none())
                && (!(*leading_fired.lock().unwrap()).clone()))
            {
                (*leading_fired.lock().unwrap()) = true;
                crate::FlightCallback::flight_call(&((slot).clone()), ((args).clone()).clone());
            }
            ((clear_timer).clone()).lock().unwrap()();
            (*timer.lock().unwrap()) = Some(crate::set_timeout(
                {
                    let mut last_args = last_args.clone();
                    let mut leading_fired = leading_fired.clone();
                    let slot = slot.clone();
                    let mut timer = timer.clone();
                    move || -> () {
                        (*timer.lock().unwrap()) = None;
                        (*leading_fired.lock().unwrap()) = false;
                        if (trailing && ((*last_args.lock().unwrap()).clone()).is_some()) {
                            crate::FlightCallback::flight_call(
                                &((slot).clone()),
                                (((*last_args.lock().unwrap()).clone()).clone().unwrap()).clone(),
                            );
                            (*last_args.lock().unwrap()) = None;
                        }
                    }
                },
                delay_ms,
            ));
        }
    });
    connect_signal(&mut source, (handler).clone(), None);
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let clear_timer = clear_timer.clone();
        let handler = handler.clone();
        let mut source = source.clone();
        move || -> () {
            disconnect_signal(&mut source, (handler).clone());
            ((clear_timer).clone()).lock().unwrap()();
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
}

// Source: upstream/packages/signals/src/throttle.ts:115 (sha256:ab31c8b9283784b537ad87203af0ffca4815b2e7a959e9a1cda9b0e8dfce80fa)
pub fn connect_signal_throttled<T: crate::FlightCallback>(
    mut source: Signal<T>,
    interval_ms: f64,
    slot: T,
    options: Option<SignalThrottleOptions>,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    let leading = (options.as_ref().and_then(|value| value.leading)).unwrap_or(true);
    let trailing = (options.as_ref().and_then(|value| value.trailing)).unwrap_or(true);
    let last_fired_at: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-f64::INFINITY)));
    let trailing_timer: std::sync::Arc<std::sync::Mutex<Option<crate::FlightTimeout>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let last_args: std::sync::Arc<std::sync::Mutex<Option<<T as crate::FlightCallback>::Args>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut clear_trailing: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut trailing_timer = trailing_timer.clone();
        move || -> () {
            if ((*trailing_timer.lock().unwrap()).clone()).is_some() {
                if let Some(__flight_timer) = ((*trailing_timer.lock().unwrap()).clone()).clone() {
                    crate::clear_timeout(__flight_timer);
                };
                (*trailing_timer.lock().unwrap()) = None;
            }
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    let handler = T::flight_from_tuple_callback({
        let clear_trailing = clear_trailing.clone();
        let mut last_args = last_args.clone();
        let mut last_fired_at = last_fired_at.clone();
        let slot = slot.clone();
        let mut trailing_timer = trailing_timer.clone();
        move |args: <T as crate::FlightCallback>::Args| -> () {
            let now = crate::flight_now_millis();
            let remaining = (interval_ms - (now - (*last_fired_at.lock().unwrap()).clone()));
            if ((remaining <= 0.0_f64) || (remaining > interval_ms)) {
                ((clear_trailing).clone()).lock().unwrap()();
                (*last_fired_at.lock().unwrap()) = now;
                if leading {
                    crate::FlightCallback::flight_call(&((slot).clone()), ((args).clone()).clone());
                } else {
                    (*last_args.lock().unwrap()) = Some((args).clone());
                }
            } else {
                if trailing {
                    ((clear_trailing).clone()).lock().unwrap()();
                    (*last_args.lock().unwrap()) = Some((args).clone());
                    (*trailing_timer.lock().unwrap()) = Some(crate::set_timeout(
                        {
                            let mut last_args = last_args.clone();
                            let mut last_fired_at = last_fired_at.clone();
                            let slot = slot.clone();
                            let mut trailing_timer = trailing_timer.clone();
                            move || -> () {
                                (*last_fired_at.lock().unwrap()) = crate::flight_now_millis();
                                (*trailing_timer.lock().unwrap()) = None;
                                if ((*last_args.lock().unwrap()).clone()).is_some() {
                                    crate::FlightCallback::flight_call(
                                        &((slot).clone()),
                                        (((*last_args.lock().unwrap()).clone()).clone().unwrap())
                                            .clone(),
                                    );
                                    (*last_args.lock().unwrap()) = None;
                                }
                            }
                        },
                        remaining,
                    ));
                }
            }
        }
    });
    connect_signal(&mut source, (handler).clone(), None);
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let clear_trailing = clear_trailing.clone();
        let handler = handler.clone();
        let mut source = source.clone();
        move || -> () {
            disconnect_signal(&mut source, (handler).clone());
            ((clear_trailing).clone()).lock().unwrap()();
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
}
