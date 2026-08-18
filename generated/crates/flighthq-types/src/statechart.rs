// @generated from upstream/packages/types/src/Statechart.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::StatechartSignals;

// Source: upstream/packages/types/src/Statechart.ts:5 (sha256:8d3ea3e0c6ab74e1a8ec906a7b0d0dd8389263e97d790ce317c1ca2c92fe3ad2)
#[derive(Clone, Default)]
pub struct StatechartInputKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub boolean: String,
    pub number: String,
    pub trigger: String,
}
impl PartialEq for StatechartInputKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static STATECHART_INPUT_KIND: std::sync::LazyLock<StatechartInputKindValues> =
    std::sync::LazyLock::new(|| StatechartInputKindValues {
        __flight_identity: std::sync::Arc::new(()),
        boolean: "Boolean".to_owned(),
        number: "Number".to_owned(),
        trigger: "Trigger".to_owned(),
    });

// Source: upstream/packages/types/src/Statechart.ts:11 (sha256:bb689f8bc1176a9e4880c33749584d051de85eb6f30361729d88dee2d8955e64)
pub type StatechartInputKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Statechart.ts:15 (sha256:f0b18429b21bfbc477142afb8582cbb92cba362d98cc83dfdbe19d086782341e)
#[derive(Clone, Default)]
pub struct StatechartComparisonValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub equal: String,
    pub greater_than: String,
    pub greater_than_or_equal: String,
    pub less_than: String,
    pub less_than_or_equal: String,
    pub not_equal: String,
}
impl PartialEq for StatechartComparisonValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static STATECHART_COMPARISON: std::sync::LazyLock<StatechartComparisonValues> =
    std::sync::LazyLock::new(|| StatechartComparisonValues {
        __flight_identity: std::sync::Arc::new(()),
        equal: "Equal".to_owned(),
        greater_than: "GreaterThan".to_owned(),
        greater_than_or_equal: "GreaterThanOrEqual".to_owned(),
        less_than: "LessThan".to_owned(),
        less_than_or_equal: "LessThanOrEqual".to_owned(),
        not_equal: "NotEqual".to_owned(),
    });

// Source: upstream/packages/types/src/Statechart.ts:24 (sha256:0328ab67dc71833f39654a7ea41aa55d42bf52747c751fa309c7ff21b63129c0)
pub type StatechartComparison = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Statechart.ts:28 (sha256:53bef091e22da64168583e29f9589fb055017490b6c47a1e86361d07ea226e22)
pub const STATECHART_ATOMIC_STATE_KIND: &'static str = "Statechart.Atomic";

// Source: upstream/packages/types/src/Statechart.ts:29 (sha256:13cb78c9ca468d01ec112170bdd7f0195dca8336c89b7b9a2fbed8b902e4ebb9)
pub const STATECHART_NESTED_STATE_KIND: &'static str = "Statechart.Nested";

