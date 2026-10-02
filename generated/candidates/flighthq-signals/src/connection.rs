// @generated from upstream/packages/signals/src/connection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{connect_signal, disconnect_signal};
use flighthq_types::{Signal, SignalConnectOptions, SignalConnection, SignalTrackedConnectOptions};

// Source: upstream/packages/signals/src/connection.ts:10 (sha256:00b8596f0748d1f357bbdc438723a38f7ea894b5b0f11530bf0f659de78dec05)
#[derive(Clone, Default)]
struct ConnectSignalTrackedSynthesizedRecord1538541574 {
    __flight_identity: std::sync::Arc<()>,
    priority: f64,
}
impl PartialEq for ConnectSignalTrackedSynthesizedRecord1538541574 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn connect_signal_tracked<T: crate::FlightCallback>(
    signal: &mut Signal<T>,
    slot: T,
    mut options: Option<SignalTrackedConnectOptions>,
) -> SignalConnection<T> {
    let connection: std::sync::Arc<std::sync::Mutex<SignalConnection<T>>> =
        std::sync::Arc::new(std::sync::Mutex::new(SignalConnection::<T> {
            __flight_identity: std::sync::Arc::new(()),
            connected: true,
            paused: false,
            signal: (*signal).clone(),
            slot: (slot).clone(),
        }));
    let once = (options.as_ref().and_then(|value| value.once)).unwrap_or(false);
    let tracked_slot = T::flight_from_tuple_callback({
        let mut connection = connection.clone();
        let slot = slot.clone();
        move |args: <T as crate::FlightCallback>::Args| -> () {
            if (*connection.lock().unwrap()).paused {
                return;
            }
            if once {
                disconnect_signal_connection(&mut (*connection.lock().unwrap()));
            }
            crate::FlightCallback::flight_call(&((slot).clone()), ((args).clone()).clone());
        }
    });
    (*connection.lock().unwrap()).slot = (tracked_slot).clone();
    let priority = options.as_ref().and_then(|value| value.priority);
    connect_signal(
        (*connection.lock().unwrap()).signal,
        (tracked_slot).clone(),
        (if (priority).is_none() {
            None
        } else {
            Some(ConnectSignalTrackedSynthesizedRecord1538541574 {
                __flight_identity: std::sync::Arc::new(()),
                priority: *(priority.as_ref().unwrap()),
            })
        })
        .as_ref()
        .map(|__flight_value| {
            let __flight_source = &(__flight_value);
            SignalConnectOptions {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                once: None,
                priority: Some(__flight_source.priority),
            }
        }),
    );
    options
        .as_mut()
        .unwrap()
        .scope
        .as_mut()
        .unwrap()
        .connections
        .as_mut()
        .unwrap()
        .push((*connection.lock().unwrap()).clone());
    return (*connection.lock().unwrap()).clone();
}

// Source: upstream/packages/signals/src/connection.ts:32 (sha256:e8ecb0873dcfc0c85b2baccfc7f9b0f94bf7d30195ddf62d0a111c3bdb9b5170)
pub fn disconnect_signal_connection<T: crate::FlightCallback>(
    connection: &mut SignalConnection<T>,
) -> () {
    if (!connection.connected) {
        return;
    }
    connection.connected = false;
    {
        let __flight_argument_1 = (connection.slot).clone();
        let __flight_result = disconnect_signal(&mut connection.signal, __flight_argument_1);
        __flight_result
    };
}

// Source: upstream/packages/signals/src/connection.ts:38 (sha256:b65de278803d121637adf19960b4b4af331450d99c60d5b6ff42296f24b1ee23)
pub fn pause_signal_connection<T: crate::FlightCallback>(
    connection: &mut SignalConnection<T>,
) -> () {
    if connection.connected {
        connection.paused = true;
    }
}

// Source: upstream/packages/signals/src/connection.ts:42 (sha256:a3adcc604071a0919ca7080c7bec2c8c733795c704d17b749470217e8b1e8596)
pub fn resume_signal_connection<T: crate::FlightCallback>(
    connection: &mut SignalConnection<T>,
) -> () {
    if connection.connected {
        connection.paused = false;
    }
}
