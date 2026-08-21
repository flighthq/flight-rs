// @generated from upstream/packages/types/src/RegistryTable.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

#[derive(Clone, Default)]
pub struct RegistryTableEntryRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub state: String,
}
impl PartialEq for RegistryTableEntryRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct RegistryTableEntryRecord2<T> {
    pub __flight_identity: std::sync::Arc<()>,
    pub state: String,
    pub value: T,
}
impl<T> PartialEq for RegistryTableEntryRecord2<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RegistryTable.ts:5 (sha256:afec52b2cf6bdb293adac880cbcbeb72af88397062e1443ab14d56170def7dd5)
pub type RegistryId = String;

// Source: upstream/packages/types/src/RegistryTable.ts:19 (sha256:d5b90ddb30ab0884f16eb15de021929e5c837aee59e1ba8f0ba57a6220806dd9)
pub type RegistryTableEntry<T> =
    crate::FlightUnion2<RegistryTableEntryRecord2<T>, RegistryTableEntryRecord1>;

// Source: upstream/packages/types/src/RegistryTable.ts:27 (sha256:2de2100ef8f8ec3ddee7524960494a73dec90beb10a6e543dce9c52d4303770e)
#[derive(Clone, Default)]
pub struct RegistryEntryStateValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bound: String,
    pub tombstoned: String,
}
impl PartialEq for RegistryEntryStateValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static REGISTRY_ENTRY_STATE: std::sync::LazyLock<RegistryEntryStateValues> =
    std::sync::LazyLock::new(|| RegistryEntryStateValues {
        __flight_identity: std::sync::Arc::new(()),
        bound: "bound".to_owned(),
        tombstoned: "tombstoned".to_owned(),
    });

// Source: upstream/packages/types/src/RegistryTable.ts:32 (sha256:2cd7cee14b4cbf703ee178a001ee11a29bf8154c5a4892effed28684eedd5bed)
pub type RegistryEntryState = String;

// Source: upstream/packages/types/src/RegistryTable.ts:42 (sha256:6c8d1b583187c8c8c4320ab1a83acb965cdd69230acfbb877dde632f333a5d72)
pub type RegistryMissPolicy = String;

// Source: upstream/packages/types/src/RegistryTable.ts:45 (sha256:93813df815fcdb2dd0cd95f6befed1f2a3cdcc3d88e69f07b6b47aa32d483510)
#[derive(Clone, Default)]
pub struct RegistryTableBase {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_miss: RegistryMissPolicy,
    pub registry: RegistryId,
}
impl PartialEq for RegistryTableBase {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RegistryTable.ts:61 (sha256:c0d2ba022672db2da6420bc33fb8b47d36dd6f077396e3199259ca96250b10c1)
#[derive(Clone)]
pub struct KeyedTable<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_miss: RegistryMissPolicy,
    pub registry: RegistryId,
    pub entries: Vec<(Kind, RegistryTableEntry<T>)>,
    pub shape: String,
}
impl<T> Default for KeyedTable<T> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            on_miss: Default::default(),
            registry: Default::default(),
            entries: Default::default(),
            shape: Default::default(),
        }
    }
}
impl<T> PartialEq for KeyedTable<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RegistryTable.ts:69 (sha256:c359ac4db9d9ca0539465d8e5184c494387ed640d5277aa6ff1a043231bf3615)
#[derive(Clone)]
pub struct SlotTable<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_miss: RegistryMissPolicy,
    pub registry: RegistryId,
    pub entry: Option<RegistryTableEntry<T>>,
    pub shape: String,
}
impl<T> Default for SlotTable<T> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            on_miss: Default::default(),
            registry: Default::default(),
            entry: Default::default(),
            shape: Default::default(),
        }
    }
}
impl<T> PartialEq for SlotTable<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RegistryTable.ts:85 (sha256:f6905dd4e9c9b6b04daecde2f859e908b2a17f615356b0e603fac80285857707)
#[derive(Clone)]
pub struct OrdinalTable<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_miss: RegistryMissPolicy,
    pub registry: RegistryId,
    pub entries: Vec<Option<T>>,
    pub shape: String,
    pub vocabulary: Vec<Kind>,
}
impl<T> Default for OrdinalTable<T> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            on_miss: Default::default(),
            registry: Default::default(),
            entries: Default::default(),
            shape: Default::default(),
            vocabulary: Default::default(),
        }
    }
}
impl<T> PartialEq for OrdinalTable<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RegistryTable.ts:94 (sha256:bd5b6f727730ec95df77813946eca4e729f49e0dab3daa1e433c85a36f7831fd)
pub type RegistryTable<T> =
    crate::FlightUnion2<KeyedTable<T>, crate::FlightUnion2<OrdinalTable<T>, SlotTable<T>>>;
