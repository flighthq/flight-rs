// @generated from upstream/packages/types/src/Md5SectionHandler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, ImportDiagnosticSeverity, Material, Md5Joint, Scene3DDocument};

// Source: upstream/packages/types/src/Md5SectionHandler.ts:10 (sha256:cf1b8caa896d336248b16b19ab45b5d6b2344710b519586149038a05afc1149a)
#[derive(Clone, Default)]
pub struct Md5DropTally {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub count: f64,
    pub detail: Vec<(
        String,
        crate::FlightUnion2<bool, crate::FlightUnion2<f64, String>>,
    )>,
    pub kind: String,
    pub severity: ImportDiagnosticSeverity,
}
impl PartialEq for Md5DropTally {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md5SectionHandler.ts:23 (sha256:50e06222b0d3fbe87b1d479683c7e1380362b631811d77a4cb016db5b367322a)
#[derive(Clone, Default)]
pub struct Md5MeshSection {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub materials: Vec<f64>,
    pub shader: String,
}
impl PartialEq for Md5MeshSection {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md5SectionHandler.ts:37 (sha256:82055db9b184242dc8a468436dabcf6ef5fcc4922dd960d702e7f774de8125d9)
#[derive(Clone, Default)]
pub struct Md5ParseContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub document: Scene3DDocument,
    pub drops: Option<Vec<(String, Md5DropTally)>>,
    pub joints: Vec<Md5Joint>,
    pub mesh: Option<Md5MeshSection>,
    pub skin: Option<f64>,
}
impl PartialEq for Md5ParseContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md5SectionHandler.ts:70 (sha256:d97c06326e07d90f138e923370fb380a27071411a0522576013f8b52084fc3c6)
#[derive(Clone)]
pub struct Md5SectionHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub feature: String,
    pub collect:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Md5ParseContext) -> () + Send + 'static>>>,
}
impl PartialEq for Md5SectionHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md5SectionHandler.ts:76 (sha256:32d55c93745f7916f1f2e5531d90fb84ca7f4bf72b2b307bd74efb8107d086f8)
pub const MD5_MATERIAL_FEATURE: &'static str = "Material";

// Source: upstream/packages/types/src/Md5SectionHandler.ts:79 (sha256:ae706c2baa638277439567cb557e086b67d00a39a9b131f0e2751651105752f4)
pub const MD5_SKELETON_FEATURE: &'static str = "Skeleton";

// Source: upstream/packages/types/src/Md5SectionHandler.ts:93 (sha256:5106bbf5b625df8c34fe2d01de3368828966aa220d12b2246ac79446d8fed927)
#[derive(Clone, Default)]
pub struct Md5ImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub section_handlers: Option<Vec<Md5SectionHandler>>,
}
impl PartialEq for Md5ImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
