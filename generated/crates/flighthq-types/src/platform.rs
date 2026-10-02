// @generated from upstream/packages/types/src/Platform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Platform.ts:6 (sha256:a04ed6cec006a086a4d2d32f5db1c3f1d642b28d54d6d8ba609bb912bc9791c9)
pub type PlatformName = String;

// Source: upstream/packages/types/src/Platform.ts:9 (sha256:df06c4e36f45ac446a2f32a4810fd8c234d807a5f335568488f7a99f6e31b942)
pub type PlatformEndianness = String;

// Source: upstream/packages/types/src/Platform.ts:12 (sha256:3d95d4c6e2bfe44636f154e6348414c571495d2a23dce6577d1522f1c262f95e)
pub type PlatformEngine = String;

// Source: upstream/packages/types/src/Platform.ts:14 (sha256:82610121fe0e13d70e44e152b6a712ed0a26a6f86a7e2b07a96baacc3353b815)
pub type PlatformKind = String;

// Source: upstream/packages/types/src/Platform.ts:18 (sha256:28a791deb1179319a937bc7fa7b2a28f774d50b4a0ee34417f1d2a710a0bc749)
pub type PlatformRuntime = String;

// Source: upstream/packages/types/src/Platform.ts:20 (sha256:e512dd1f3520f3f7d17a5567176782bc1f6cab6945e1bf94b0e8c118e35aa296)
#[derive(Clone, Default)]
pub struct PlatformInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub name: PlatformName,
    pub kind: PlatformKind,
    pub version: String,
    pub arch: String,
    pub locale: String,
    pub is_touch: bool,
    pub runtime: PlatformRuntime,
    pub engine: PlatformEngine,
    pub engine_version: String,
    pub endianness: PlatformEndianness,
    pub pointer_width: f64,
    pub os_build: String,
    pub distro: String,
    pub distro_version: String,
}
impl PartialEq for PlatformInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for PlatformInfo {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Platform.ts:50 (sha256:b745dd4c46c361c414cdf9bcc0ae041cb23c0b65ca9972ab08440823a1f6f082)
#[derive(Clone)]
pub struct HostPlatformCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_info: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(PlatformInfo) -> PlatformInfo + Send + 'static>>,
    >,
}
impl PartialEq for HostPlatformCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
