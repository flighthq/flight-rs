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
#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
}

// Source: upstream/packages/types/src/FileSystem.ts:10 (sha256:33ee2b08d331911b3bc41b64fda2427f2fc6adaa1c99162cfb8baa97d6bdebc1)
pub type FileWatchEventType = String;

// Source: upstream/packages/types/src/FileSystem.ts:12 (sha256:9f015c20852be1b166ac56bf9148df155faabf9799539f10ed50547cfee36429)
#[derive(Clone)]
pub struct FileWatchEvent {
    pub type_: FileWatchEventType,
    pub path: String,
}

// Source: upstream/packages/types/src/FileSystem.ts:17 (sha256:cb6abf1b2c839e4b1f961dd9e9135a46fd52df8599153d567bf91ceb3dff67a9)
#[derive(Clone)]
pub struct FileStat {
    pub size: f64,
    pub is_directory: bool,
    pub modified_time: f64,
    pub created_time: f64,
    pub is_symlink: bool,
}

// Source: upstream/packages/types/src/FileSystem.ts:27 (sha256:362f818235d81511bbb0d5952e5fa14f7fb229aed67b2d2f0942d008058a78a4)
#[derive(Clone)]
pub struct FilePermissions {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

// Source: upstream/packages/types/src/FileSystem.ts:35 (sha256:841067e277893e01b880c558cd5378868092395d8600f4cef5306fdf9c9cfa60)
#[derive(Clone)]
pub struct FileSystemUsage {
    pub used_bytes: f64,
    pub quota_bytes: f64,
}

// Source: upstream/packages/types/src/FileSystem.ts:41 (sha256:034031679c1d16ee2669a56f80a4e8264e80521c09861939d1435e1a4c27fffe)
#[derive(Clone)]
pub struct FileWalkOptions {
    pub max_depth: Option<f64>,
}

// Source: upstream/packages/types/src/FileSystem.ts:49 (sha256:a026f347634dfc6e47086381b646b20b3e8f292c8aacbe9f9b46c7f8536cfc7d)
#[derive(Clone)]
pub struct FileSystemBackend {
    pub read_text_file: crate::OpaqueHostValue,
    pub write_text_file: crate::OpaqueHostValue,
    pub read_binary_file: crate::OpaqueHostValue,
    pub read_binary_file_range: crate::OpaqueHostValue,
    pub write_binary_file: crate::OpaqueHostValue,
    pub write_file_atomic: crate::OpaqueHostValue,
    pub file_exists: crate::OpaqueHostValue,
    pub directory_exists: crate::OpaqueHostValue,
    pub remove_file: crate::OpaqueHostValue,
    pub remove_directory: crate::OpaqueHostValue,
    pub make_directory: crate::OpaqueHostValue,
    pub read_directory: crate::OpaqueHostValue,
    pub read_directory_recursive: crate::OpaqueHostValue,
    pub stat_file: crate::OpaqueHostValue,
    pub rename: crate::OpaqueHostValue,
    pub copy: crate::OpaqueHostValue,
    pub append_text_file: crate::OpaqueHostValue,
    pub open_file_read_stream: crate::OpaqueHostValue,
    pub open_file_write_stream: crate::OpaqueHostValue,
    pub create_file_symlink: crate::OpaqueHostValue,
    pub read_file_symlink: crate::OpaqueHostValue,
    pub get_file_real_path: crate::OpaqueHostValue,
    pub get_file_permissions: crate::OpaqueHostValue,
    pub set_file_permissions: crate::OpaqueHostValue,
    pub can_access_file: crate::OpaqueHostValue,
    pub get_file_system_usage: crate::OpaqueHostValue,
    pub watch: crate::OpaqueHostValue,
    pub get_path: crate::OpaqueHostValue,
}
