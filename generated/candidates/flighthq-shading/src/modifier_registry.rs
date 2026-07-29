// @generated from upstream/packages/shading/src/modifierRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Modifier, ModifierKind, ModifierSlot};

// Source: upstream/packages/shading/src/modifierRegistry.ts:14 (sha256:bc5a0db9693884c4b2159347b87eabaa0ba4e2330a93083c4aaaa4c837b4ddc6)
#[derive(Clone, Default)]
pub struct ModifierDefinition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub get_define_signature: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier) -> String + Send + 'static>>>,
    >,
}
impl PartialEq for ModifierDefinition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/modifierRegistry.ts:25 (sha256:778ef63f44eba1c663f0dde187afd6c28a7a394c91ab6100af02d72a7819c8fb)
#[derive(Clone, Default)]
pub struct ModifierRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub definitions: Vec<(ModifierKind, ModifierDefinition)>,
}
impl PartialEq for ModifierRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/shading/src/modifierRegistry.ts:31 (sha256:0f74d3677f807ea5e6bf56bf1e3f06f544c0a07f77acab117e2e5d4712815e35)
#[derive(Clone, Default)]
struct CreateModifierRegistryRecord1 {
    __flight_identity: std::sync::Arc<()>,
    definitions: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
}
impl PartialEq for CreateModifierRegistryRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_modifier_registry() -> ModifierRegistry {
    return ModifierRegistry {
        __flight_identity: std::sync::Arc::new(()),
        definitions: Vec::new(),
    };
}

// Source: upstream/packages/shading/src/modifierRegistry.ts:39 (sha256:9f336eacf8ae25dd546ca74573a8cf3e0b5b96e5d5748b5bde6b8031420415b5)
pub fn register_modifier(registry: &mut ModifierRegistry, definition: &ModifierDefinition) -> () {
    {
        let __flight_key = (definition.kind).clone();
        let __flight_value = (*definition).clone();
        if let Some((_, value)) = registry
            .definitions
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            registry.definitions.push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/shading/src/modifierRegistry.ts:45 (sha256:2bf9a9334ccdfc5f8e23af861004af8ffeb723002d45be4bb6278ab5e7b32f71)
pub fn resolve_modifier(
    registry: &ModifierRegistry,
    kind: ModifierKind,
) -> Option<ModifierDefinition> {
    return registry
        .definitions
        .iter()
        .find(|(key, _)| key == &(kind).clone())
        .map(|(_, value)| value.clone());
}
