// @generated from upstream/packages/types/src/Dialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AppWindow, EntityRuntime};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: Option<FileDialogHandle>,
    pub outcome: String,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub facing_mode: Option<MediaCaptureDialogFacingMode>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:6 (sha256:f34aa1677e841ff6c792a56b19e811b3c069b88092ba22b0c2a650e93080e091)
#[derive(Clone, Default)]
pub struct FileDialogFilter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accept: Vec<(String, Vec<String>)>,
    pub name: String,
}
impl PartialEq for FileDialogFilter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:14 (sha256:b5be2dbadf33ae5cccd92f8c8bb0afc9630f1ea90ecf0b9db71ae346fe10bdc0)
#[derive(Clone, Default)]
pub struct FileDialogHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: String,
    pub name: String,
    pub path: Option<String>,
}
impl PartialEq for FileDialogHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FileDialogHandle {
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

// Source: upstream/packages/types/src/Dialog.ts:22 (sha256:b21011a0d73d21fa5f4a09965d3d2a250481579287f8af22a173fed5c2e3a801)
#[derive(Clone, Default)]
pub struct FileDialogHandleOperations {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub read_binary: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(Option<crate::OpaqueHostValue>) -> crate::FlightTask<Option<Vec<u8>>>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub read_text: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(Option<crate::OpaqueHostValue>) -> crate::FlightTask<Option<String>>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub write_binary: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(Vec<u8>, Option<crate::OpaqueHostValue>) -> crate::FlightTask<bool>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub write_text: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(String, Option<crate::OpaqueHostValue>) -> crate::FlightTask<bool>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for FileDialogHandleOperations {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:29 (sha256:4d4bfeaf326c7b6ded66d7fb8dddac98dc7da72828012afc8c29b6bcfff4dd84)
pub type FileDialogHandleRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Dialog.ts:33 (sha256:e5504b2207169e92925a50c11f5dc49d524660c3f228c5a6304bf93c32e64db1)
#[derive(Clone, Default)]
pub struct OpenFileDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub filters: Option<Vec<FileDialogFilter>>,
    pub multiple: Option<bool>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for OpenFileDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:39 (sha256:3b0bf6516d82d0b4a16452670965fc30a41da728216ba1e903972dd99c7b893b)
#[derive(Clone, Default)]
pub struct OpenDirectoryDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for OpenDirectoryDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:43 (sha256:1c1da0b9d69672b797b5c784008c481da7e6a9444b684756e558c1be691c6cb8)
#[derive(Clone, Default)]
pub struct SaveFileDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub default_name: Option<String>,
    pub filters: Option<Vec<FileDialogFilter>>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for SaveFileDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:49 (sha256:6c9f1d87b91003079637f01f9b3101eebd4144807ec011f29a6b5a7d237ed80f)
#[derive(Clone, Default)]
pub struct FileOpenDialogResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handles: Option<Vec<FileDialogHandle>>,
    pub outcome: String,
}
impl PartialEq for FileOpenDialogResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:55 (sha256:a72deb6dd76f2e6b36f7ccc5ab201668c1cb76567c54b0078c65932166800ea4)
#[derive(Clone, Default)]
pub struct DirectoryOpenDialogResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: Option<FileDialogHandle>,
    pub outcome: String,
}
impl PartialEq for DirectoryOpenDialogResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:61 (sha256:3da5114b3a648a09fefbe6256f7a4bf3ebcca8f10a3de0018b5d48e1adc8ae52)
#[derive(Clone, Default)]
pub struct FileSaveDialogResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: Option<FileDialogHandle>,
    pub outcome: String,
}
impl PartialEq for FileSaveDialogResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:67 (sha256:dd6768002d2a3fee25086d4f5e50fac81e445e80e0adadc72476732ea132cfa5)
#[derive(Clone, Default)]
pub struct OpenImageDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for OpenImageDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:73 (sha256:b0443a3680add911b9e9813960a68d3c5a2808b3c7f38b9f67dbdd8da0b76f7e)
pub type MediaCaptureDialogFacingMode = String;

// Source: upstream/packages/types/src/Dialog.ts:75 (sha256:89b487ee430e5924b7db3afc411b07cf9f220b6e23cc2b96b6e957357c2c666f)
#[derive(Clone, Default)]
pub struct CapturePhotoDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facing_mode: Option<MediaCaptureDialogFacingMode>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for CapturePhotoDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:80 (sha256:ee258ba8655a6810a922cfdf14f62cf0a199020de4b537d02c6211b498d7cd73)
#[derive(Clone, Default)]
pub struct CaptureVideoDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facing_mode: Option<MediaCaptureDialogFacingMode>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for CaptureVideoDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:85 (sha256:621d224c0a6ef4c12402dec101f86bd34b7e1800fdad5895ba8b54df4be580cc)
#[derive(Clone, Default)]
pub struct DialogImage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data_url: String,
    pub height: f64,
    pub mime_type: String,
    pub width: f64,
}
impl PartialEq for DialogImage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:92 (sha256:0d3d989fa58573d40931676ff04f7664d8db679d268bb1ccb62a80c1bec33235)
#[derive(Clone, Default)]
pub struct DialogVideo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data_url: String,
    pub duration: f64,
    pub mime_type: String,
}
impl PartialEq for DialogVideo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:99 (sha256:c337c90b83b65435f9205fe1093258d24069c996ba73a251672abdff0041d19b)
#[derive(Clone, Default)]
pub struct ImageOpenDialogResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image: Option<DialogImage>,
    pub outcome: String,
}
impl PartialEq for ImageOpenDialogResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:105 (sha256:dc98ee4af0b9873eff6e3e5b2cf8874fe4eea7b040f8cb1602943a90391da523)
#[derive(Clone, Default)]
pub struct PhotoCaptureDialogResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
    pub photo: Option<DialogImage>,
}
impl PartialEq for PhotoCaptureDialogResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:111 (sha256:e571746e6ee356b7faab1cf8fa626c5821acba9dfe0b0cd487454dcd3ec7218d)
#[derive(Clone, Default)]
pub struct VideoCaptureDialogResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
    pub video: Option<DialogVideo>,
}
impl PartialEq for VideoCaptureDialogResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:118 (sha256:ae71eb25f12b9694d783cc5cce988488228bce728b69bb5f8b4014485a1b2842)
#[derive(Clone, Default)]
pub struct PromptDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: Option<String>,
    pub message: String,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub parent_window: Option<AppWindow>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for PromptDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:128 (sha256:9869481c0caaab6137630affbecc226ba834f0bc836e73934890ca1036ec47f6)
pub type MessageDialogKind = String;

// Source: upstream/packages/types/src/Dialog.ts:130 (sha256:adf049892fc881643f3ed9fc0283a43b104f3951c264aa137cd8e28f555d3826)
#[derive(Clone, Default)]
pub struct MessageDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: Option<String>,
    pub message: String,
    pub detail: Option<String>,
    pub buttons: Option<Vec<String>>,
    pub kind: Option<MessageDialogKind>,
    pub checkbox_label: Option<String>,
    pub checkbox_checked: Option<bool>,
    pub default_id: Option<f64>,
    pub cancel_id: Option<f64>,
    pub parent_window: Option<AppWindow>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for MessageDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Dialog.ts:151 (sha256:5955c19ad7714113252dfbab0942d980da86468e0febf98aaf04b3bfc2ce69c6)
#[derive(Clone, Default)]
pub struct MessageDialogResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub button_index: f64,
    pub cancelled: bool,
    pub checkbox_checked: bool,
}
impl PartialEq for MessageDialogResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
