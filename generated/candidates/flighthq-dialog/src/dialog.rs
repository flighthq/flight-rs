// @generated from upstream/packages/dialog/src/dialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    ApplicationWindow, DialogBackend, FileDialogHandle, FileDialogStartIn, MessageDialogOptions,
    MessageDialogResult, OpenDirectoryDialogOptions, OpenFileDialogOptions, PromptDialogOptions,
    SaveFileDialogOptions,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub accept: Vec<(String, Vec<String>)>,
    pub description: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub webkitdirectory: bool,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:17 (sha256:5a4253d6c5ac2cccf58826d79fb5dc9e4f36bcccdaf28708217fcfe485bdc451)
pub fn create_web_dialog_backend() -> DialogBackend {
    return DialogBackend {
        __flight_identity: std::sync::Arc::new(()),
        confirm: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: MessageDialogOptions| -> crate::Promise<crate::OpaqueHostValue> {
                return false;
            },
        )
            as Box<dyn FnMut(MessageDialogOptions) -> crate::Promise<bool> + Send + 'static>)),
        message: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: MessageDialogOptions| -> crate::Promise<crate::OpaqueHostValue> {
                let checkbox_checked = (options.checkbox_checked).unwrap_or(false);
                {
                    return MessageDialogResult {
                        __flight_identity: std::sync::Arc::new(()),
                        button_index: 0.0_f64,
                        cancelled: false,
                        checkbox_checked: checkbox_checked,
                    };
                }
            },
        )
            as Box<
                dyn FnMut(MessageDialogOptions) -> crate::Promise<MessageDialogResult>
                    + Send
                    + 'static,
            >)),
        open_directory: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: OpenDirectoryDialogOptions| -> crate::Promise<Vec<FileDialogHandle>> {
                return open_web_directory_dialog((options).clone());
            },
        )
            as Box<
                dyn FnMut(OpenDirectoryDialogOptions) -> crate::Promise<Vec<FileDialogHandle>>
                    + Send
                    + 'static,
            >)),
        open_file: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: OpenFileDialogOptions| -> crate::Promise<Vec<FileDialogHandle>> {
                return open_web_file_dialog((options).clone());
            },
        )
            as Box<
                dyn FnMut(OpenFileDialogOptions) -> crate::Promise<Vec<FileDialogHandle>>
                    + Send
                    + 'static,
            >)),
        prompt: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: PromptDialogOptions| -> crate::Promise<crate::OpaqueHostValue> {
                return None;
            },
        )
            as Box<
                dyn FnMut(PromptDialogOptions) -> crate::Promise<Option<String>> + Send + 'static,
            >)),
        save_file: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: SaveFileDialogOptions| -> crate::Promise<crate::OpaqueHostValue> {
                return save_web_file(&options);
            },
        )
            as Box<
                dyn FnMut(SaveFileDialogOptions) -> crate::Promise<Option<FileDialogHandle>>
                    + Send
                    + 'static,
            >)),
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:63 (sha256:f618be040bb83fbb65a94a1c223a2c0f1848f54fe0715d60edf89497bbeeb7ea)
pub fn get_dialog_backend() -> DialogBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_dialog_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/dialog/src/dialog.ts:71 (sha256:9d589de968d392612d2cf0a19937cb5906ff451c3b6134a77173e4044955202f)
pub fn get_web_directory_system_handle(
    handle: &FileDialogHandle,
) -> Option<crate::OpaqueHostValue> {
    return (*_FILE_SYSTEM_DIRECTORY_HANDLE_REGISTRY.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*handle).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/dialog/src/dialog.ts:78 (sha256:990efff96750d3e9b1ef342191583818ddd5edd227b59c8824259b04fd55af29)
pub fn get_web_file_system_handle(handle: &FileDialogHandle) -> Option<crate::OpaqueHostValue> {
    return (*_FILE_SYSTEM_HANDLE_REGISTRY.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*handle).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/dialog/src/dialog.ts:83 (sha256:490f4976d24c6110114d780901003a6a948c25837ade67d5d34cc8ade339f7b3)
pub fn set_dialog_backend(backend: Option<DialogBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/dialog/src/dialog.ts:88 (sha256:cace4f5ddac8b7fcae47cc544f7df8b5dd92d1a5df3a28b9d036d7c0afe82dda)
pub fn show_confirm_dialog(options: &MessageDialogOptions) -> crate::Promise<bool> {
    return {
        let __flight_callback = (get_dialog_backend().confirm).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*options).clone());
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:94 (sha256:83e6db7e81141b364d7dd5c2ea6998324091a8df4b491197924eabc5c84310ce)
#[derive(Clone, Default)]
struct ShowErrorBoxSynthesizedRecord2998621112 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
    message: String,
    title: String,
}
impl PartialEq for ShowErrorBoxSynthesizedRecord2998621112 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn show_error_box(title: String, content: String) -> crate::Promise<MessageDialogResult> {
    return {
        let __flight_callback = (get_dialog_backend().message).clone();
        let __flight_result = __flight_callback.lock().unwrap()({
            let __flight_source = &(ShowErrorBoxSynthesizedRecord2998621112 {
                __flight_identity: std::sync::Arc::new(()),
                kind: "error".to_owned(),
                message: (content).clone(),
                title: (title).clone(),
            });
            MessageDialogOptions {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                title: Some((__flight_source.title).clone()),
                message: (__flight_source.message).clone(),
                detail: None,
                buttons: None,
                kind: Some((__flight_source.kind).clone()),
                checkbox_label: None,
                checkbox_checked: None,
                default_id: None,
                cancel_id: None,
                parent_window: None,
            }
        });
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:99 (sha256:21490604628d2aeaf798490688178e5fd8505008ffd13d45eca42b6d6bc4f9ec)
#[derive(Clone, Default)]
struct ShowErrorDialogSynthesizedRecord1450939776 {
    __flight_identity: std::sync::Arc<()>,
    buttons: Option<Vec<String>>,
    cancel_id: Option<f64>,
    checkbox_checked: Option<bool>,
    checkbox_label: Option<String>,
    default_id: Option<f64>,
    detail: Option<String>,
    kind: String,
    message: String,
    parent_window: Option<ApplicationWindow>,
    title: Option<String>,
}
impl PartialEq for ShowErrorDialogSynthesizedRecord1450939776 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn show_error_dialog(options: &MessageDialogOptions) -> crate::Promise<MessageDialogResult> {
    return {
        let __flight_callback = (get_dialog_backend().message).clone();
        let __flight_result = __flight_callback.lock().unwrap()({
            let __flight_source = &({
                let __flight_spread_0 = options;
                ShowErrorDialogSynthesizedRecord1450939776 {
                    __flight_identity: std::sync::Arc::new(()),
                    buttons: (__flight_spread_0.buttons).clone(),
                    cancel_id: __flight_spread_0.cancel_id,
                    checkbox_checked: __flight_spread_0.checkbox_checked,
                    checkbox_label: (__flight_spread_0.checkbox_label).clone(),
                    default_id: __flight_spread_0.default_id,
                    detail: (__flight_spread_0.detail).clone(),
                    kind: "error".to_owned(),
                    message: (__flight_spread_0.message).clone(),
                    parent_window: (__flight_spread_0.parent_window).clone(),
                    title: (__flight_spread_0.title).clone(),
                }
            });
            MessageDialogOptions {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                title: (__flight_source.title).clone(),
                message: (__flight_source.message).clone(),
                detail: (__flight_source.detail).clone(),
                buttons: (__flight_source.buttons).clone(),
                kind: Some((__flight_source.kind).clone()),
                checkbox_label: (__flight_source.checkbox_label).clone(),
                checkbox_checked: __flight_source.checkbox_checked,
                default_id: __flight_source.default_id,
                cancel_id: __flight_source.cancel_id,
                parent_window: (__flight_source.parent_window).clone(),
            }
        });
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:104 (sha256:67d11b7b237ac6d07afcc39d4538c12b689a18c5b342058265999c1204d97e7a)
#[derive(Clone, Default)]
struct ShowInfoDialogSynthesizedRecord1450939776 {
    __flight_identity: std::sync::Arc<()>,
    buttons: Option<Vec<String>>,
    cancel_id: Option<f64>,
    checkbox_checked: Option<bool>,
    checkbox_label: Option<String>,
    default_id: Option<f64>,
    detail: Option<String>,
    kind: String,
    message: String,
    parent_window: Option<ApplicationWindow>,
    title: Option<String>,
}
impl PartialEq for ShowInfoDialogSynthesizedRecord1450939776 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn show_info_dialog(options: &MessageDialogOptions) -> crate::Promise<MessageDialogResult> {
    return {
        let __flight_callback = (get_dialog_backend().message).clone();
        let __flight_result = __flight_callback.lock().unwrap()({
            let __flight_source = &({
                let __flight_spread_0 = options;
                ShowInfoDialogSynthesizedRecord1450939776 {
                    __flight_identity: std::sync::Arc::new(()),
                    buttons: (__flight_spread_0.buttons).clone(),
                    cancel_id: __flight_spread_0.cancel_id,
                    checkbox_checked: __flight_spread_0.checkbox_checked,
                    checkbox_label: (__flight_spread_0.checkbox_label).clone(),
                    default_id: __flight_spread_0.default_id,
                    detail: (__flight_spread_0.detail).clone(),
                    kind: "info".to_owned(),
                    message: (__flight_spread_0.message).clone(),
                    parent_window: (__flight_spread_0.parent_window).clone(),
                    title: (__flight_spread_0.title).clone(),
                }
            });
            MessageDialogOptions {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                title: (__flight_source.title).clone(),
                message: (__flight_source.message).clone(),
                detail: (__flight_source.detail).clone(),
                buttons: (__flight_source.buttons).clone(),
                kind: Some((__flight_source.kind).clone()),
                checkbox_label: (__flight_source.checkbox_label).clone(),
                checkbox_checked: __flight_source.checkbox_checked,
                default_id: __flight_source.default_id,
                cancel_id: __flight_source.cancel_id,
                parent_window: (__flight_source.parent_window).clone(),
            }
        });
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:110 (sha256:0dab0baa9e59a6ed34bdc98adb9864b6aa965e0389bf221ab900fd6d3b028a18)
pub fn show_message_dialog(options: &MessageDialogOptions) -> crate::Promise<MessageDialogResult> {
    return {
        let __flight_callback = (get_dialog_backend().message).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*options).clone());
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:116 (sha256:fe6d399fb3af8c31b8774e0da9e4f591a38d2811efc5cf8796256a427a49c418)
pub fn show_open_directory_dialog(
    options: &OpenDirectoryDialogOptions,
) -> crate::Promise<Vec<FileDialogHandle>> {
    return {
        let __flight_callback = (get_dialog_backend().open_directory).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*options).clone());
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:122 (sha256:7ba830b2f542c8b7ca0ecee76aaeab7d9669d4978fe866f1348e83b749388278)
pub fn show_open_file_dialog(
    options: &OpenFileDialogOptions,
) -> crate::Promise<Vec<FileDialogHandle>> {
    return {
        let __flight_callback = (get_dialog_backend().open_file).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*options).clone());
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:128 (sha256:e0561887e91a8fa624c9b7f7d69f10c1e4f97761fc197517c96d97b91c26a683)
pub fn show_prompt_dialog(options: &PromptDialogOptions) -> crate::Promise<Option<String>> {
    return {
        let __flight_callback = (get_dialog_backend().prompt).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*options).clone());
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:135 (sha256:651b1757700b38fdf4f0f947ad781d00aa7b40ac26ff6915b76f57593490cb75)
pub fn show_save_file_dialog(
    options: &SaveFileDialogOptions,
) -> crate::Promise<Option<FileDialogHandle>> {
    return {
        let __flight_callback = (get_dialog_backend().save_file).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*options).clone());
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:139 (sha256:c07292898ae42b80221bdebccdd7a7f32b5fc90bf895c4dd0c204afef9cfc2f6)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<DialogBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/dialog/src/dialog.ts:142 (sha256:122ebe4bca9c345cc3f64177087c93acea1d30b10ca164cf851330f91fab3293)
#[derive(Clone, Default)]
struct BuildFileSystemAccessTypesRecord3 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for BuildFileSystemAccessTypesRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn build_file_system_access_types(
    filters: crate::OpaqueHostValue,
) -> Option<Vec<SharedStructuralRecord1>> {
    if ((filters).is_none()) || (crate::host_value::<f64>("host.length") == 0.0_f64) {
        return None;
    }
    let mut types: Vec<SharedStructuralRecord1> = vec![];
    for filter in (filters).iter().cloned() {
        let mut accept: Vec<(String, Vec<String>)> = {
            let mut __flight_record = Vec::new();
            __flight_record
        };
        let extensions = ((filter.extensions.filter)(std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move |e: crate::OpaqueHostValue| -> f64 { (e != "*") })
                as Box<dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static>,
        )))
        .map)(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |e: crate::OpaqueHostValue| -> f64 {
                if match &(crate::host_value::<()>("host.startsWith")) {
                    crate::OpaqueHostValue::Undefined | crate::OpaqueHostValue::Null => false,
                    crate::OpaqueHostValue::Bool(value) => *value,
                    crate::OpaqueHostValue::Number(value) => *value != 0.0_f64 && !value.is_nan(),
                    crate::OpaqueHostValue::String(value) => !value.is_empty(),
                    crate::OpaqueHostValue::Object => true,
                } {
                    e
                } else {
                    format!(".{}", e)
                }
            },
        )
            as Box<dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static>)));
        if (extensions.length > 0.0_f64) {
            let mime = if (filter.mime_types) && (filter.mime_types.length > 0.0_f64) {
                filter.mime_types[0.0_f64 as usize].clone()
            } else {
                "application/octet-stream".to_owned()
            };
            accept
                .iter()
                .find(|(key, _)| key == &(mime).clone())
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = extensions;
        }
        if filter.mime_types {
            for mime in (filter.mime_types).iter().cloned() {
                if false {
                    accept
                        .iter()
                        .find(|(key, _)| key == &(mime).clone())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent") = extensions;
                }
            }
        }
        if (crate::host_value::<()>("host.keys").length == 0.0_f64) {
            continue;
        }
        types.push(SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            accept: (accept).clone(),
            description: (filter.name).clone(),
        });
    }
    return if ((types.len() as f64) > 0.0_f64) {
        Some((types).clone())
    } else {
        None
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:166 (sha256:4cf24806d4e7e8e046c390ca21bf19aae91e4b7c0f2c49e2336d7860f7e72b63)
fn build_accept_attribute(filters: crate::OpaqueHostValue) -> String {
    if ((filters).is_none()) || (crate::host_value::<f64>("host.length") == 0.0_f64) {
        return "".to_owned();
    }
    let mut parts: Vec<String> = vec![];
    for filter in (filters).iter().cloned() {
        for extension in (filter.extensions).iter().cloned() {
            if (extension == "*") {
                continue;
            }
            parts.push(if (extension.starts_with)(".") {
                extension
            } else {
                format!(".{}", extension)
            });
        }
        if filter.mime_types {
            for mime in (filter.mime_types).iter().cloned() {
                parts.push(mime);
            }
        }
    }
    return (parts.join)(",");
}

// Source: upstream/packages/dialog/src/dialog.ts:186 (sha256:a65902736d9a3b07b1cfa3e78b34f40a866b595b2c1a782bc4c306e895de117b)
fn to_file_system_access_start_in(start_in: FileDialogStartIn) -> Option<String> {
    let allowed: Vec<crate::OpaqueHostValue> = Vec::new();
    return if allowed.iter().any(|item| item == &start_in) {
        Some(start_in)
    } else {
        None
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:195 (sha256:d23c89cc3f12e499a97e575cf416c199912c8b64d6de43a2de591807b4cf9d32)
fn open_web_directory_dialog(
    options: OpenDirectoryDialogOptions,
) -> crate::Promise<Vec<FileDialogHandle>> {
    {
        return crate::host_value::<crate::Promise<Vec<FileDialogHandle>>>("host.resolve");
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:246 (sha256:b899e4b9db2555d6330dd9e3cdf86a011e79a2f0355b05f846f7c7204dbae732)
fn open_directory_picker_access_api(
    options: &OpenDirectoryDialogOptions,
) -> crate::Promise<Vec<FileDialogHandle>> {
    Default::default()
}

// Source: upstream/packages/dialog/src/dialog.ts:274 (sha256:15c02271442ed28205463fdfc4cc279e564000553cf6681a89cc05a22bd6e1e9)
fn open_web_file_dialog(options: OpenFileDialogOptions) -> crate::Promise<Vec<FileDialogHandle>> {
    {
        return crate::host_value::<crate::Promise<Vec<FileDialogHandle>>>("host.resolve");
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:319 (sha256:0a2092597843117736235fd619c2d4f395e5ff517d455c075d5948fee291c1cc)
fn open_file_system_access_picker(
    options: &OpenFileDialogOptions,
) -> crate::Promise<Vec<FileDialogHandle>> {
    Default::default()
}

// Source: upstream/packages/dialog/src/dialog.ts:349 (sha256:e88949abb7c69f2698087c43c21543a34eaf2427cac71f67fccf397a0a8a7156)
fn save_web_file(options: &SaveFileDialogOptions) -> crate::Promise<Option<FileDialogHandle>> {
    Default::default()
}

// Source: upstream/packages/dialog/src/dialog.ts:383 (sha256:a80bd1271ea368099ab2b27ff7778089cc04422829bd44b0c92288cc1e3f939d)
static _FILE_SYSTEM_DIRECTORY_HANDLE_REGISTRY: std::sync::LazyLock<
    std::sync::Mutex<Vec<(FileDialogHandle, crate::OpaqueHostValue)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/dialog/src/dialog.ts:389 (sha256:66ba729463a131d4e56282b3a70c2d836b04dade0f07b5703fee071202814123)
static _FILE_SYSTEM_HANDLE_REGISTRY: std::sync::LazyLock<
    std::sync::Mutex<Vec<(FileDialogHandle, crate::OpaqueHostValue)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/dialog/src/dialog.ts:392 (sha256:084fd6a03f215ebf161779571ca59d102edde07e56f57aa47cf0bb304f5856d5)
#[derive(Clone, Default)]
struct ShowWarningDialogSynthesizedRecord1450939776 {
    __flight_identity: std::sync::Arc<()>,
    buttons: Option<Vec<String>>,
    cancel_id: Option<f64>,
    checkbox_checked: Option<bool>,
    checkbox_label: Option<String>,
    default_id: Option<f64>,
    detail: Option<String>,
    kind: String,
    message: String,
    parent_window: Option<ApplicationWindow>,
    title: Option<String>,
}
impl PartialEq for ShowWarningDialogSynthesizedRecord1450939776 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn show_warning_dialog(options: &MessageDialogOptions) -> crate::Promise<MessageDialogResult> {
    return {
        let __flight_callback = (get_dialog_backend().message).clone();
        let __flight_result = __flight_callback.lock().unwrap()({
            let __flight_source = &({
                let __flight_spread_0 = options;
                ShowWarningDialogSynthesizedRecord1450939776 {
                    __flight_identity: std::sync::Arc::new(()),
                    buttons: (__flight_spread_0.buttons).clone(),
                    cancel_id: __flight_spread_0.cancel_id,
                    checkbox_checked: __flight_spread_0.checkbox_checked,
                    checkbox_label: (__flight_spread_0.checkbox_label).clone(),
                    default_id: __flight_spread_0.default_id,
                    detail: (__flight_spread_0.detail).clone(),
                    kind: "warning".to_owned(),
                    message: (__flight_spread_0.message).clone(),
                    parent_window: (__flight_spread_0.parent_window).clone(),
                    title: (__flight_spread_0.title).clone(),
                }
            });
            MessageDialogOptions {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                title: (__flight_source.title).clone(),
                message: (__flight_source.message).clone(),
                detail: (__flight_source.detail).clone(),
                buttons: (__flight_source.buttons).clone(),
                kind: Some((__flight_source.kind).clone()),
                checkbox_label: (__flight_source.checkbox_label).clone(),
                checkbox_checked: __flight_source.checkbox_checked,
                default_id: __flight_source.default_id,
                cancel_id: __flight_source.cancel_id,
                parent_window: (__flight_source.parent_window).clone(),
            }
        });
        __flight_result
    };
}

// Source: upstream/packages/dialog/src/dialog.ts:397 (sha256:aa9339bc7c25b90f762be7bada6fe0ea8735cff29662768fcfa43ad2e460826e)
#[derive(Clone)]
struct FileSystemFileHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: String,
    pub get_file: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>,
        >,
    >,
    pub create_writable: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>,
        >,
    >,
}
impl PartialEq for FileSystemFileHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:404 (sha256:b38c3877df24ab37c17f86b20b7e99072e70ee7af95c2191f101f67f28635ea0)
#[derive(Clone)]
struct FileSystemWritableFileStream {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub write: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        crate::FlightUnion2<crate::OpaqueHostValue, String>,
                    ) -> crate::Promise<crate::OpaqueHostValue>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub close: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::Promise<crate::OpaqueHostValue> + Send + 'static>,
        >,
    >,
}
impl PartialEq for FileSystemWritableFileStream {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:409 (sha256:5e67de81ebf6cd391be0a44fb7c9571e553cea5b611f165616fc20f364ad6e60)
#[derive(Clone, Default)]
struct FileSystemDirectoryHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: String,
}
impl PartialEq for FileSystemDirectoryHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:414 (sha256:6974f6afc26b04ed9b19e0f24679ca9bdfcf230390f099f275905ddb15404e7a)
#[derive(Clone, Default)]
struct FileSystemAccessOpenPickerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub exclude_accept_all_option: Option<bool>,
    pub multiple: Option<bool>,
    pub start_in: Option<String>,
    pub types: Option<Vec<SharedStructuralRecord1>>,
}
impl PartialEq for FileSystemAccessOpenPickerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:421 (sha256:3893efdad7e3e6c02684d95ea95f3acd3938e2f64f158fbef67d6bab0c767aff)
#[derive(Clone, Default)]
struct FileSystemAccessDirectoryPickerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: Option<String>,
    pub start_in: Option<String>,
}
impl PartialEq for FileSystemAccessDirectoryPickerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:426 (sha256:f9dd7b0dbd52ccc1e2499337ed52d0d70628649234999b381c33388a42e1ea53)
#[derive(Clone, Default)]
struct FileSystemAccessSavePickerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub exclude_accept_all_option: Option<bool>,
    pub start_in: Option<String>,
    pub suggested_name: Option<String>,
    pub types: Option<Vec<SharedStructuralRecord1>>,
}
impl PartialEq for FileSystemAccessSavePickerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/dialog/src/dialog.ts:433 (sha256:ef43aee4eb3ed18fc4d76852cc9d4dc0de89d83164f627ee99e6101f6cc3e8c7)
#[derive(Clone, Default)]
struct WindowWithFileSystemAccess {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub show_directory_picker: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            Option<crate::OpaqueHostValue>,
                        ) -> crate::Promise<crate::OpaqueHostValue>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub show_open_file_picker: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            Option<crate::OpaqueHostValue>,
                        )
                            -> crate::Promise<Vec<crate::OpaqueHostValue>>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub show_save_file_picker: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            Option<crate::OpaqueHostValue>,
                        ) -> crate::Promise<crate::OpaqueHostValue>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for WindowWithFileSystemAccess {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
