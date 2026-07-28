// @generated from upstream/packages/assets/src/assetLibrary.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    AssetEntry, AssetGroupLoadOptions, AssetLibrary, AssetLibraryRuntime, AssetLoaderAdapter,
    AssetManifest, AssetType,
};

// Source: upstream/packages/assets/src/assetLibrary.ts:19 (sha256:b45215505304a8b172aa8c9c031dd65f66d6a651666a58645db0f7b5e4307eda)
pub fn acquire_asset<T: Clone>(library: &mut AssetLibrary, id: String) -> crate::Promise<T> {
    let descriptor = library
        .runtime
        .descriptors
        .iter()
        .find(|(key, _)| key == &(id).clone())
        .map(|(_, value)| value.clone());
    if (descriptor).is_none() {
        return crate::host_value::<crate::Promise<T>>("host.reject");
    }
    let adapter = library
        .runtime
        .adapters
        .iter()
        .find(|(key, _)| key == &(descriptor.as_ref().unwrap().type_).clone())
        .map(|(_, value)| value.clone());
    if (adapter).is_none() {
        return crate::host_value::<crate::Promise<T>>("host.reject");
    }
    let mut existing = library
        .runtime
        .entries
        .iter()
        .find(|(key, _)| key == &(id).clone())
        .map(|(_, value)| value.clone());
    if (existing).is_some() {
        {
            existing.as_mut().unwrap().refcount += 1.0;
            existing.as_mut().unwrap().refcount
        };
        if existing.as_mut().unwrap().resident {
            return crate::host_value::<crate::Promise<T>>("host.resolve");
        }
        return ((existing.as_mut().unwrap().load_promise).clone()).unwrap();
    }
    let entry: std::sync::Arc<std::sync::Mutex<AssetEntry>> =
        std::sync::Arc::new(std::sync::Mutex::new(AssetEntry {
            __flight_identity: std::sync::Arc::new(()),
            value: crate::OpaqueHostValue::Undefined,
            refcount: 1.0_f64,
            load_promise: None,
            resident: false,
        }));
    {
        let __flight_key = (id).clone();
        let __flight_value = (*entry.lock().unwrap()).clone();
        if let Some((_, value)) = library
            .runtime
            .entries
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            library.runtime.entries.push((__flight_key, __flight_value));
        }
    };
    let load_promise = {
        let __flight_promise = {
            let __flight_callback = (adapter.as_ref().unwrap().load).clone();
            let __flight_result =
                __flight_callback.lock().unwrap()((descriptor.as_ref().unwrap()).clone());
            __flight_result
        };
        let __flight_callback = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let adapter = adapter.clone();
            let mut entry = entry.clone();
            let mut runtime = runtime.clone();
            move |value: T| -> () {
                if (!(((*runtime.lock().unwrap())
                    .entries
                    .iter()
                    .find(|(key, _)| key == &(id).clone())
                    .map(|(_, value)| value.clone()))
                    == Some((*entry.lock().unwrap()).clone())))
                    || ((*entry.lock().unwrap()).refcount <= 0.0_f64)
                {
                    {
                        let __flight_callback = (adapter.as_ref().unwrap().dispose).clone();
                        let __flight_result = __flight_callback.lock().unwrap()((value).clone());
                        __flight_result
                    };
                    return value;
                }
                (*entry.lock().unwrap()).value = value;
                (*entry.lock().unwrap()).resident = true;
                (*entry.lock().unwrap()).load_promise = None;
                return value;
            }
        })
            as Box<dyn FnMut(T) -> () + Send + 'static>));
        let _ = (&__flight_promise, &__flight_callback);
        crate::Promise::<()>::default()
    };
    (*entry.lock().unwrap()).load_promise = Some(load_promise);
    return load_promise;
}

