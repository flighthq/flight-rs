// @generated from upstream/packages/accessibility/src/accessibility.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    AccessibilityBackend, AccessibilityLiveness, AccessibilityNode, AccessibilityState,
};

// Source: upstream/packages/accessibility/src/accessibility.ts:11 (sha256:86e42e2a79c1c8e740386bf1331cd24006f9a92fc9515cc0314ee901995e7ce8)
pub fn announce_accessibility(message: String, liveness: Option<AccessibilityLiveness>) -> () {
    let liveness = liveness.unwrap_or("polite".to_owned());
    {
        let __flight_callback = (get_accessibility_backend().announce).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((message).clone(), (liveness).clone());
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:17 (sha256:44dfa7c3b19e520f5630dddb0849635b05ee563c5f8a57fc7f3359248cf4088c)
pub fn clear_accessibility_tree() -> () {
    {
        let __flight_callback = (get_accessibility_backend().clear).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:27 (sha256:fdee57bbd10d17303c4547c17846b416231df0be96a17ff8213c41a7c45de5fa)
pub fn create_web_accessibility_backend(
    container: Option<crate::OpaqueHostValue>,
) -> AccessibilityBackend {
    let elements: std::sync::Arc<std::sync::Mutex<Vec<(String, crate::OpaqueHostValue)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let live_regions: std::sync::Arc<
        std::sync::Mutex<Vec<(AccessibilityLiveness, crate::OpaqueHostValue)>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let root: std::sync::Arc<std::sync::Mutex<Option<crate::OpaqueHostValue>>> =
        std::sync::Arc::new(std::sync::Mutex::new(container));
    let root_resolved: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new((container).is_some()));
    let get_root: std::sync::Arc<
        std::sync::Mutex<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<dyn FnMut() -> Option<crate::OpaqueHostValue> + Send + 'static>,
                >,
            >,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let mut root = root.clone();
            let mut root_resolved = root_resolved.clone();
            move || -> Option<crate::OpaqueHostValue> {
                if (*root_resolved.lock().unwrap()).clone() {
                    return (*root.lock().unwrap()).clone();
                }
                (*root_resolved.lock().unwrap()) = true;
                {
                    (*root.lock().unwrap()) = None;
                    return None;
                }
            }
        })
            as Box<dyn FnMut() -> Option<crate::OpaqueHostValue> + Send + 'static>),
    )));
    return AccessibilityBackend {
        __flight_identity: std::sync::Arc::new(()),
        set_node: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut elements = elements.clone();
            let mut get_root = get_root.clone();
            move |node: AccessibilityNode| -> () {
                let mut overlay_root = {
                    let __flight_callback = (*get_root.lock().unwrap()).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if (overlay_root).is_none() {
                    return;
                }
                let mut element = (*elements.lock().unwrap())
                    .iter()
                    .find(|(key, _)| key == &(node.id).clone())
                    .map(|(_, value)| value.clone());
                if ((element).clone()).is_none() {
                    element = Some(crate::host_value::<crate::OpaqueHostValue>(
                        "host.createElement",
                    ));
                    crate::host_value::<()>("host.setAttribute");
                    {
                        let __flight_key = (node.id).clone();
                        let __flight_value = {
                            let __flight_portable_source = (element).clone();
                            match (&__flight_portable_source).as_ref() {
                                Some(value) => (value).clone(),
                                None => crate::FlightValue::Null,
                            }
                        };
                        if let Some((_, value)) = (*elements.lock().unwrap())
                            .iter_mut()
                            .find(|(key, _)| key == &__flight_key)
                        {
                            *value = __flight_value;
                        } else {
                            (*elements.lock().unwrap()).push((__flight_key, __flight_value));
                        }
                    };
                }
                _apply_accessibility_element_attributes(
                    ({
                        let __flight_portable_source = (element).clone();
                        match (&__flight_portable_source).as_ref() {
                            Some(value) => (value).clone(),
                            None => crate::FlightValue::Null,
                        }
                    })
                    .clone(),
                    &node,
                );
                _reparent_accessibility_element(
                    ({
                        let __flight_portable_source = (element).clone();
                        match (&__flight_portable_source).as_ref() {
                            Some(value) => (value).clone(),
                            None => crate::FlightValue::Null,
                        }
                    })
                    .clone(),
                    ((node.parent_id).clone()).clone(),
                    &(*elements.lock().unwrap()),
                    (overlay_root.as_mut().unwrap()).clone(),
                );
            }
        })
            as Box<dyn FnMut(AccessibilityNode) -> () + Send + 'static>)),
        remove_node: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut elements = elements.clone();
            move |id: String| -> () {
                let element = (*elements.lock().unwrap())
                    .iter()
                    .find(|(key, _)| key == &(id).clone())
                    .map(|(_, value)| value.clone());
                if (element).is_none() {
                    return;
                }
                for __iteration0 in ((*elements.lock().unwrap()).clone()).iter().cloned() {
                    let key = __iteration0[0.0_f64 as usize].clone();
                    let other = __iteration0[1.0_f64 as usize].clone();
                    if match &(crate::host_value::<()>("host.contains")) {
                        crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
                        crate::OpaqueHostValue::Bool(value) => *value,
                        crate::OpaqueHostValue::Number(value) => {
                            *value != 0.0_f64 && !value.is_nan()
                        }
                        crate::OpaqueHostValue::String(value) => !value.is_empty(),
                        crate::OpaqueHostValue::Array(_)
                        | crate::OpaqueHostValue::Record(_)
                        | crate::OpaqueHostValue::Function
                        | crate::OpaqueHostValue::Symbol
                        | crate::OpaqueHostValue::Object => true,
                    } {
                        {
                            let __flight_key = key;
                            if let Some(__flight_index) = (*elements.lock().unwrap())
                                .iter()
                                .position(|(key, _)| key == &__flight_key)
                            {
                                (*elements.lock().unwrap()).remove(__flight_index);
                                true
                            } else {
                                false
                            }
                        };
                    }
                }
                if (crate::host_value::<Option<crate::OpaqueHostValue>>("host.parentNode"))
                    .is_some()
                {
                    crate::host_value::<()>("host.removeChild");
                }
            }
        })
            as Box<dyn FnMut(String) -> () + Send + 'static>)),
        clear: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut elements = elements.clone();
            let mut get_root = get_root.clone();
            let mut live_regions = live_regions.clone();
            move || -> () {
                let overlay_root = {
                    let __flight_callback = (*get_root.lock().unwrap()).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                (*elements.lock().unwrap()).clear();
                (*live_regions.lock().unwrap()).clear();
                if (overlay_root).is_some() {
                    crate::host_value::<()>("host.replaceChildren");
                }
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        set_focus: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut elements = elements.clone();
            let mut get_root = get_root.clone();
            move |id: String| -> bool {
                let overlay_root = {
                    let __flight_callback = (*get_root.lock().unwrap()).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if (overlay_root).is_none() {
                    return false;
                }
                let element = (*elements.lock().unwrap())
                    .iter()
                    .find(|(key, _)| key == &(id).clone())
                    .map(|(_, value)| value.clone());
                if (element).is_none() {
                    return false;
                }
                crate::host_value::<()>("host.focus");
                return (crate::host_value::<crate::OpaqueHostValue>("host.activeElement")
                    == element.as_ref().unwrap());
            }
        })
            as Box<dyn FnMut(String) -> bool + Send + 'static>)),
        announce: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut get_root = get_root.clone();
            let mut live_regions = live_regions.clone();
            move |message: String, liveness: AccessibilityLiveness| -> () {
                let overlay_root = {
                    let __flight_callback = (*get_root.lock().unwrap()).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                if (overlay_root).is_none() {
                    return;
                }
                let mut region = _get_accessibility_live_region(
                    (overlay_root.as_ref().unwrap()).clone(),
                    &mut (*live_regions.lock().unwrap()),
                    (liveness).clone(),
                );
                crate::host_set("host.textContent", message);
            }
        })
            as Box<dyn FnMut(String, AccessibilityLiveness) -> () + Send + 'static>)),
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:95 (sha256:76399fa590f93877bb742393f95ca5bae5abe50aa48ae6627e4799d50914807a)
pub fn get_accessibility_backend() -> AccessibilityBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_accessibility_backend(None));
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/accessibility/src/accessibility.ts:101 (sha256:280784fc9eb942ee9607ea7b7f52ce5f4a22a980ff6afaf16ef800a210883561)
pub fn remove_accessibility_node(id: String) -> () {
    {
        let __flight_callback = (get_accessibility_backend().remove_node).clone();
        let __flight_result = __flight_callback.lock().unwrap()((id).clone());
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:106 (sha256:9c2150404fdc1c0ef9a73753512fa05d78e4350d378787b58966aca5000b2ae9)
pub fn set_accessibility_backend(backend: Option<AccessibilityBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/accessibility/src/accessibility.ts:112 (sha256:afed3a491be320f8ed387e3a88795157e8e894d32a43d830ab42c090ae81b002)
pub fn set_accessibility_focus(id: String) -> bool {
    return {
        let __flight_callback = (get_accessibility_backend().set_focus).clone();
        let __flight_result = __flight_callback.lock().unwrap()((id).clone());
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:118 (sha256:424a6b2312331c87a45df318680bf93a84c1fe629ed84bbe08df46097dbb94d4)
pub fn set_accessibility_node(node: &AccessibilityNode) -> () {
    {
        let __flight_callback = (get_accessibility_backend().set_node).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*node).clone());
        __flight_result
    };
}

// Source: upstream/packages/accessibility/src/accessibility.ts:122 (sha256:fa0b551f65e8a4cd7abf102113b8de32b26634cdd65f5bbbb86990a62ac10bc7)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<AccessibilityBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/accessibility/src/accessibility.ts:124 (sha256:59075ddb3b7e32f4b8d12399ebad0bb99b87857d5ea5d33ce599737e024b4ab4)
const _TEXT_NODE: f64 = 3.0_f64;

// Source: upstream/packages/accessibility/src/accessibility.ts:129 (sha256:fbc8f134dba3cb12dc6c8e5b871c17ada805a89b8c1c6715ecb517508fec9e87)
fn _apply_accessibility_element_attributes(
    element: crate::OpaqueHostValue,
    node: &AccessibilityNode,
) -> () {
    crate::host_value::<()>("host.setAttribute");
    _reflect_accessibility_attribute(
        (element).clone(),
        "aria-label".to_owned(),
        ((node.label).clone()).clone(),
    );
    _reflect_accessibility_attribute(
        (element).clone(),
        "aria-description".to_owned(),
        ((node.description).clone()).clone(),
    );
    _reflect_accessibility_attribute(
        (element).clone(),
        "title".to_owned(),
        ((node.description).clone()).clone(),
    );
    _reflect_accessibility_attribute(
        (element).clone(),
        "aria-valuetext".to_owned(),
        ((node.value).clone()).clone(),
    );
    _set_accessibility_element_value_text((element).clone(), ((node.value).clone()).clone());
    crate::host_value::<()>("host.setAttribute");
    _apply_accessibility_state_attributes(
        (element).clone(),
        &((node.states).clone()).unwrap_or(((*_EMPTY_STATE).clone()).clone()),
    );
}

// Source: upstream/packages/accessibility/src/accessibility.ts:144 (sha256:bafb5868a53c147572f81419836e5cea16ddd68e867aea2c77ab4ea6e867a35b)
fn _apply_accessibility_state_attributes(
    element: crate::OpaqueHostValue,
    state: &AccessibilityState,
) -> () {
    _reflect_accessibility_boolean(
        (element).clone(),
        "aria-disabled".to_owned(),
        state.disabled,
    );
    _reflect_accessibility_boolean((element).clone(), "aria-checked".to_owned(), state.checked);
    _reflect_accessibility_boolean(
        (element).clone(),
        "aria-expanded".to_owned(),
        state.expanded,
    );
    _reflect_accessibility_boolean(
        (element).clone(),
        "aria-selected".to_owned(),
        state.selected,
    );
    _reflect_accessibility_boolean((element).clone(), "aria-pressed".to_owned(), state.pressed);
    _reflect_accessibility_boolean((element).clone(), "aria-busy".to_owned(), state.busy);
    _reflect_accessibility_boolean((element).clone(), "aria-hidden".to_owned(), state.hidden);
    _reflect_accessibility_boolean(
        (element).clone(),
        "aria-readonly".to_owned(),
        state.readonly,
    );
    _reflect_accessibility_boolean(
        (element).clone(),
        "aria-required".to_owned(),
        state.required,
    );
    _reflect_accessibility_number((element).clone(), "aria-level".to_owned(), state.level);
    _reflect_accessibility_number(
        (element).clone(),
        "aria-valuemin".to_owned(),
        state.value_min,
    );
    _reflect_accessibility_number(
        (element).clone(),
        "aria-valuemax".to_owned(),
        state.value_max,
    );
    _reflect_accessibility_number(
        (element).clone(),
        "aria-valuenow".to_owned(),
        state.value_now,
    );
}

// Source: upstream/packages/accessibility/src/accessibility.ts:162 (sha256:765a32dcd37221032140101e553ac26c55c35ae2d191a71fee99003d70ec8b02)
fn _create_hidden_accessibility_container(doc: crate::OpaqueHostValue) -> crate::OpaqueHostValue {
    let mut container = crate::host_value::<()>("host.createElement");
    crate::host_value::<()>("host.setAttribute");
    let mut style = crate::host_value::<crate::OpaqueHostValue>("host.style");
    crate::host_set("host.position", "absolute");
    crate::host_set("host.width", "1px");
    crate::host_set("host.height", "1px");
    crate::host_set("host.margin", "-1px");
    crate::host_set("host.padding", "0");
    crate::host_set("host.border", "0");
    crate::host_set("host.overflow", "hidden");
    crate::host_set("host.clip", "rect(0 0 0 0)");
    crate::host_set("host.clipPath", "inset(50%)");
    crate::host_set("host.whiteSpace", "nowrap");
    return container;
}

// Source: upstream/packages/accessibility/src/accessibility.ts:181 (sha256:fc9affb9cadf5ee9f967f2d2ce8da4476278010e33d869b6922e2e65b3460d28)
fn _get_accessibility_live_region(
    root: crate::OpaqueHostValue,
    live_regions: &mut Vec<(AccessibilityLiveness, crate::OpaqueHostValue)>,
    liveness: AccessibilityLiveness,
) -> crate::OpaqueHostValue {
    let mut region = live_regions
        .iter()
        .find(|(key, _)| key == &(liveness).clone())
        .map(|(_, value)| value.clone());
    if (((region).clone()).is_none())
        || ((crate::host_value::<Option<crate::OpaqueHostValue>>("host.parentNode")).is_none())
    {
        region = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createElement",
        ));
        crate::host_value::<()>("host.setAttribute");
        crate::host_value::<()>("host.setAttribute");
        crate::host_value::<()>("host.setAttribute");
        {
            let __flight_key = (liveness).clone();
            let __flight_value = {
                let __flight_portable_source = (region).clone();
                match (&__flight_portable_source).as_ref() {
                    Some(value) => (value).clone(),
                    None => crate::FlightValue::Null,
                }
            };
            if let Some((_, value)) = live_regions
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                live_regions.push((__flight_key, __flight_value));
            }
        };
        crate::host_value::<()>("host.appendChild");
    }
    return ({
        let __flight_portable_source = (region).clone();
        match (&__flight_portable_source).as_ref() {
            Some(value) => (value).clone(),
            None => crate::FlightValue::Null,
        }
    })
    .clone();
}

