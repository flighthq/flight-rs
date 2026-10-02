// @generated from upstream/packages/signals/src/scope.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::disconnect_signal_connection;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, SignalScope};

// Source: upstream/packages/signals/src/scope.ts:6 (sha256:cfffa67d11bedb2402898237a46f68276275ae5a77c2b84e2d3838f59e871e18)
pub fn create_signal_scope() -> SignalScope {
    let mut out = allocate_entity();
    initialize_signal_scope((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/signals/src/scope.ts:16 (sha256:5d0945aad59fea9ee3401655f27c55f5c49e13d5f1304291f04a9d1766f3ab04)
pub fn disconnect_signal_scope(scope: &mut SignalScope) -> () {
    if ((scope.connections.len() as f64) == 0.0_f64) {
        return;
    }
    let mut pending = (scope.connections).clone();
    scope.connections.clear();
    {
        let mut i = 0.0_f64;
        while (i < (pending.len() as f64)) {
            disconnect_signal_connection(&mut pending[i as usize]);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/signals/src/scope.ts:28 (sha256:927cd4784c66ed8086e0947a68caee5ea0fc373cb7c56b51ff5ad59eeca8733c)
pub fn initialize_signal_scope(out: EntityConstruction<SignalScope>) -> () {
    crate::host_set("host.connections", vec![]);
}
