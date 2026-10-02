// @generated from upstream/packages/types/src/FileSystem.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FileSystem.ts:2 (sha256:2718cbb95074b86ea0727156e30cbba423b170648c186273724302d5fe108b14)
pub type FileSystemPathKind = String;

// Source: upstream/packages/types/src/FileSystem.ts:4 (sha256:798e6ae7b339bf8fda3e53144f6b89168f0861e6787d5800a65326bd26bc99f9)
#[derive(Clone, Default)]
pub struct FileEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub path: String,
    pub is_directory: bool,
}
impl PartialEq for FileEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FileSystem.ts:10 (sha256:33ee2b08d331911b3bc41b64fda2427f2fc6adaa1c99162cfb8baa97d6bdebc1)
pub type FileWatchEventType = String;

// Source: upstream/packages/types/src/FileSystem.ts:12 (sha256:9f015c20852be1b166ac56bf9148df155faabf9799539f10ed50547cfee36429)
#[derive(Clone, Default)]
pub struct FileWatchEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub type_: FileWatchEventType,
    pub path: String,
}
impl PartialEq for FileWatchEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FileSystem.ts:17 (sha256:cb6abf1b2c839e4b1f961dd9e9135a46fd52df8599153d567bf91ceb3dff67a9)
#[derive(Clone, Default)]
pub struct FileStat {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub size: f64,
    pub is_directory: bool,
    pub modified_time: f64,
    pub created_time: f64,
    pub is_symlink: bool,
}
impl PartialEq for FileStat {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FileSystem.ts:27 (sha256:362f818235d81511bbb0d5952e5fa14f7fb229aed67b2d2f0942d008058a78a4)
#[derive(Clone, Default)]
pub struct FilePermissions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}
impl PartialEq for FilePermissions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FileSystem.ts:35 (sha256:841067e277893e01b880c558cd5378868092395d8600f4cef5306fdf9c9cfa60)
#[derive(Clone, Default)]
pub struct FileSystemUsage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub used_bytes: f64,
    pub quota_bytes: f64,
}
impl PartialEq for FileSystemUsage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FileSystem.ts:41 (sha256:2eadb69bcf0b53c0ec04ac655ae956cb636ae9289d0ad7bdd2f6c9052a92c6cf)
#[derive(Clone)]
pub struct FileSystemBasicOperations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub append_text_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, String, Option<crate::OpaqueHostValue>) -> crate::FlightTask<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub copy: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, String) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
    pub directory_exists: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub file_exists: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub make_directory: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub read_binary_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Option<crate::OpaqueHostValue>,
                    ) -> crate::FlightTask<Option<Vec<u8>>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_directory: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Option<crate::OpaqueHostValue>,
                    ) -> crate::FlightTask<Vec<FileEntry>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_text_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Option<crate::OpaqueHostValue>,
                    ) -> crate::FlightTask<Option<String>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_directory: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, Option<bool>) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
    pub remove_file: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub rename: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, String) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
    pub stat_file: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> crate::FlightTask<Option<FileStat>> + Send + 'static>,
        >,
    >,
    pub write_binary_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        Vec<u8>,
                        Option<crate::OpaqueHostValue>,
                    ) -> crate::FlightTask<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub write_text_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, String, Option<crate::OpaqueHostValue>) -> crate::FlightTask<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for FileSystemBasicOperations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FileSystem.ts:62 (sha256:79226d300d763465253aaccd0e3e900a41f22802308292f3eba0d562156b8e94)
#[derive(Clone, Default)]
pub struct HostFileSystemCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub can_access_file: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(String, String) -> crate::FlightTask<bool> + Send + 'static>,
            >,
        >,
    >,
    pub get_file_system_usage: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut() -> crate::FlightTask<Option<FileSystemUsage>> + Send + 'static>,
            >,
        >,
    >,
    pub open_file_read_stream: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            Option<crate::OpaqueHostValue>,
                        )
                            -> crate::FlightTask<Option<crate::OpaqueHostValue>>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub open_file_write_stream: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            Option<crate::OpaqueHostValue>,
                        )
                            -> crate::FlightTask<Option<crate::OpaqueHostValue>>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub read_binary_file_range: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            f64,
                            f64,
                            Option<crate::OpaqueHostValue>,
                        ) -> crate::FlightTask<Option<Vec<u8>>>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub read_directory_recursive: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(String, Option<FileWalkOptions>) -> crate::FlightTask<Vec<FileEntry>>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub write_file_atomic: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            crate::FlightUnion2<Vec<u8>, String>,
                            Option<crate::OpaqueHostValue>,
                        ) -> crate::FlightTask<bool>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for HostFileSystemCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FileSystem.ts:73 (sha256:4f0162b493a1b12c816645720048ed08509d724a23f20683f4cf52d54b1277fa)
#[derive(Clone, Default)]
pub struct FileWalkOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_depth: Option<f64>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for FileWalkOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
