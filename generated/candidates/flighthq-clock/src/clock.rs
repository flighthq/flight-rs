// @generated from upstream/packages/clock/src/clock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{clear_signal, emit_signal};
use flighthq_types::{Clock, ClockOptions, EntityConstruction};

// Source: upstream/packages/clock/src/clock.ts:7 (sha256:67ae767fcb7f925e8bdcc584678906522628b2d6627f52cd42fb5de3d302297e)
pub fn add_clock_child(parent: &mut Clock, child: &mut Clock) -> () {
    if ((child.parent).as_deref().cloned()) == Some((*parent).clone()) {
        return;
    }
    if ((child.parent).as_deref().cloned()).is_some() {
        {
            let mut __flight_argument_0 = child
                .parent
                .replace(Box::new(Default::default()))
                .expect("narrowed recursive field was absent");
            let __flight_result = remove_clock_child(&mut *__flight_argument_0, child);
            if child.parent.is_some() {
                child.parent = Some(__flight_argument_0);
            }
            __flight_result
        };
    }
    child.parent = Some(Box::new((*parent).clone()));
    parent.children.push(((*child).clone()).clone());
}

// Source: upstream/packages/clock/src/clock.ts:20 (sha256:2c46a0fc3ec795459c7c8f94d2d954344376489c79b038376f97bd26a660e2f5)
pub fn advance_clock(clock: &mut Clock, delta_time: f64) -> () {
    let scaled_delta = if clock.paused {
        0.0_f64
    } else {
        (delta_time * clock.scale)
    };
    clock.delta_time = scaled_delta;
    clock.elapsed += scaled_delta;
    if ((clock.on_tick).clone()).is_some() {
        emit_signal(((clock.on_tick).clone()).unwrap(), (scaled_delta,));
    }
    {
        let mut i = 0.0_f64;
        while (i < (clock.children.len() as f64)) {
            advance_clock(&mut clock.children[i as usize], scaled_delta);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/clock/src/clock.ts:33 (sha256:827988cde163cc0a8c19ea99806fa81cc5522fcb1c54a74cd7c970613566273a)
pub fn create_child_clock(parent: &mut Clock, options: Option<ClockOptions>) -> Clock {
    let mut child = create_clock(((options).clone()).clone());
    add_clock_child(parent, &mut child);
    return child;
}

// Source: upstream/packages/clock/src/clock.ts:39 (sha256:d0d4db9b28e952d50f5d35f7a352eac2f38cd74906a5d2c0dcce8af5797c0ab0)
pub fn create_clock(options: Option<ClockOptions>) -> Clock {
    let mut out = allocate_entity();
    initialize_clock((out).clone(), ((options).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/clock/src/clock.ts:49 (sha256:d654ecb7d75778d2bc4b2fbc2fe90c930a32e598a4639a4a91b284c11985718c)
pub fn dispose_clock(clock: &mut Clock) -> () {
    if ((clock.parent).as_deref().cloned()).is_some() {
        {
            let mut __flight_argument_0 = clock
                .parent
                .replace(Box::new(Default::default()))
                .expect("narrowed recursive field was absent");
            let __flight_result = remove_clock_child(&mut *__flight_argument_0, clock);
            if clock.parent.is_some() {
                clock.parent = Some(__flight_argument_0);
            }
            __flight_result
        };
    }
    {
        let mut i = 0.0_f64;
        while (i < (clock.children.len() as f64)) {
            clock.children[i as usize].parent = None;
            {
                i += 1.0;
                i
            };
        }
    }
    clock.children.clear();
    if ((clock.on_tick).clone()).is_some() {
        clear_signal(clock.on_tick.as_mut().unwrap());
    }
}

// Source: upstream/packages/clock/src/clock.ts:62 (sha256:117a788cea03f0d05b139bdaf6200843d508a8b743234552ac899617eebb73ed)
pub fn get_clock_effective_scale(clock: &Clock) -> f64 {
    let mut scale = clock.scale;
    let mut current: Option<Clock> = (clock.parent).as_deref().cloned();
    while (current).is_some() {
        scale *= current.as_mut().unwrap().scale;
        current = (current.as_mut().unwrap().parent).as_deref().cloned();
    }
    return scale;
}

// Source: upstream/packages/clock/src/clock.ts:74 (sha256:63fab6c43184be1e74ebf0e90578139752db73173af703fd4feb5554cc79242e)
pub fn get_clock_parent(clock: &Clock) -> Option<Clock> {
    return (clock.parent).as_deref().cloned();
}

// Source: upstream/packages/clock/src/clock.ts:80 (sha256:80f1f75936ec0ba8a8bdede78710d7f6dd321d96a230b747377bd0468d2a1da9)
pub fn initialize_clock(out: EntityConstruction<Clock>, options: Option<ClockOptions>) -> () {
    crate::host_set(
        "host.scale",
        (options.as_ref().and_then(|value| value.scale)).unwrap_or(1.0_f64),
    );
    crate::host_set(
        "host.paused",
        (options.as_ref().and_then(|value| value.paused)).unwrap_or(false),
    );
    crate::host_set("host.deltaTime", 0.0_f64);
    crate::host_set("host.elapsed", 0.0_f64);
    crate::host_set("host.parent", None);
    crate::host_set("host.children", vec![]);
    crate::host_set("host.onTick", None);
}

// Source: upstream/packages/clock/src/clock.ts:92 (sha256:94d1d81f53c1df88a647fb9f771812bec263d9a53a8c84b6554ca25f3bf79f40)
pub fn is_clock_effectively_paused(clock: &Clock) -> bool {
    let mut current: Option<Clock> = Some((*clock).clone());
    while (current).is_some() {
        if current.as_mut().unwrap().paused {
            return true;
        }
        current = (current.as_mut().unwrap().parent).as_deref().cloned();
    }
    return false;
}

// Source: upstream/packages/clock/src/clock.ts:103 (sha256:46f90f82801a01378831c3c91036b22a58c99217be9b8a49b52a3ad5a97ea7fd)
pub fn pause_clock(clock: &mut Clock) -> () {
    clock.paused = true;
}

// Source: upstream/packages/clock/src/clock.ts:109 (sha256:86c295204a18e48d81ef51eb8cbed41d659c00bd2cfce9de166dff50b40fd3a5)
pub fn remove_clock_child(parent: &mut Clock, child: &mut Clock) -> () {
    let index = {
        let __flight_value = (*child).clone();
        (parent.children)
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (index == (-1.0_f64)) {
        return;
    }
    {
        let __flight_start = (index);
        let __flight_count = (1.0_f64);
        parent
            .children
            .splice(
                (__flight_start) as usize..(__flight_start + __flight_count) as usize,
                vec![],
            )
            .collect::<Vec<_>>()
    };
    child.parent = None;
}

// Source: upstream/packages/clock/src/clock.ts:118 (sha256:5ec957e1ea5cf0054e328d3a11d0f50be72ce669ff436f5c45b893b299662fa9)
pub fn reset_clock(clock: &mut Clock) -> () {
    clock.elapsed = 0.0_f64;
    clock.delta_time = 0.0_f64;
}

// Source: upstream/packages/clock/src/clock.ts:125 (sha256:391914411c72d6c22b6b875b5cf8bd4e95401569e9b451e825fb9c801a7a49f6)
pub fn resume_clock(clock: &mut Clock) -> () {
    clock.paused = false;
}

// Source: upstream/packages/clock/src/clock.ts:131 (sha256:009ae0c20f8b455ead83020a0e6b90a84bfbd2f4e380869f204bc7973fcea8ab)
pub fn set_clock_scale(clock: &mut Clock, scale: f64) -> () {
    clock.scale = scale;
}
