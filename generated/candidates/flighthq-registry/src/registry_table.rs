// @generated from upstream/packages/registry/src/registryTable.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    KeyedTable, Kind, OrdinalTable, REGISTRY_ENTRY_STATE as registry_entry_state_constant,
    RegistryId, RegistryMissPolicy, RegistryTable, RegistryTableEntry, SlotTable,
};

// Source: upstream/packages/registry/src/registryTable.ts:32 (sha256:1be087dc21a2fa6abf8ea109c51f4f4383abdf3f2d1596c587c69347c508c05b)
pub fn concat_registry_table<T: Clone>(
    base: &crate::FlightUnion2<KeyedTable<T>, SlotTable<T>>,
    overlay: &crate::FlightUnion2<KeyedTable<T>, SlotTable<T>>,
) -> crate::FlightUnion2<KeyedTable<T>, SlotTable<T>> {
    if (match &((*base).clone()) {
        crate::FlightUnion2::A(value) => (value).shape.clone(),
        crate::FlightUnion2::B(value) => (value).shape.clone(),
    } != match &((*overlay).clone()) {
        crate::FlightUnion2::A(value) => (value).shape.clone(),
        crate::FlightUnion2::B(value) => (value).shape.clone(),
    }) {
        panic!(
            "{}",
            format!(
                "concatRegistryTable: cannot compose a '{}' table with a '{}' table",
                match &((*base).clone()) {
                    crate::FlightUnion2::A(value) => (value).shape.clone(),
                    crate::FlightUnion2::B(value) => (value).shape.clone(),
                },
                match &((*overlay).clone()) {
                    crate::FlightUnion2::A(value) => (value).shape.clone(),
                    crate::FlightUnion2::B(value) => (value).shape.clone(),
                }
            )
        );
    }
    if (match &((*base).clone()) {
        crate::FlightUnion2::A(value) => (value).registry.clone(),
        crate::FlightUnion2::B(value) => (value).registry.clone(),
    } != match &((*overlay).clone()) {
        crate::FlightUnion2::A(value) => (value).registry.clone(),
        crate::FlightUnion2::B(value) => (value).registry.clone(),
    }) {
        panic!(
            "{}",
            format!(
                "concatRegistryTable: cannot compose registry '{}' with registry '{}'",
                match &((*base).clone()) {
                    crate::FlightUnion2::A(value) => (value).registry.clone(),
                    crate::FlightUnion2::B(value) => (value).registry.clone(),
                },
                match &((*overlay).clone()) {
                    crate::FlightUnion2::A(value) => (value).registry.clone(),
                    crate::FlightUnion2::B(value) => (value).registry.clone(),
                }
            )
        );
    }
    if (match &((*base).clone()) {
        crate::FlightUnion2::A(value) => (value).on_miss.clone(),
        crate::FlightUnion2::B(value) => (value).on_miss.clone(),
    } != match &((*overlay).clone()) {
        crate::FlightUnion2::A(value) => (value).on_miss.clone(),
        crate::FlightUnion2::B(value) => (value).on_miss.clone(),
    }) {
        panic!(
            "{}",
            format!(
                "concatRegistryTable: cannot compose miss policy '{}' with miss policy '{}'",
                match &((*base).clone()) {
                    crate::FlightUnion2::A(value) => (value).on_miss.clone(),
                    crate::FlightUnion2::B(value) => (value).on_miss.clone(),
                },
                match &((*overlay).clone()) {
                    crate::FlightUnion2::A(value) => (value).on_miss.clone(),
                    crate::FlightUnion2::B(value) => (value).on_miss.clone(),
                }
            )
        );
    }
    if matches!(&(base), crate::FlightUnion2::B(_)) {
        let overlay_slot = match (*overlay).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        };
        return crate::FlightUnion2::<KeyedTable<T>, SlotTable<T>>::B(SlotTable::<T> {
            __flight_identity: std::sync::Arc::new(()),
            entry: ((overlay_slot.entry).clone())
                .clone()
                .or(((match (*base).clone() {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                })
                .entry)
                    .clone()),
            on_miss: ((match (*base).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            })
            .on_miss)
                .clone(),
            registry: ((match (*base).clone() {
                crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                crate::FlightUnion2::B(value) => value,
            })
            .registry)
                .clone(),
            shape: "slot".to_owned(),
        });
    }
    let base_keyed = match (*base).clone() {
        crate::FlightUnion2::A(value) => value,
        crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
    };
    let overlay_keyed = match (*overlay).clone() {
        crate::FlightUnion2::A(value) => value,
        crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
    };
    let mut entries: Vec<(Kind, RegistryTableEntry<T>)> = (base_keyed.entries).clone();
    for __iteration0 in ((overlay_keyed.entries).clone()).iter().cloned() {
        let key = __iteration0.0.clone();
        let entry = __iteration0.1.clone();
        {
            let __switch_value = match &((entry).clone()) {
                crate::FlightUnion2::A(value) => (value).state.clone(),
                crate::FlightUnion2::B(value) => (value).state.clone(),
            };
            let __flight_case = if __switch_value == registry_entry_state_constant.bound {
                0_usize
            } else if __switch_value == registry_entry_state_constant.tombstoned {
                1_usize
            } else {
                2_usize
            };
            '__flight_switch: {
                if __flight_case <= 0_usize {
                    {
                        let __flight_key = (key).clone();
                        let __flight_value = (entry).clone();
                        if let Some((_, value)) =
                            entries.iter_mut().find(|(key, _)| key == &__flight_key)
                        {
                            *value = __flight_value;
                        } else {
                            entries.push((__flight_key, __flight_value));
                        }
                    };
                    break '__flight_switch;
                }
                if __flight_case <= 1_usize {
                    {
                        let __flight_key = (key).clone();
                        let __flight_value = (entry).clone();
                        if let Some((_, value)) =
                            entries.iter_mut().find(|(key, _)| key == &__flight_key)
                        {
                            *value = __flight_value;
                        } else {
                            entries.push((__flight_key, __flight_value));
                        }
                    };
                    break '__flight_switch;
                }
                if __flight_case <= 2_usize {
                    {
                        return panic!("TypeScript never value was reached");
                    }
                }
            }
        }
    }
    return crate::FlightUnion2::<KeyedTable<T>, SlotTable<T>>::A(KeyedTable::<T> {
        __flight_identity: std::sync::Arc::new(()),
        entries: (entries).clone(),
        on_miss: (base_keyed.on_miss).clone(),
        registry: (base_keyed.registry).clone(),
        shape: "keyed".to_owned(),
    });
}