// Source: upstream/packages/accessibility/src/accessibility.ts:199 (sha256:480632ebc019a08ca06de4543ce0b6940dd580b5ab7a289fe465adda12203b8d)
fn _reflect_accessibility_attribute(
    element: crate::OpaqueHostValue,
    attribute: String,
    value: Option<String>,
) -> () {
    if (value).is_none() {
        crate::host_value::<()>("host.removeAttribute");
        return;
    }
    crate::host_value::<()>("host.setAttribute");
}

// Source: upstream/packages/accessibility/src/accessibility.ts:208 (sha256:3de594c07b56e809d28aa9fd1d7bf0312fa585d18f81bdfdf7e11434de4b68e5)
fn _reflect_accessibility_boolean(
    element: crate::OpaqueHostValue,
    attribute: String,
    value: Option<bool>,
) -> () {
    if (value).is_none() {
        crate::host_value::<()>("host.removeAttribute");
        return;
    }
    crate::host_value::<()>("host.setAttribute");
}

// Source: upstream/packages/accessibility/src/accessibility.ts:217 (sha256:d36900314b287524fbe0755af7f063ea45bfcf00313df0c64722739eb959f2a5)
fn _reflect_accessibility_number(
    element: crate::OpaqueHostValue,
    attribute: String,
    value: Option<f64>,
) -> () {
    if (value).is_none() {
        crate::host_value::<()>("host.removeAttribute");
        return;
    }
    crate::host_value::<()>("host.setAttribute");
}

