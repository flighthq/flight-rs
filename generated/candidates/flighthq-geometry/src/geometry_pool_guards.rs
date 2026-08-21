// @generated from upstream/packages/geometry/src/geometryPoolGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/geometry/src/geometryPoolGuards.ts:1 (sha256:da95b055b0878feda0018e51fd66a664bcac77fce3efdcd53c09bd7ccab092a0)
pub(crate) type GeometryPoolReleaseFunction = String;

// Source: upstream/packages/geometry/src/geometryPoolGuards.ts:11 (sha256:f7972148574b4cc00da8960a399fec226827be072b108367f7604d10d84af87a)
pub(crate) type GeometryPoolReleaseGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(GeometryPoolReleaseFunction) -> () + Send + 'static>>,
>;

// Source: upstream/packages/geometry/src/geometryPoolGuards.ts:15 (sha256:473c24e737514b629025a21e63ff77e71d9bb25e612116681a27f20627ae9e22)
pub static GEOMETRY_POOL_RELEASE_GUARD: std::sync::LazyLock<
    std::sync::Mutex<Option<GeometryPoolReleaseGuard>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/geometry/src/geometryPoolGuards.ts:17 (sha256:b70b431157a944bd7bcb7e8efeabb0c362d824b0604a58be14bb864ead1e7cf9)
pub fn set_geometry_pool_release_guard(guard: &Option<GeometryPoolReleaseGuard>) -> () {
    (*GEOMETRY_POOL_RELEASE_GUARD.lock().unwrap()) = (*guard).clone();
}