// Source: upstream/packages/assets/src/assetLibrary.ts:58 (sha256:755a8e4302f984e1d215413315a2a9561bd9e0066a4e32b5150c9da008a77489)
#[derive(Clone, Default)]
struct CreateAssetLibraryRecord1 {
    __flight_identity: std::sync::Arc<()>,
    adapters: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    descriptors: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    entries: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    groups: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
}
impl PartialEq for CreateAssetLibraryRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_asset_library() -> AssetLibrary {
    let runtime: AssetLibraryRuntime = AssetLibraryRuntime {
        __flight_identity: std::sync::Arc::new(()),
        adapters: Vec::new(),
        descriptors: Vec::new(),
        entries: Vec::new(),
        groups: Vec::new(),
    };
    return AssetLibrary {
        __flight_identity: std::sync::Arc::new(()),
        runtime: (runtime).clone(),
    };
}

// Source: upstream/packages/assets/src/assetLibrary.ts:70 (sha256:d9a4d5fa9b9f681bc9d3c8eed76b63bc5b1bb546f6343f65ff4f170f9dd0a85c)
pub fn dispose_asset_library(library: &mut AssetLibrary) -> () {
    for __iteration0 in ((library.runtime.entries).clone()).iter().cloned() {
        let id = __iteration0[0.0_f64 as usize].clone();
        let entry = __iteration0[1.0_f64 as usize].clone();
        if (!entry.resident) {
            continue;
        }
        let descriptor = library
            .runtime
            .descriptors
            .iter()
            .find(|(key, _)| key == &id)
            .map(|(_, value)| value.clone());
        let adapter = if (descriptor).is_some() {
            library
                .runtime
                .adapters
                .iter()
                .find(|(key, _)| key == &(descriptor.as_ref().unwrap().type_).clone())
                .map(|(_, value)| value.clone())
        } else {
            None
        };
        if (adapter).is_some() {
            {
                let __flight_callback = (adapter.as_ref().unwrap().dispose).clone();
                let __flight_result = __flight_callback.lock().unwrap()((entry.value).clone());
                __flight_result
            };
        }
    }
    library.runtime.adapters.clear();
    library.runtime.descriptors.clear();
    library.runtime.entries.clear();
    library.runtime.groups.clear();
}

// Source: upstream/packages/assets/src/assetLibrary.ts:86 (sha256:8b47c3d6e95e449668370588eeb122a7178febd15e0c61ea5b45fde4c1514e99)
pub fn get_asset<T: Clone>(library: &AssetLibrary, id: String) -> Option<T> {
    let entry = library
        .runtime
        .entries
        .iter()
        .find(|(key, _)| key == &(id).clone())
        .map(|(_, value)| value.clone());
    return if ((entry).is_some()) && (entry.as_ref().unwrap().resident) {
        Some((entry.as_ref().unwrap().value).clone())
    } else {
        None
    };
}

// Source: upstream/packages/assets/src/assetLibrary.ts:93 (sha256:b73e7f2e39f832834731a356e53c3ddd4da91f6d6a2f92dac5e4ae42793d3ecc)
pub fn get_asset_ref_count(library: &AssetLibrary, id: String) -> f64 {
    let entry = library
        .runtime
        .entries
        .iter()
        .find(|(key, _)| key == &(id).clone())
        .map(|(_, value)| value.clone());
    return if (entry).is_some() {
        entry.as_ref().unwrap().refcount
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/assets/src/assetLibrary.ts:103 (sha256:f3bdc028a58d1524f00b1b963fb051844bec376d8010a26492a62cbd6dacea3b)
pub fn load_asset_group(
    library: AssetLibrary,
    name: String,
    options: Option<AssetGroupLoadOptions>,
) -> crate::Promise<crate::OpaqueHostValue> {
    Default::default()
}

// Source: upstream/packages/assets/src/assetLibrary.ts:141 (sha256:719d1d43473aca413ba6c12e0d04a0892923a54176310010ce400b685b832126)
pub fn load_asset_manifest(library: &mut AssetLibrary, manifest: &AssetManifest) -> () {
    for descriptor in (manifest).iter().cloned() {
        {
            let __flight_key = (descriptor.id).clone();
            let __flight_value = descriptor;
            if let Some((_, value)) = library
                .runtime
                .descriptors
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                library
                    .runtime
                    .descriptors
                    .push((__flight_key, __flight_value));
            }
        };
        if (descriptor.group).is_none() {
            continue;
        }
        let mut members = library
            .runtime
            .groups
            .iter()
            .find(|(key, _)| key == &(descriptor.group).clone())
            .map(|(_, value)| value.clone());
        if (members).is_none() {
            members = Some(vec![]);
            {
                let __flight_key = (descriptor.group).clone();
                let __flight_value = (members).clone().unwrap();
                if let Some((_, value)) = library
                    .runtime
                    .groups
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    library.runtime.groups.push((__flight_key, __flight_value));
                }
            };
        }
        if (!{
            let __flight_value = (descriptor.id).clone();
            (members)
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item == &__flight_value)
        }) {
            members.as_mut().unwrap().push((descriptor.id).clone());
        }
    }
}

