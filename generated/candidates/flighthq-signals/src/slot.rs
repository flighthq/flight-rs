// @generated from upstream/packages/signals/src/slot.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Signal, SignalConnectOptions, SignalData};

// Source: upstream/packages/signals/src/slot.ts:7 (sha256:45811039b54e4f3299e78de23cdf80c2c1f8a8158edb7ffa3c483cc000dc1d33)
pub fn clear_signal<T: crate::FlightCallback>(signal: &mut Signal<T>) -> () {
    signal.emit = T::flight_noop();
    signal.data = None;
}

// Source: upstream/packages/signals/src/slot.ts:12 (sha256:3372b85168d969415a4ac2283d3e46ef757a1cde4ebb3a5d018b498208fd8eac)
pub fn connect_signal<T: crate::FlightCallback>(
    signal: &mut Signal<T>,
    slot: T,
    options: Option<SignalConnectOptions>,
) -> () {
    let priority = (options.as_ref().and_then(|value| value.priority)).unwrap_or(0.0_f64);
    let repeat = (!(options.as_ref().and_then(|value| value.once)).unwrap_or(false));
    init_signal(signal);
    let mut data = (signal.data).clone();
    {
        let mut i = 0.0_f64;
        while (i
            < (data
                .as_ref()
                .unwrap()
                .inner
                .lock()
                .unwrap()
                .priorities
                .len() as f64))
        {
            if (priority
                > data.as_ref().unwrap().inner.lock().unwrap().priorities[i as usize].clone())
            {
                data.as_ref().unwrap().inner.lock().unwrap().slots.splice(
                    (i) as usize..((i) + (0.0_f64)) as usize,
                    vec![(slot).clone()],
                );
                data.as_ref()
                    .unwrap()
                    .inner
                    .lock()
                    .unwrap()
                    .priorities
                    .splice((i) as usize..((i) + (0.0_f64)) as usize, vec![priority]);
                data.as_ref()
                    .unwrap()
                    .inner
                    .lock()
                    .unwrap()
                    .repeat
                    .splice((i) as usize..((i) + (0.0_f64)) as usize, vec![repeat]);
                return;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    data.as_ref()
        .unwrap()
        .inner
        .lock()
        .unwrap()
        .slots
        .push(((slot).clone()).clone());
    data.as_ref()
        .unwrap()
        .inner
        .lock()
        .unwrap()
        .priorities
        .push(priority);
    data.as_ref()
        .unwrap()
        .inner
        .lock()
        .unwrap()
        .repeat
        .push(repeat);
}

// Source: upstream/packages/signals/src/slot.ts:37 (sha256:92c7597cd586af0cffbff07064b94dbac151c645a68af8d0b4e9d73acddb6435)
pub fn disconnect_signal<T: crate::FlightCallback>(signal: &mut Signal<T>, slot: T) -> () {
    let mut data = (signal.data).clone();
    if (data).is_none() {
        return;
    }
    let mut i = (data.as_mut().unwrap().inner.lock().unwrap().slots.len() as f64);
    while ({
        i -= 1.0;
        i
    } >= 0.0_f64)
    {
        if crate::FlightCallback::flight_same(
            &(data.as_mut().unwrap().inner.lock().unwrap().slots[i as usize].clone()),
            &((slot).clone()),
        ) {
            data.as_mut()
                .unwrap()
                .inner
                .lock()
                .unwrap()
                .slots
                .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
            data.as_mut()
                .unwrap()
                .inner
                .lock()
                .unwrap()
                .priorities
                .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
            data.as_mut()
                .unwrap()
                .inner
                .lock()
                .unwrap()
                .repeat
                .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
        }
    }
    if ((data.as_mut().unwrap().inner.lock().unwrap().slots.len() as f64) == 0.0_f64) {
        signal.emit = T::flight_noop();
        signal.data = None;
    }
}

// Source: upstream/packages/signals/src/slot.ts:56 (sha256:f9e7f2c203efab8f134bd3da34dfcd6cc8077c614fe6d121d664a36a7a643aeb)
pub fn has_signal_slots<T: crate::FlightCallback>(signal: Signal<T>) -> bool {
    return (((signal.data).clone()).is_some())
        && ((signal
            .data
            .as_ref()
            .unwrap()
            .inner
            .lock()
            .unwrap()
            .slots
            .len() as f64)
            > 0.0_f64);
}

// Source: upstream/packages/signals/src/slot.ts:60 (sha256:53d4a7642c70a243bfa1ebc3d8dfea148586739a29ffb797616a6740ac9a20fc)
fn init_signal<T: crate::FlightCallback>(signal: &mut Signal<T>) -> () {
    if ((signal.data).clone()).is_some() {
        return;
    }
    let mut data: SignalData<T> = SignalData::<T>::new(vec![], vec![], vec![], false);
    signal.data = Some((data).clone());
    signal.emit = make_dispatch((data).clone());
}

// Source: upstream/packages/signals/src/slot.ts:67 (sha256:8b42fc0718c235693ee2db00927e515b1d37487376359b5c5ff12cd9801e8359)
pub fn is_slot_connected<T: crate::FlightCallback>(signal: Signal<T>, slot: T) -> bool {
    return (((signal.data).clone()).is_some())
        && ({
            let __flight_value = (slot).clone();
            ((signal.data.as_ref().unwrap().inner.lock().unwrap().slots).clone())
                .iter()
                .position(|item| crate::FlightCallback::flight_same(item, &__flight_value))
                .map_or(-1.0_f64, |index| index as f64)
        } != (-1.0_f64));
}

// Source: upstream/packages/signals/src/slot.ts:71 (sha256:1256e8499e3f9e2364648cbcc28a8d8405445b32c0ea7b2b37e74c6a229bd329)
fn make_dispatch<T: crate::FlightCallback>(mut data: SignalData<T>) -> T {
    return T::flight_from_tuple_callback({
        let mut data = data.clone();
        move |args: <T as crate::FlightCallback>::Args| -> () {
            data.inner.lock().unwrap().cancelled = false;
            let mut i = 0.0_f64;
            while (i < (data.inner.lock().unwrap().slots.len() as f64)) {
                crate::FlightCallback::flight_call(
                    &(data.inner.lock().unwrap().slots[i as usize].clone()),
                    ((args).clone()).clone(),
                );
                if data.inner.lock().unwrap().cancelled {
                    break;
                }
                if (!data.inner.lock().unwrap().repeat[i as usize].clone()) {
                    data.inner
                        .lock()
                        .unwrap()
                        .slots
                        .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
                    data.inner
                        .lock()
                        .unwrap()
                        .priorities
                        .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
                    data.inner
                        .lock()
                        .unwrap()
                        .repeat
                        .splice((i) as usize..((i) + (1.0_f64)) as usize, vec![]);
                } else {
                    {
                        i += 1.0;
                        i
                    };
                }
            }
        }
    });
}