// Source: upstream/packages/registry/src/registryTable.ts:80 (sha256:c4a89ae46771f55a113cb8049e9a199472d58c66129782e8ca8d1627b452b7b8)
pub fn create_keyed_table<T: Clone>(
    registry: RegistryId,
    on_miss: RegistryMissPolicy,
) -> KeyedTable<T> {
    return KeyedTable::<T> {
        __flight_identity: std::sync::Arc::new(()),
        entries: Vec::new(),
        on_miss: (on_miss).clone(),
        registry: (registry).clone(),
        shape: "keyed".to_owned(),
    };
}

// Source: upstream/packages/registry/src/registryTable.ts:85 (sha256:9b098a060d3fea89a004e918f87229113f80717b2e05ddc3ad6646e82552ea49)
pub fn create_ordinal_table<T: Clone>(
    registry: RegistryId,
    on_miss: RegistryMissPolicy,
    vocabulary: &Vec<Kind>,
) -> OrdinalTable<T> {
    return OrdinalTable::<T> {
        __flight_identity: std::sync::Arc::new(()),
        entries: (vocabulary)
            .iter()
            .cloned()
            .map(|__flight_unused_0: Kind| -> Option<T> { None })
            .collect::<Vec<_>>(),
        on_miss: (on_miss).clone(),
        registry: (registry).clone(),
        shape: "ordinal".to_owned(),
        vocabulary: (*vocabulary).clone(),
    };
}