// Source: upstream/packages/assets/src/assetLibrary.ts:158 (sha256:c8f5faf8f9ee2948104072ed3a7d3a044aa3131c833ef54a377120225b5276ab)
pub fn register_asset_loader<T: Clone>(
    library: &mut AssetLibrary,
    type_: AssetType,
    adapter: &AssetLoaderAdapter<T>,
) -> () {
    {
        let __flight_key = (type_).clone();
        let __flight_value = {
            let __flight_source = &((*adapter).clone());
            AssetLoaderAdapter::<crate::OpaqueHostValue> {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                load: (__flight_source.load).clone(),
                dispose: (__flight_source.dispose).clone(),
            }
        };
        if let Some((_, value)) = library
            .runtime
            .adapters
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            library
                .runtime
                .adapters
                .push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/assets/src/assetLibrary.ts:169 (sha256:897eb6ac3d724d7c9dd9380c38ac4b3bc85868065032138d7c0e6e276a4fa22e)
pub fn release_asset(library: &mut AssetLibrary, id: String) -> () {
    let mut entry = library
        .runtime
        .entries
        .iter()
        .find(|(key, _)| key == &(id).clone())
        .map(|(_, value)| value.clone());
    if (entry).is_none() {
        return;
    }
    {
        entry.as_mut().unwrap().refcount -= 1.0;
        entry.as_mut().unwrap().refcount
    };
    if (entry.as_mut().unwrap().refcount > 0.0_f64) {
        return;
    }
    dispose_asset_entry(&mut library.runtime, (id).clone(), &entry.as_mut().unwrap());
}

// Source: upstream/packages/assets/src/assetLibrary.ts:181 (sha256:c55a8bdf18006c0cdd1d60b66a6b9ba1e221fcbf9f197df91e45ad061892a3a0)
pub fn release_asset_group(library: &mut AssetLibrary, name: String) -> () {
    let ids = library
        .runtime
        .groups
        .iter()
        .find(|(key, _)| key == &(name).clone())
        .map(|(_, value)| value.clone());
    if (ids).is_none() {
        return;
    }
    for id in (ids.as_ref().unwrap()).iter().cloned() {
        release_asset(library, (id).clone());
    }
}

// Source: upstream/packages/assets/src/assetLibrary.ts:190 (sha256:0de46b55940effcfb7555dc8f17f99c24aa0dbbf8e8349535688e8c1c43d4908)
fn dispose_asset_entry(runtime: &mut AssetLibraryRuntime, id: String, entry: &AssetEntry) -> () {
    {
        let __flight_key = (id).clone();
        if let Some(__flight_index) = runtime
            .entries
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            runtime.entries.remove(__flight_index);
            true
        } else {
            false
        }
    };
    if (!entry.resident) {
        return;
    }
    let descriptor = runtime
        .descriptors
        .iter()
        .find(|(key, _)| key == &(id).clone())
        .map(|(_, value)| value.clone());
    let adapter = if (descriptor).is_some() {
        runtime
            .adapters
            .iter()
            .find(|(key, _)| key == &(descriptor.as_ref().unwrap().type_).clone())
            .map(|(_, value)| value.clone())
    } else {
        None
    };
    if (adapter).is_some() {
        {
            let __flight_callback = (adapter.as_ref().unwrap().dispose).clone();
            let __flight_result = __flight_callback.lock().unwrap()((entry.value).clone());
            __flight_result
        };
    }
}
