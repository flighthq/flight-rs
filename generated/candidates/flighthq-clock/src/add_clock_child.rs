// @generated from upstream/packages/clock/src/addClockChild.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Clock;

// Source: upstream/packages/clock/src/addClockChild.ts:5 (sha256:67ae767fcb7f925e8bdcc584678906522628b2d6627f52cd42fb5de3d302297e)
pub fn add_clock_child(parent: &mut Clock, child: &mut Clock) -> () {
    if ((child.parent).clone()) == Some((*parent).clone()) {
        return;
    }
    if ((child.parent).clone()).is_some() {
        remove_clock_child(child.parent.as_mut().unwrap(), child);
    }
    child.parent = Some((*parent).clone());
    parent.children.push(((*child).clone()).clone());
}

// Source: upstream/packages/clock/src/addClockChild.ts:14 (sha256:86c295204a18e48d81ef51eb8cbed41d659c00bd2cfce9de166dff50b40fd3a5)
pub fn remove_clock_child(parent: &mut Clock, child: &mut Clock) -> () {
    let index = {
        let __flight_value = (*child).clone();
        ((parent.children).clone())
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (index == (-1.0_f64)) {
        return;
    }
    parent
        .children
        .splice((index) as usize..((index) + (1.0_f64)) as usize, vec![]);
    child.parent = None;
}
