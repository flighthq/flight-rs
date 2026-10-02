// @generated from upstream/packages/flow/src/flow.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, FlowStack, FlowState, LogData, LogDataProvider};

// Source: upstream/packages/flow/src/flow.ts:9 (sha256:c807f627ebaf016391700043b587ec56a4c00cf6083c98ae65e4bb8738751c96)
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

// Source: upstream/packages/flow/src/flow.ts:17 (sha256:737363bcdccaa46960292e79caf2878cf0dc985eebc1e87944e95b913181aae8)
pub fn create_flow_stack() -> FlowStack {
    let mut out = allocate_entity();
    initialize_flow_stack((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/flow/src/flow.ts:25 (sha256:2cf869d1898ccb66ba939fdb648b6c9e49d16aa544aa157d81ec455698dc4b5d)
pub fn get_active_flow_state(stack: &FlowStack) -> Option<FlowState> {
    return if ((stack.states.len() as f64) > 0.0_f64) {
        Some(stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone())
    } else {
        None
    };
}

// Source: upstream/packages/flow/src/flow.ts:31 (sha256:859731f97f3b4ad8ec9d9a781a9b4760bca6ac828425799a581c852010847a4a)
pub fn get_flow_stack_depth(stack: &FlowStack) -> f64 {
    return (stack.states.len() as f64);
}

// Source: upstream/packages/flow/src/flow.ts:41 (sha256:1db2c021ed4c1ea7e5cfe34c5df44f206dd6f94d22c9b61b469f523112288ab5)
pub fn get_flow_stack_visible_states(stack: &FlowStack, out: &mut Vec<FlowState>) -> () {
    out.clear();
    let top = ((stack.states.len() as f64) - 1.0_f64);
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

// Source: upstream/packages/flow/src/flow.ts:59 (sha256:93b03c260a18521af15cf8ddc3315d1a7a86021b5ba8da3d4f6f17299b1e03ca)
pub fn initialize_flow_stack(out: EntityConstruction<FlowStack>) -> () {
    crate::host_set("host.states", vec![]);
}

// Source: upstream/packages/flow/src/flow.ts:66 (sha256:d91786875c92cc6fbc3cc9a868990e775b286c0cc40fe97501a7e1f73467d59b)
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

// Source: upstream/packages/flow/src/flow.ts:81 (sha256:de184bd5c87b7744c0c586250873ed01b1dc770cf0d0f404c89b94e2af9cf904)
pub fn push_flow_state(stack: &mut FlowStack, state: &FlowState) -> () {
    if ((*TRANSITION_DEPTH.lock().unwrap()).clone() > 0.0_f64) {
        ({
            #[derive(Clone, Default)]
            struct ClosureRecord1 {
                __flight_identity: std::sync::Arc<()>,
                kind: String,
            }
            impl PartialEq for ClosureRecord1 {
                fn eq(&self, other: &Self) -> bool {
                    std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
                }
            }

            || -> () {
                if (!enabled) {
                    return;
                }
                log_once(
                    format!("flow:{}", "transition-during-transition".to_owned()),
                    LogLevel::Warn,
                    &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                        String,
                        Vec<(String, crate::FlightValue)>,
                    >::B(
                        {
                        let mut __flight_record = Vec::new();
                        __flight_record.push(("kind".to_owned(), {
                            let __flight_portable_source =
                                "transition-during-transition".to_owned();
                            crate::FlightValue::String((&__flight_portable_source).clone())
                        }));
                        __flight_record
                    }
                    ))),
                    Some(("flow".to_owned()).clone()),
                );
            }
        })();
    }
    if {
        let __flight_value = (*state).clone();
        (stack.states).iter().any(|item| item == &__flight_value)
    } {
        ({
            #[derive(Clone, Default)]
            struct ClosureRecord1 {
                __flight_identity: std::sync::Arc<()>,
                kind: String,
            }
            impl PartialEq for ClosureRecord1 {
                fn eq(&self, other: &Self) -> bool {
                    std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
                }
            }

            || -> () {
                if (!enabled) {
                    return;
                }
                log_once(
                    format!("flow:{}", "duplicate-state-push".to_owned()),
                    LogLevel::Warn,
                    &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                        String,
                        Vec<(String, crate::FlightValue)>,
                    >::B(
                        {
                        let mut __flight_record = Vec::new();
                        __flight_record.push(("kind".to_owned(), {
                            let __flight_portable_source = "duplicate-state-push".to_owned();
                            crate::FlightValue::String((&__flight_portable_source).clone())
                        }));
                        __flight_record
                    }
                    ))),
                    Some(("flow".to_owned()).clone()),
                );
            }
        })();
    }
    let previous_top = if ((stack.states.len() as f64) > 0.0_f64) {
        Some(stack.states[((stack.states.len() as f64) - 1.0_f64) as usize].clone())
    } else {
        None
    };
    {
        (*TRANSITION_DEPTH.lock().unwrap()) += 1.0;
        (*TRANSITION_DEPTH.lock().unwrap())
    };
    {
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
    {
        {
            (*TRANSITION_DEPTH.lock().unwrap()) -= 1.0;
            (*TRANSITION_DEPTH.lock().unwrap())
        };
    }
}

// Source: upstream/packages/flow/src/flow.ts:100 (sha256:2af7005b06f7db90ca28b0e6cb078138bfd7f63e6f4a82dc288c4f8accab76ca)
pub fn replace_flow_state(stack: &mut FlowStack, state: &FlowState) -> () {
    if ((*TRANSITION_DEPTH.lock().unwrap()).clone() > 0.0_f64) {
        ({
            #[derive(Clone, Default)]
            struct ClosureRecord1 {
                __flight_identity: std::sync::Arc<()>,
                kind: String,
            }
            impl PartialEq for ClosureRecord1 {
                fn eq(&self, other: &Self) -> bool {
                    std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
                }
            }

            || -> () {
                if (!enabled) {
                    return;
                }
                log_once(
                    format!("flow:{}", "transition-during-transition".to_owned()),
                    LogLevel::Warn,
                    &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                        String,
                        Vec<(String, crate::FlightValue)>,
                    >::B(
                        {
                        let mut __flight_record = Vec::new();
                        __flight_record.push(("kind".to_owned(), {
                            let __flight_portable_source =
                                "transition-during-transition".to_owned();
                            crate::FlightValue::String((&__flight_portable_source).clone())
                        }));
                        __flight_record
                    }
                    ))),
                    Some(("flow".to_owned()).clone()),
                );
            }
        })();
    }
    if ((stack.states.len() as f64) > 0.0_f64) {
        let previous_top = stack.states.pop().unwrap();
        {
            (*TRANSITION_DEPTH.lock().unwrap()) += 1.0;
            (*TRANSITION_DEPTH.lock().unwrap())
        };
        {
            {
                let __flight_callback = (previous_top.on_exit).clone();
                __flight_callback
                    .as_ref()
                    .map(|callback| callback.lock().unwrap()())
            };
        }
        {
            {
                (*TRANSITION_DEPTH.lock().unwrap()) -= 1.0;
                (*TRANSITION_DEPTH.lock().unwrap())
            };
        }
    }
    stack.states.push(((*state).clone()).clone());
    {
        (*TRANSITION_DEPTH.lock().unwrap()) += 1.0;
        (*TRANSITION_DEPTH.lock().unwrap())
    };
    {
        {
            let __flight_callback = (state.on_enter).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()())
        };
    }
    {
        {
            (*TRANSITION_DEPTH.lock().unwrap()) -= 1.0;
            (*TRANSITION_DEPTH.lock().unwrap())
        };
    }
}

// Source: upstream/packages/flow/src/flow.ts:121 (sha256:0322c4c80cc0df0bc7485f51c4594f20331897eec8e4437c8eda0496f92beaf0)
static TRANSITION_DEPTH: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/flow/src/flow.ts:128 (sha256:eb688c48071945a1aabcc22f1874b54a7c97c5f3319211d91ef76baf7a74855f)
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