// Source: upstream/packages/registry/src/registryTable.ts:94 (sha256:c402a8c4575bd4076d5b4fbf6fd7c347e8c1193034662fae8ff30217de1b681a)
pub fn create_slot_table<T: Clone>(
    registry: RegistryId,
    on_miss: RegistryMissPolicy,
) -> SlotTable<T> {
    return SlotTable::<T> {
        __flight_identity: std::sync::Arc::new(()),
        entry: None,
        on_miss: (on_miss).clone(),
        registry: (registry).clone(),
        shape: "slot".to_owned(),
    };
}

// Source: upstream/packages/registry/src/registryTable.ts:101 (sha256:2a0f4ba64d2c527a05f90cdedbe66a15dd41e78226d6993deb091513431632a4)
pub fn get_ordinal_table_entry<T: Clone>(table: &OrdinalTable<T>, ordinal: f64) -> Option<T> {
    if ((!(ordinal).is_finite() && (ordinal).fract() == 0.0_f64) || (ordinal < 0.0_f64))
        || (ordinal >= (table.entries.len() as f64))
    {
        return None;
    }
    return table.entries[ordinal as usize].clone();
}

// Source: upstream/packages/registry/src/registryTable.ts:111 (sha256:fc7f898ba4dc6dedd8efef8ffc1b2a46e7f151fe0ce9fc793e05e0f8e6b71de9)
pub fn get_registry_table_entry<T: Clone>(table: &RegistryTable<T>, key: Kind) -> Option<T> {
    let entry = get_registry_table_entry_state(table, (key).clone());
    if ((entry).is_none())
        || (match ((entry).clone())
            .as_ref()
            .expect("TypeScript nullable union property was not narrowed")
        {
            crate::FlightUnion2::A(value) => (value).state.clone(),
            crate::FlightUnion2::B(value) => (value).state.clone(),
        } != registry_entry_state_constant.bound)
    {
        return None;
    }
    return Some(
        ((match (entry.as_ref().unwrap()).clone() {
            flighthq_types::RegistryTableEntry::<T>::A(value) => value,
            flighthq_types::RegistryTableEntry::<T>::B(_) => {
                panic!("TypeScript union narrowing failed")
            }
        })
        .value)
            .clone(),
    );
}