// Source: upstream/packages/accessibility/src/accessibility.ts:228 (sha256:8ac0ae5ccc27e54397c7aca0d0bed62324dfcca8ddabde5a5afde5deeac17d7f)
fn _reparent_accessibility_element(
    element: crate::OpaqueHostValue,
    parent_id: Option<String>,
    elements: &Vec<(String, crate::OpaqueHostValue)>,
    root: crate::OpaqueHostValue,
) -> () {
    let mut parent = (root).clone();
    if (parent_id).is_some() {
        let found = elements
            .iter()
            .find(|(key, _)| key == &(parent_id.as_ref().unwrap()).clone())
            .map(|(_, value)| value.clone());
        if (found).is_some() {
            parent = (found.as_ref().unwrap()).clone();
        }
    }
    if (crate::host_value::<crate::OpaqueHostValue>("host.parentNode") != parent) {
        crate::host_value::<()>("host.appendChild");
    }
}

// Source: upstream/packages/accessibility/src/accessibility.ts:244 (sha256:7c11e4e703dc2f9f3ffc51512714e72d6ca96c8a6765de3114752c7304c193e0)
fn _set_accessibility_element_value_text(
    element: crate::OpaqueHostValue,
    value: Option<String>,
) -> () {
    let mut first = crate::host_value::<crate::OpaqueHostValue>("host.firstChild");
    if (value).is_none() {
        if (((first).clone()).is_some())
            && (crate::host_value::<f64>("host.nodeType") == _TEXT_NODE)
        {
            crate::host_value::<()>("host.removeChild");
        }
        return;
    }
    if ((first).is_some()) && (crate::host_value::<f64>("host.nodeType") == _TEXT_NODE) {
        crate::host_set("host.nodeValue", (value.as_ref().unwrap()).clone());
        return;
    }
    crate::host_value::<()>("host.insertBefore");
}

// Source: upstream/packages/accessibility/src/accessibility.ts:257 (sha256:42c20547d810f5ee2eb4487b6ed8f4a1980b58f105f431b35fc5486e6a4d652b)
static _EMPTY_STATE: std::sync::LazyLock<AccessibilityState> =
    std::sync::LazyLock::new(|| AccessibilityState {
        __flight_identity: std::sync::Arc::new(()),
        disabled: None,
        checked: None,
        expanded: None,
        selected: None,
        pressed: None,
        busy: None,
        hidden: None,
        readonly: None,
        required: None,
        level: None,
        value_min: None,
        value_max: None,
        value_now: None,
    });