// Source: upstream/packages/types/src/Statechart.ts:32 (sha256:33eaa2d8e27482f0dd168784805292b3f4ea43bb6b6dc4828ec20cf05dc6ec46)
#[derive(Clone, Default)]
pub struct StatechartInput {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub initial_value: f64,
    pub kind: StatechartInputKind,
    pub name: Option<String>,
}
impl PartialEq for StatechartInput {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Statechart.ts:39 (sha256:3215c135be242191155c72f14deb9e2ac8380b6ca45320c66f824972a1d9f629)
#[derive(Clone, Default)]
pub struct StatechartCondition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub comparison: StatechartComparison,
    pub input_index: f64,
    pub value: f64,
}
impl PartialEq for StatechartCondition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Statechart.ts:47 (sha256:ee1a1c67324b9c8fb812541fc64da6edb13d01144a9bfd7289e47a151cefd755)
#[derive(Clone, Default)]
pub struct StatechartTransition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub conditions: Vec<StatechartCondition>,
    pub duration_ms: f64,
    pub exit_time_ratio: f64,
    pub target_state_index: f64,
}
impl PartialEq for StatechartTransition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Statechart.ts:56 (sha256:decf7fe340c128e6a1f153a139af5a745851388a7c362e4660bb156def070f05)
#[derive(Clone, Default)]
pub struct StatechartState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
    pub transitions: Vec<StatechartTransition>,
}
impl PartialEq for StatechartState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Statechart.ts:63 (sha256:882e0d77b136ee617b5ba9c0a7d578d413d16fd2a91d872175aa5135d01bfe4f)
#[derive(Clone, Default)]
pub struct StatechartRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub initial_state_index: f64,
    pub name: Option<String>,
    pub states: Vec<StatechartState>,
}
impl PartialEq for StatechartRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Statechart.ts:70 (sha256:cfe564914537b7c6b1f9b16a293e7a1b8c28d3d743a920f8054b8d1304fa39b7)
#[derive(Clone, Default)]
pub struct Statechart {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub inputs: Vec<StatechartInput>,
    pub name: Option<String>,
    pub regions: Vec<StatechartRegion>,
}
impl PartialEq for Statechart {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Statechart.ts:78 (sha256:22ea4259be50f9379bbc87e7bd14d2a0f15c6f210c27a933204561b345be02e4)
pub type StatechartDurationGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(StatechartInstance, StatechartTransitionExplanation) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/Statechart.ts:89 (sha256:99282de81a9db1f080a9e203455b3ae137163ab87e2b608b70d3defc5949fa86)
#[derive(Clone, Default)]
pub struct StatechartInstance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub chart: Statechart,
    pub duration_guard: Option<StatechartDurationGuard>,
    pub input_values: Vec<f64>,
    pub region_blend: Vec<f32>,
    pub region_duration: Vec<f64>,
    pub region_elapsed: Vec<f64>,
    pub region_states: Vec<i32>,
    pub region_transitions: Vec<i32>,
    pub signals: Option<StatechartSignals>,
}
impl PartialEq for StatechartInstance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Statechart.ts:103 (sha256:02afe6825272c5462f70153336cf4a25615ddb76ba4a19c227392cfa634527b7)
#[derive(Clone, Default)]
pub struct StatechartTransitionStatusValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub conditions_unmet: String,
    pub exit_time_pending: String,
    pub invalid_region: String,
    pub missing_region_duration: String,
    pub no_transitions: String,
    pub ready: String,
    pub transitioning: String,
}
impl PartialEq for StatechartTransitionStatusValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static STATECHART_TRANSITION_STATUS: std::sync::LazyLock<StatechartTransitionStatusValues> =
    std::sync::LazyLock::new(|| StatechartTransitionStatusValues {
        __flight_identity: std::sync::Arc::new(()),
        conditions_unmet: "ConditionsUnmet".to_owned(),
        exit_time_pending: "ExitTimePending".to_owned(),
        invalid_region: "InvalidRegion".to_owned(),
        missing_region_duration: "MissingRegionDuration".to_owned(),
        no_transitions: "NoTransitions".to_owned(),
        ready: "Ready".to_owned(),
        transitioning: "Transitioning".to_owned(),
    });

// Source: upstream/packages/types/src/Statechart.ts:113 (sha256:92cb7742ea4bdc3b0e58fbcd9bff28680fef986fe5de7ebfa416f4929fba6be2)
pub type StatechartTransitionStatus = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Statechart.ts:115 (sha256:f7c1c8098f7f7c82fe01a7ddf2d805715c481cb04fccaf02da88b46b1d167363)
#[derive(Clone, Default)]
pub struct StatechartTransitionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blend: f64,
    pub condition_index: f64,
    pub region_index: f64,
    pub source_state_index: f64,
    pub status: StatechartTransitionStatus,
    pub target_state_index: f64,
    pub transition_index: f64,
}
impl PartialEq for StatechartTransitionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
