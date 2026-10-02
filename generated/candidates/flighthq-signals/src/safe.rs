// @generated from upstream/packages/signals/src/safe.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Signal, SignalData};

// Source: upstream/packages/signals/src/safe.ts:14 (sha256:e66fffcde24bd1f0ca5235cb4e8d0b000601b1e92c51cda66672adf1b4cd44c1)
pub fn emit_signal_safe<T: crate::FlightCallback>(
    signal: &mut Signal<T>,
    args: <T as crate::FlightCallback>::Args,
) -> () {
    let mut data = (signal.data).clone();
    if (data).is_none() {
        return;
    }
    let slots = ((data.as_mut().unwrap().inner.lock().unwrap().slots).clone()).clone();
    let priorities = ((data.as_mut().unwrap().inner.lock().unwrap().priorities).clone()).clone();
    let repeat = ((data.as_mut().unwrap().inner.lock().unwrap().repeat).clone()).clone();
    data.as_mut().unwrap().inner.lock().unwrap().cancelled = false;
    {
        data.as_mut().unwrap().inner.lock().unwrap().depth += 1.0;
        data.as_mut().unwrap().inner.lock().unwrap().depth
    };
    {
        {
            let mut i = 0.0_f64;
            while (i < (slots.len() as f64)) {
                let slot: Option<T> = slots.get(i as usize).cloned().flatten();
                if crate::FlightCallback::flight_same(&((slot).clone()), &(None)) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                if (!repeat[i as usize].clone()) {
                    tombstone_once_slot(
                        (data.as_mut().unwrap()).clone(),
                        (((slot).clone()).clone().unwrap()).clone(),
                        priorities[i as usize].clone(),
                    );
                }
                crate::FlightCallback::flight_call(&((slot).clone()), ((args).clone()).clone());
                if data.as_mut().unwrap().inner.lock().unwrap().cancelled {
                    break;
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    {
        {
            data.as_mut().unwrap().inner.lock().unwrap().depth -= 1.0;
            data.as_mut().unwrap().inner.lock().unwrap().depth
        };
        if (data.as_mut().unwrap().inner.lock().unwrap().depth == 0.0_f64) {
            compact_signal_data(signal, (data.as_mut().unwrap()).clone());
        }
    }
}

// Source: upstream/packages/signals/src/safe.ts:39 (sha256:23167867fdf636fddefa29708b3766dcd2c81e771ff9423e8c971a04c7e1fc01)
fn tombstone_once_slot<T: crate::FlightCallback>(
    mut data: SignalData<T>,
    slot: T,
    priority: f64,
) -> () {
    {
        let mut i = 0.0_f64;
        while (i < (data.inner.lock().unwrap().slots.len() as f64)) {
            if ((!((data.inner.lock().unwrap().slots[i as usize].clone()) == Some((slot).clone())))
                || (data.inner.lock().unwrap().repeat[i as usize].clone()))
                || (data.inner.lock().unwrap().priorities[i as usize].clone() != priority)
            {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            {
                let __flight_index = (i) as usize;
                let __flight_value = None;
                if __flight_index == data.inner.lock().unwrap().slots.len() {
                    data.inner.lock().unwrap().slots.push(__flight_value);
                } else {
                    data.inner.lock().unwrap().slots[__flight_index] = __flight_value;
                }
            };
            return;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/signals/src/safe.ts:47 (sha256:c6babc43ea7f80c6a856fd2ff855e2d4653c405cb4bcd7380893f53e08e2fc49)
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