// Source: upstream/packages/registry/src/registryTable.ts:120 (sha256:0e95d1aa15636c03bca865b201414cd7b0c3ebb898dda25178c3648afa154b90)
pub fn get_registry_table_keys(
    out: &mut Vec<Kind>,
    table: &RegistryTable<crate::FlightValue>,
) -> () {
    out.clear();
    if matches!(
        &(table),
        flighthq_types::RegistryTable::<crate::FlightValue>::A(_)
    ) {
        for __iteration1 in (((match (*table).clone() {
            flighthq_types::RegistryTable::<crate::FlightValue>::A(value) => value,
            flighthq_types::RegistryTable::<crate::FlightValue>::B(_) => {
                panic!("TypeScript union narrowing failed")
            }
        })
        .entries)
            .clone())
        .iter()
        .cloned()
        {
            let key = __iteration1.0.clone();
            let entry = __iteration1.1.clone();
            if matches!(
                &(entry),
                flighthq_types::RegistryTableEntry::<crate::FlightValue>::A(_)
            ) {
                out.push(((key).clone()).clone());
            }
        }
    } else {
        if matches!(&(table), crate::FlightUnion2::B(crate::FlightUnion2::B(_))) {
            if ((((match (*table).clone() {
                flighthq_types::RegistryTable::<crate::FlightValue>::A(_) => {
                    panic!("TypeScript union narrowing failed")
                }
                flighthq_types::RegistryTable::<crate::FlightValue>::B(value) => match value {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                },
            })
            .entry)
                .clone())
            .is_some())
                && (match (((match (*table).clone() {
                    flighthq_types::RegistryTable::<crate::FlightValue>::A(_) => {
                        panic!("TypeScript union narrowing failed")
                    }
                    flighthq_types::RegistryTable::<crate::FlightValue>::B(value) => match value {
                        crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                        crate::FlightUnion2::B(value) => value,
                    },
                })
                .entry)
                    .clone())
                .as_ref()
                .expect("TypeScript nullable union property was not narrowed")
                {
                    crate::FlightUnion2::A(value) => (value).state.clone(),
                    crate::FlightUnion2::B(value) => (value).state.clone(),
                } == registry_entry_state_constant.bound)
            {
                out.push(
                    ((match (*table).clone() {
                        flighthq_types::RegistryTable::<crate::FlightValue>::A(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                        flighthq_types::RegistryTable::<crate::FlightValue>::B(value) => {
                            match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => value,
                            }
                        }
                    })
                    .registry)
                        .clone(),
                );
            }
        } else {
            {
                let mut ordinal = 0.0_f64;
                while (ordinal
                    < ((match (*table).clone() {
                        flighthq_types::RegistryTable::<crate::FlightValue>::A(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                        flighthq_types::RegistryTable::<crate::FlightValue>::B(value) => {
                            match value {
                                crate::FlightUnion2::A(value) => value,
                                crate::FlightUnion2::B(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                            }
                        }
                    })
                    .entries
                    .len() as f64))
                {
                    if !((match (*table).clone() {
                        flighthq_types::RegistryTable::<crate::FlightValue>::A(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                        flighthq_types::RegistryTable::<crate::FlightValue>::B(value) => {
                            match value {
                                crate::FlightUnion2::A(value) => value,
                                crate::FlightUnion2::B(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                            }
                        }
                    })
                    .entries
                    .get((ordinal) as usize)
                    .is_none())
                    {
                        out.push(
                            (match (*table).clone() {
                                flighthq_types::RegistryTable::<crate::FlightValue>::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                flighthq_types::RegistryTable::<crate::FlightValue>::B(value) => {
                                    match value {
                                        crate::FlightUnion2::A(value) => value,
                                        crate::FlightUnion2::B(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                    }
                                }
                            })
                            .vocabulary[ordinal as usize]
                                .clone(),
                        );
                    }
                    {
                        ordinal += 1.0;
                        ordinal
                    };
                }
            }
        }
    }
    {
        let mut __flight_values = out;
        __flight_values
            .sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
        __flight_values
    };
}

// Source: upstream/packages/registry/src/registryTable.ts:138 (sha256:3997933e57915a6d58103c7a4e0159852c4bc206eb400ef6ca63d0027ccbfaa3)
pub fn has_registry_table_entry(table: &RegistryTable<crate::FlightValue>, key: Kind) -> bool {
    let entry = get_registry_table_entry_state(table, (key).clone());
    return ((entry).is_some())
        && (matches!(
            &(entry.as_ref().unwrap()),
            flighthq_types::RegistryTableEntry::<crate::FlightValue>::A(_)
        ));
}

// Source: upstream/packages/registry/src/registryTable.ts:149 (sha256:40de641abe6c4758444546ac3e58bb3fb9022aaa0db6000f23be8eab0aa0f176)
pub fn without_registry_table_entry<T: Clone>(table: &KeyedTable<T>, key: Kind) -> KeyedTable<T> {
    let mut entries: Vec<(Kind, RegistryTableEntry<T>)> = (table.entries).clone();
    {
        let __flight_key = (key).clone();
        if let Some(__flight_index) = entries.iter().position(|(key, _)| key == &__flight_key) {
            entries.remove(__flight_index);
            true
        } else {
            false
        }
    };
    return KeyedTable::<T> {
        __flight_identity: std::sync::Arc::new(()),
        entries: (entries).clone(),
        on_miss: (table.on_miss).clone(),
        registry: (table.registry).clone(),
        shape: "keyed".to_owned(),
    };
}

// Source: upstream/packages/registry/src/registryTable.ts:158 (sha256:d78d076e3c803c5f818e2d7ededea4b0736b6193686b98d7de47738edd7d6604)
pub fn with_registry_table_entry<T: Clone>(
    table: &KeyedTable<T>,
    key: Kind,
    value: T,
) -> KeyedTable<T> {
    let mut entries: Vec<(Kind, RegistryTableEntry<T>)> = (table.entries).clone();
    {
        let __flight_key = (key).clone();
        let __flight_value = flighthq_types::RegistryTableEntry::<T>::A(
            flighthq_types::RegistryTableEntryRecord2::<T> {
                __flight_identity: std::sync::Arc::new(()),
                state: (registry_entry_state_constant.bound).clone(),
                value: (value).clone(),
            },
        );
        if let Some((_, value)) = entries.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            entries.push((__flight_key, __flight_value));
        }
    };
    return KeyedTable::<T> {
        __flight_identity: std::sync::Arc::new(()),
        entries: (entries).clone(),
        on_miss: (table.on_miss).clone(),
        registry: (table.registry).clone(),
        shape: "keyed".to_owned(),
    };
}

// Source: upstream/packages/registry/src/registryTable.ts:171 (sha256:339cc94609622c72f5ad30c097300c0549883b1f94338fec6d3643cb3645dfc1)
pub fn with_registry_table_tombstone<T: Clone>(table: &KeyedTable<T>, key: Kind) -> KeyedTable<T> {
    let mut entries: Vec<(Kind, RegistryTableEntry<T>)> = (table.entries).clone();
    {
        let __flight_key = (key).clone();
        let __flight_value =
            flighthq_types::RegistryTableEntry::<T>::B(flighthq_types::RegistryTableEntryRecord1 {
                __flight_identity: std::sync::Arc::new(()),
                state: (registry_entry_state_constant.tombstoned).clone(),
            });
        if let Some((_, value)) = entries.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            entries.push((__flight_key, __flight_value));
        }
    };
    return KeyedTable::<T> {
        __flight_identity: std::sync::Arc::new(()),
        entries: (entries).clone(),
        on_miss: (table.on_miss).clone(),
        registry: (table.registry).clone(),
        shape: "keyed".to_owned(),
    };
}

// Source: upstream/packages/registry/src/registryTable.ts:180 (sha256:a97089ef0a6905cc566d54700b48748bb64a05f8915eff07c8d80c13c7881b5a)
fn get_registry_table_entry_state<T: Clone>(
    table: &RegistryTable<T>,
    key: Kind,
) -> Option<RegistryTableEntry<T>> {
    if matches!(&(table), flighthq_types::RegistryTable::<T>::A(_)) {
        return (match (*table).clone() {
            flighthq_types::RegistryTable::<T>::A(value) => value,
            flighthq_types::RegistryTable::<T>::B(_) => panic!("TypeScript union narrowing failed"),
        })
        .entries
        .iter()
        .find(|(entry_key, _)| entry_key == &(key).clone())
        .map(|(_, value)| value.clone());
    }
    if matches!(&(table), crate::FlightUnion2::B(crate::FlightUnion2::B(_))) {
        return if (key
            == ((match (*table).clone() {
                flighthq_types::RegistryTable::<T>::A(_) => {
                    panic!("TypeScript union narrowing failed")
                }
                flighthq_types::RegistryTable::<T>::B(value) => match value {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                },
            })
            .registry)
                .clone())
        {
            ((match (*table).clone() {
                flighthq_types::RegistryTable::<T>::A(_) => {
                    panic!("TypeScript union narrowing failed")
                }
                flighthq_types::RegistryTable::<T>::B(value) => match value {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                },
            })
            .entry)
                .clone()
        } else {
            None
        };
    }
    let ordinal = {
        let __flight_value = (key).clone();
        ((match (*table).clone() {
            flighthq_types::RegistryTable::<T>::A(_) => panic!("TypeScript union narrowing failed"),
            flighthq_types::RegistryTable::<T>::B(value) => match value {
                crate::FlightUnion2::A(value) => value,
                crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
            },
        })
        .vocabulary)
            .iter()
            .position(|item| item == &__flight_value)
            .map_or(-1.0_f64, |index| index as f64)
    };
    if (ordinal == (-1.0_f64)) {
        return None;
    }
    let value = (match (*table).clone() {
        flighthq_types::RegistryTable::<T>::A(_) => panic!("TypeScript union narrowing failed"),
        flighthq_types::RegistryTable::<T>::B(value) => match value {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        },
    })
    .entries[ordinal as usize]
        .clone();
    return if (value).is_none() {
        None
    } else {
        Some(flighthq_types::RegistryTableEntry::<T>::A(
            flighthq_types::RegistryTableEntryRecord2::<T> {
                __flight_identity: std::sync::Arc::new(()),
                state: (registry_entry_state_constant.bound).clone(),
                value: (value).clone().unwrap(),
            },
        ))
    };
}
