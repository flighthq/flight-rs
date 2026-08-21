// @generated from upstream/packages/flow/src/flow.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{FlowStack, FlowState};

// Source: upstream/packages/flow/src/flow.ts:6 (sha256:c807f627ebaf016391700043b587ec56a4c00cf6083c98ae65e4bb8738751c96)
pub fn clear_flow_stack(stack: &mut FlowStack) -> () {
    {
        let mut i = ((stack.states.len() as f64) - 1.0_f64);
        while (i >= 0.0_f64) {
            {
                let __flight_callback = (stack.states[i as usize].on_exit).clone();
                __flight_callback
                    .as_ref()
                    .map(|callback| callback.lock().unwrap()())
            };
            {
                i -= 1.0;
                i
            };
        }
    }
    stack.states.clear();
}

// Source: upstream/packages/flow/src/flow.ts:16 (sha256:559dc3b60dfa4f8b221042c9de38a615a1da1be1f893c16828715200449a30ca)
pub fn create_flow_stack() -> FlowStack {
    return FlowStack {
        __flight_identity: std::sync::Arc::new(()),
        states: vec![],
    };
}

// Source: upstream/packages/flow/src/flow.ts:22 (sha256:2cf869d1898ccb66ba939fdb648b6c9e49d16aa544aa157d81ec455698dc4b5d)
pub fn get_active_flow_state(stack: &FlowStack) -> Option<FlowState> {
    return if ((stack.states.len() as f64) > 0.0_f64) {
        Some(stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone())
    } else {
        None
    };
}

// Source: upstream/packages/flow/src/flow.ts:28 (sha256:859731f97f3b4ad8ec9d9a781a9b4760bca6ac828425799a581c852010847a4a)
pub fn get_flow_stack_depth(stack: &FlowStack) -> f64 {
    return (stack.states.len() as f64);
}

// Source: upstream/packages/flow/src/flow.ts:38 (sha256:1db2c021ed4c1ea7e5cfe34c5df44f206dd6f94d22c9b61b469f523112288ab5)
pub fn get_flow_stack_visible_states(stack: &FlowStack, out: &mut Vec<FlowState>) -> () {
    out.clear();
    let mut top = ((stack.states.len() as f64) - 1.0_f64);
    if (top < 0.0_f64) {
        return;
    }
    let mut lowest = top;
    while (lowest > 0.0_f64) && ((stack.states[lowest as usize].render_below).unwrap_or(false)) {
        {
            lowest -= 1.0;
            lowest
        };
    }
    {
        let mut i = lowest;
        while (i <= top) {
            out.push(stack.states[i as usize].clone());
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/flow/src/flow.ts:57 (sha256:d91786875c92cc6fbc3cc9a868990e775b286c0cc40fe97501a7e1f73467d59b)
pub fn pop_flow_state(stack: &mut FlowStack) -> Option<FlowState> {
    if ((stack.states.len() as f64) == 0.0_f64) {
        return None;
    }
    let popped = stack.states.pop().unwrap();
    {
        let __flight_callback = (popped.on_exit).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    let revealed = if ((stack.states.len() as f64) > 0.0_f64) {
        Some(stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone())
    } else {
        None
    };
    {
        let __flight_callback = revealed
            .as_ref()
            .and_then(|value| (value.on_resume).clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    return Some((popped).clone());
}

// Source: upstream/packages/flow/src/flow.ts:72 (sha256:3e3f6d6dc3ce40bdbb2e9add91b1fe41bda7b33e690b532d08bf38efce660305)
pub fn push_flow_state(stack: &mut FlowStack, state: &FlowState) -> () {
    let previous_top = if ((stack.states.len() as f64) > 0.0_f64) {
        Some(stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone())
    } else {
        None
    };
    {
        let __flight_callback = previous_top
            .as_ref()
            .and_then(|value| (value.on_pause).clone());
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
    stack.states.push(((*state).clone()).clone());
    {
        let __flight_callback = (state.on_enter).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
}

// Source: upstream/packages/flow/src/flow.ts:84 (sha256:45909cbe992bf399b284f4d6ccc462d9c05306eb7e0d39a516feab708232d35d)
pub fn replace_flow_state(stack: &mut FlowStack, state: &FlowState) -> () {
    if ((stack.states.len() as f64) > 0.0_f64) {
        let previous_top = stack.states.pop().unwrap();
        {
            let __flight_callback = (previous_top.on_exit).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()())
        };
    }
    stack.states.push(((*state).clone()).clone());
    {
        let __flight_callback = (state.on_enter).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()())
    };
}

// Source: upstream/packages/flow/src/flow.ts:99 (sha256:eb688c48071945a1aabcc22f1874b54a7c97c5f3319211d91ef76baf7a74855f)
pub fn update_flow_stack(stack: &FlowStack, delta_time: f64) -> () {
    let mut index = ((stack.states.len() as f64) - 1.0_f64);
    if (index < 0.0_f64) {
        return;
    }
    {
        let __flight_callback = (stack.states[index as usize].on_update).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()(delta_time))
    };
    while (index > 0.0_f64) && ((stack.states[index as usize].update_below).unwrap_or(false)) {
        {
            index -= 1.0;
            index
        };
        {
            let __flight_callback = (stack.states[index as usize].on_update).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()(delta_time))
        };
    }
}
