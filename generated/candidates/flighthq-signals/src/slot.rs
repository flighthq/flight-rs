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
                {
                    let __flight_start = (i);
                    let __flight_count = (0.0_f64);
                    data.as_ref()
                        .unwrap()
                        .inner
                        .lock()
                        .unwrap()
                        .slots
                        .splice(
                            (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                            vec![Some((slot).clone())],
                        )
                        .collect::<Vec<_>>()
                };
                {
                    let __flight_start = (i);
                    let __flight_count = (0.0_f64);
                    data.as_ref()
                        .unwrap()
                        .inner
                        .lock()
                        .unwrap()
                        .priorities
                        .splice(
                            (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                            vec![priority],
                        )
                        .collect::<Vec<_>>()
                };
                {
                    let __flight_start = (i);
                    let __flight_count = (0.0_f64);
                    data.as_ref()
                        .unwrap()
                        .inner
                        .lock()
                        .unwrap()
                        .repeat
                        .splice(
                            (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                            vec![repeat],
                        )
                        .collect::<Vec<_>>()
                };
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
        .push((Some((slot).clone())).clone());
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

// Source: upstream/packages/signals/src/slot.ts:37 (sha256:34757d0325d95ff39bb73115cd0eee421a1ba2a38603a51c055a2003c131802a)
pub fn disconnect_signal<T: crate::FlightCallback>(signal: &mut Signal<T>, slot: T) -> () {
    let mut data = (signal.data).clone();
    if (data).is_none() {
        return;
    }
    let dispatching = (data.as_mut().unwrap().inner.lock().unwrap().depth > 0.0_f64);
    let mut i = (data.as_mut().unwrap().inner.lock().unwrap().slots.len() as f64);
    while ({
        i -= 1.0;
        i
    } >= 0.0_f64)
    {
        if !((data.as_mut().unwrap().inner.lock().unwrap().slots[i as usize].clone())
            == Some((slot).clone()))
        {
            continue;
        }
        if dispatching {
            {
                let __flight_index = (i) as usize;
                let __flight_value = None;
                if __flight_index == data.as_mut().unwrap().inner.lock().unwrap().slots.len() {
                    data.as_mut()
                        .unwrap()
                        .inner
                        .lock()
                        .unwrap()
                        .slots
                        .push(__flight_value);
                } else {
                    data.as_mut().unwrap().inner.lock().unwrap().slots[__flight_index] =
                        __flight_value;
                }
            };
            continue;
        }
        {
            let __flight_start = (i);
            let __flight_count = (1.0_f64);
            data.as_mut()
                .unwrap()
                .inner
                .lock()
                .unwrap()
                .slots
                .splice(
                    (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                    vec![],
                )
                .collect::<Vec<_>>()
        };
        {
            let __flight_start = (i);
            let __flight_count = (1.0_f64);
            data.as_mut()
                .unwrap()
                .inner
                .lock()
                .unwrap()
                .priorities
                .splice(
                    (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                    vec![],
                )
                .collect::<Vec<_>>()
        };
        {
            let __flight_start = (i);
            let __flight_count = (1.0_f64);
            data.as_mut()
                .unwrap()
                .inner
                .lock()
                .unwrap()
                .repeat
                .splice(
                    (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                    vec![],
                )
                .collect::<Vec<_>>()
        };
    }
    if (!dispatching)
        && ((data.as_mut().unwrap().inner.lock().unwrap().slots.len() as f64) == 0.0_f64)
    {
        signal.emit = T::flight_noop();
        signal.data = None;
    }
}

// Source: upstream/packages/signals/src/slot.ts:64 (sha256:849b9747b53d9deb9246a9bb0a50af56a5c20505cf55bd73a3bfafef5640f661)
pub fn has_signal_slots<T: crate::FlightCallback>(signal: Signal<T>) -> bool {
    let data = (signal.data).clone();
    if (data).is_none() {
        return false;
    }
    if (data.as_ref().unwrap().inner.lock().unwrap().depth == 0.0_f64) {
        return ((data.as_ref().unwrap().inner.lock().unwrap().slots.len() as f64) > 0.0_f64);
    }
    return (count_live_slots((data.as_ref().unwrap()).clone()) > 0.0_f64);
}

// Source: upstream/packages/signals/src/slot.ts:74 (sha256:3058fcfeb66ee19d52f5f238c9cd0f9c2cf2d50c87230ae84c0495d4094d16ed)
fn init_signal<T: crate::FlightCallback>(signal: &mut Signal<T>) -> () {
    if ((signal.data).clone()).is_some() {
        return;
    }
    let mut data: SignalData<T> = SignalData::<T>::new(vec![], vec![], vec![], false, 0.0_f64);
    signal.data = Some((data).clone());
    signal.emit = make_dispatch((signal).clone(), (data).clone());
}

// Source: upstream/packages/signals/src/slot.ts:81 (sha256:8b42fc0718c235693ee2db00927e515b1d37487376359b5c5ff12cd9801e8359)
pub fn is_slot_connected<T: crate::FlightCallback>(signal: Signal<T>, slot: T) -> bool {
    return (((signal.data).clone()).is_some())
        && ({
            let __flight_value = Some((slot).clone());
            (signal.data.as_ref().unwrap().inner.lock().unwrap().slots)
                .iter()
                .position(|item| item == &__flight_value)
                .map_or(-1.0_f64, |index| index as f64)
        } != (-1.0_f64));
}

// Source: upstream/packages/signals/src/slot.ts:87 (sha256:44f298b854cd024417af92f19af94a7f768cb97dd20d2fae417c5b190c2644ef)
fn make_dispatch<T: crate::FlightCallback>(mut signal: Signal<T>, mut data: SignalData<T>) -> T {
    return T::flight_from_tuple_callback({
        let mut data = data.clone();
        let mut signal = signal.clone();
        move |args: <T as crate::FlightCallback>::Args| -> () {
            data.inner.lock().unwrap().cancelled = false;
            {
                data.inner.lock().unwrap().depth += 1.0;
                data.inner.lock().unwrap().depth
            };
            let mut i = 0.0_f64;
            while (i < (data.inner.lock().unwrap().slots.len() as f64)) {
                let slot: Option<T> = data
                    .inner
                    .lock()
                    .unwrap()
                    .slots
                    .get(i as usize)
                    .cloned()
                    .flatten();
                if crate::FlightCallback::flight_same(&(slot), &(None)) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                crate::FlightCallback::flight_call(&(slot), ((args).clone()).clone());
                if data.inner.lock().unwrap().cancelled {
                    break;
                }
                if (!data.inner.lock().unwrap().repeat[i as usize].clone()) {
                    {
                        let __flight_index = (i) as usize;
                        let __flight_value = None;
                        if __flight_index == data.inner.lock().unwrap().slots.len() {
                            data.inner.lock().unwrap().slots.push(__flight_value);
                        } else {
                            data.inner.lock().unwrap().slots[__flight_index] = __flight_value;
                        }
                    };
                }
                {
                    i += 1.0;
                    i
                };
            }
            {
                data.inner.lock().unwrap().depth -= 1.0;
                data.inner.lock().unwrap().depth
            };
            if (data.inner.lock().unwrap().depth == 0.0_f64) {
                compact_signal_data(&mut signal, (data).clone());
            }
        }
    });
}

// Source: upstream/packages/signals/src/slot.ts:110 (sha256:c6babc43ea7f80c6a856fd2ff855e2d4653c405cb4bcd7380893f53e08e2fc49)
fn compact_signal_data<T: crate::FlightCallback>(
    signal: &mut Signal<T>,
    mut data: SignalData<T>,
) -> () {
    let mut write = 0.0_f64;
    {
        let mut read = 0.0_f64;
        while (read < (data.inner.lock().unwrap().slots.len() as f64)) {
            if data
                .inner
                .lock()
                .unwrap()
                .slots
                .get((read) as usize)
                .is_none()
            {
                {
                    read += 1.0;
                    read
                };
                continue;
            }
            if (write != read) {
                {
                    let __flight_index = (write) as usize;
                    let __flight_value = data.inner.lock().unwrap().slots[read as usize].clone();
                    if __flight_index == data.inner.lock().unwrap().slots.len() {
                        data.inner.lock().unwrap().slots.push(__flight_value);
                    } else {
                        data.inner.lock().unwrap().slots[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (write) as usize;
                    let __flight_value =
                        data.inner.lock().unwrap().priorities[read as usize].clone();
                    if __flight_index == data.inner.lock().unwrap().priorities.len() {
                        data.inner.lock().unwrap().priorities.push(__flight_value);
                    } else {
                        data.inner.lock().unwrap().priorities[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (write) as usize;
                    let __flight_value = data.inner.lock().unwrap().repeat[read as usize].clone();
                    if __flight_index == data.inner.lock().unwrap().repeat.len() {
                        data.inner.lock().unwrap().repeat.push(__flight_value);
                    } else {
                        data.inner.lock().unwrap().repeat[__flight_index] = __flight_value;
                    }
                };
            }
            {
                write += 1.0;
                write
            };
            {
                read += 1.0;
                read
            };
        }
    }
    if (write == (data.inner.lock().unwrap().slots.len() as f64)) {
        return;
    }
    data.inner.lock().unwrap().slots.truncate((write) as usize);
    data.inner
        .lock()
        .unwrap()
        .priorities
        .truncate((write) as usize);
    data.inner.lock().unwrap().repeat.truncate((write) as usize);
    if (write == 0.0_f64) && (((signal.data).clone()) == Some((data).clone())) {
        signal.emit = T::flight_noop();
        signal.data = None;
    }
}

// Source: upstream/packages/signals/src/slot.ts:134 (sha256:fd16c72217b026514b8653540a8c9389f6d6254dd2cb84fb9ae14018abfd7e11)
fn count_live_slots<T: crate::FlightCallback>(data: SignalData<T>) -> f64 {
    let mut live = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (data.inner.lock().unwrap().slots.len() as f64)) {
            if !(data.inner.lock().unwrap().slots.get((i) as usize).is_none()) {
                {
                    live += 1.0;
                    live
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return live;
}
