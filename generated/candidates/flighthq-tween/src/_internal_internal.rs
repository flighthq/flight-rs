// @generated from upstream/packages/tween/src/internal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Tween;

// Source: upstream/packages/tween/src/internal.ts:3 (sha256:293162664fe26ff50eecedb84de09bed39e388d682425a94b6533d2640022bee)
pub fn initialize_tween<T: Clone>(tween: &mut Tween<T>) -> () {
    let target = (tween.target).clone();
    let property_map = crate::host_value::<Vec<(String, f64)>>("host.cast");
    for detail in ((tween.properties).clone()).iter().cloned() {
        let start = target
            .iter()
            .find(|(key, _)| key == &(detail.key).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        let end = property_map
            .iter()
            .find(|(key, _)| key == &(detail.key).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        detail.start = start;
        detail.change = (end - start);
        if tween.smart_rotation {
            let mut change = (((detail.change % 360.0_f64) + 360.0_f64) % 360.0_f64);
            if (change > 180.0_f64) {
                change -= 360.0_f64;
            }
            detail.change = change;
        }
    }
    tween.initialized = true;
}
