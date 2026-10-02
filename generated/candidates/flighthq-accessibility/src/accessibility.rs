// @generated from upstream/packages/accessibility/src/accessibility.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    AccessibilityLiveness, AccessibilityNode, AccessibilityOperationOutcome,
    HostAccessibilityCapability,
};

// Source: upstream/packages/accessibility/src/accessibility.ts:9 (sha256:39f70137cb2dcbb3a96cb7fee9ce839cde96b641b207247386b4698bf013565f)
pub fn announce_accessibility(
    host_accessibility: &HostAccessibilityCapability,
    message: String,
    liveness: Option<AccessibilityLiveness>,
) -> AccessibilityOperationOutcome<String> {
    let liveness = liveness.unwrap_or("polite".to_owned());
    return {
        let __flight_callback = (host_accessibility.announce).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((message).clone(), (liveness).clone());
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:18 (sha256:e1dbb74b988f8e00e164642b8011863ee8a92612a0bdbddc0dcbc5e205c92212)
pub fn clear_accessibility_tree(
    host_accessibility: &mut HostAccessibilityCapability,
) -> AccessibilityOperationOutcome<String> {
    return {
        let __flight_callback = (host_accessibility.clear).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:26 (sha256:670c5ef1696037232a86af8173e7b644b979eb55476c099c9504cdd685b46e0d)
pub fn destroy_accessibility(host_accessibility: &HostAccessibilityCapability) -> () {
    {
        let __flight_callback = (host_accessibility.destroy).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:31 (sha256:afe2e01a0756e923d1b4623802da8e1cda6f5d18e562925d3c0a6793a1d0444a)
pub fn remove_accessibility_node(
    host_accessibility: &HostAccessibilityCapability,
    id: String,
) -> AccessibilityOperationOutcome<String> {
    return {
        let __flight_callback = (host_accessibility.remove_node).clone();
        let __flight_result = __flight_callback.lock().unwrap()((id).clone());
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:39 (sha256:7510280473891d351ddd99a094211d0ba7d1b96c54c3c2f585d21e0d80ea57e3)
pub fn set_accessibility_focus(
    host_accessibility: &HostAccessibilityCapability,
    id: String,
) -> AccessibilityOperationOutcome<String> {
    return {
        let __flight_callback = (host_accessibility.set_focus).clone();
        let __flight_result = __flight_callback.lock().unwrap()((id).clone());
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:47 (sha256:726d4e44591938c0580f49ec92e827d098de709e8368e3b45994c31b398a015a)
pub fn set_accessibility_node(
    host_accessibility: &HostAccessibilityCapability,
    node: &AccessibilityNode,
) -> AccessibilityOperationOutcome<String> {
    return {
        let __flight_callback = (host_accessibility.set_node).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*node).clone());
        __flight_result
    };
}
