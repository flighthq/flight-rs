// @generated from upstream/packages/shell/src/shell.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    ShellBackend, ShellOpenExternalOptions, ShellOpenPathOptions, ShellShortcutLink,
    ShellShortcutWriteOperation,
};

// Source: upstream/packages/shell/src/shell.ts:12 (sha256:5a57e250310062ed7ff58b0faafb6c297a5e55103784e3ac9fedf3541bf264dd)
pub fn create_web_shell_backend() -> ShellBackend {
    return ShellBackend {
        __flight_identity: std::sync::Arc::new(()),
        beep: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
        )),
        move_items_to_trash: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: Vec<String>| -> crate::Promise<crate::OpaqueHostValue> {
                return vec![];
            },
        )
            as Box<dyn FnMut(Vec<String>) -> crate::Promise<Vec<bool>> + Send + 'static>)),
        move_to_trash: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> crate::Promise<crate::OpaqueHostValue> {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>)),
        open_external: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |url: String,
                  __flight_unused_1: Option<ShellOpenExternalOptions>|
                  -> crate::Promise<crate::OpaqueHostValue> {
                return false;
            },
        )
            as Box<
                dyn FnMut(String, Option<ShellOpenExternalOptions>) -> crate::Promise<bool>
                    + Send
                    + 'static,
            >)),
        open_path: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String,
                  __flight_unused_1: Option<ShellOpenPathOptions>|
                  -> crate::Promise<crate::OpaqueHostValue> {
                return false;
            },
        )
            as Box<
                dyn FnMut(String, Option<ShellOpenPathOptions>) -> crate::Promise<bool>
                    + Send
                    + 'static,
            >)),
        open_path_result: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String,
                  __flight_unused_1: Option<ShellOpenPathOptions>|
                  -> crate::Promise<crate::OpaqueHostValue> {
                return "unavailable on web";
            },
        )
            as Box<
                dyn FnMut(String, Option<ShellOpenPathOptions>) -> crate::Promise<String>
                    + Send
                    + 'static,
            >)),
        read_shortcut_link: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> crate::Promise<crate::OpaqueHostValue> {
                return None;
            },
        )
            as Box<
                dyn FnMut(String) -> crate::Promise<Option<ShellShortcutLink>> + Send + 'static,
            >)),
        show_item_in_folder: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String| -> crate::Promise<crate::OpaqueHostValue> {
                return false;
            },
        )
            as Box<dyn FnMut(String) -> crate::Promise<bool> + Send + 'static>)),
        write_shortcut_link: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: String,
                  __flight_unused_1: ShellShortcutLink,
                  __flight_unused_2: Option<ShellShortcutWriteOperation>|
                  -> crate::Promise<crate::OpaqueHostValue> {
                return false;
            },
        )
            as Box<
                dyn FnMut(
                        String,
                        ShellShortcutLink,
                        Option<ShellShortcutWriteOperation>,
                    ) -> crate::Promise<bool>
                    + Send
                    + 'static,
            >)),
    };
}

// Source: upstream/packages/shell/src/shell.ts:59 (sha256:07fbf8fa17c1a31b7a3ed990901e037931bc533345ed289fb0fdfa4778c2b28e)
pub fn get_shell_backend() -> ShellBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_shell_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/shell/src/shell.ts:66 (sha256:10d7f9fdb91ea8b4223cbbac2c018be5e533a9145b75aac75bcc8e6c100dfdc3)
pub fn is_shell_url_allowed(url: String) -> bool {
    if ((*_URL_SCHEME_ALLOWLIST.lock().unwrap()).clone()).is_none() {
        return true;
    }
    let __flight_try_return: Option<bool> =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<bool> {
            {
                let scheme = crate::host_value::<()>("host.replace");
                return Some({
                    let __flight_value = scheme;
                    ((_URL_SCHEME_ALLOWLIST.as_ref().unwrap()).clone())
                        .iter()
                        .any(|item| item == &__flight_value)
                });
            }
            None
        })) {
            Ok(value) => value,
            Err(_) => (|| -> Option<bool> {
                {
                    return Some(false);
                }
                None
            })(),
        };
    return __flight_try_return.expect("TypeScript try/catch completed without returning");
}

// Source: upstream/packages/shell/src/shell.ts:78 (sha256:6cfa43607d30b529ea4d4e6ea091fb9aea6f743059ef94c16b5fc7b98e570b29)
pub fn move_items_to_trash(paths: &Vec<String>) -> crate::Promise<Vec<bool>> {
    return {
        let __flight_callback = (get_shell_backend().move_items_to_trash).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*paths).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:83 (sha256:c96f2d915899ea210326c7b2b116dc8e199d283cda0d497923c7bf59e03ec7af)
pub fn move_item_to_trash(path: String) -> crate::Promise<bool> {
    return {
        let __flight_callback = (get_shell_backend().move_to_trash).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:93 (sha256:3ee54d030bb5f8c558c66cf678def3333b232f303d45c4492074f4bbdc8d17cf)
pub fn open_shell_external_url(
    url: String,
    options: Option<ShellOpenExternalOptions>,
) -> crate::Promise<bool> {
    if (!is_shell_url_allowed((url).clone())) {
        return crate::host_value::<crate::Promise<bool>>("host.resolve");
    }
    return {
        let __flight_callback = (get_shell_backend().open_external).clone();
        let __flight_result = __flight_callback.lock().unwrap()((url).clone(), (options).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:99 (sha256:e62da09b822219708b62c8abb0bf1a8112b14d65a1471c6754a63e93b7d42c14)
pub fn open_shell_path(
    path: String,
    options: Option<ShellOpenPathOptions>,
) -> crate::Promise<bool> {
    return {
        let __flight_callback = (get_shell_backend().open_path).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone(), (options).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:106 (sha256:c0d5271895a1a17bba202bba04583f408bbf666b60864d7848cb5e3e1fc2b4ec)
pub fn open_shell_path_result(
    path: String,
    options: Option<ShellOpenPathOptions>,
) -> crate::Promise<String> {
    return {
        let __flight_callback = (get_shell_backend().open_path_result).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone(), (options).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:112 (sha256:be9612b4caa0a90b6da6624eaa47168bf9f6437047b90da92c882e96a0b0acde)
pub fn read_shell_shortcut_link(
    shortcut_path: String,
) -> crate::Promise<Option<ShellShortcutLink>> {
    return {
        let __flight_callback = (get_shell_backend().read_shortcut_link).clone();
        let __flight_result = __flight_callback.lock().unwrap()((shortcut_path).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:117 (sha256:96446a6a731d6b397c350a77ece65dc6738bd3b21162e2b677d779f35c07361e)
pub fn set_shell_backend(backend: Option<ShellBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/shell/src/shell.ts:125 (sha256:7d3df88bd601f3362350ecdaa199259b14cb36d1561a1ef467600f7a58250fc3)
pub fn set_shell_url_scheme_allowlist(schemes: Option<Vec<String>>) -> () {
    (*_URL_SCHEME_ALLOWLIST.lock().unwrap()) = (schemes).clone();
}

// Source: upstream/packages/shell/src/shell.ts:130 (sha256:723cffbfdb4f8ab269463ca47385a7f7efb4fc7bd7904ad8bebcefa3d63e7730)
pub fn shell_beep() -> () {
    {
        let __flight_callback = (get_shell_backend().beep).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:135 (sha256:10f61b7bc8d2c67bb22ecf92eb71f0fd8bbf504aaa92f97271c9ee4680140659)
pub fn show_item_in_folder(path: String) -> crate::Promise<bool> {
    return {
        let __flight_callback = (get_shell_backend().show_item_in_folder).clone();
        let __flight_result = __flight_callback.lock().unwrap()((path).clone());
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:143 (sha256:27c9563b487981203616e5154679deb28ed1d728805982d08c814f242453ce51)
pub fn write_shell_shortcut_link(
    shortcut_path: String,
    link: &ShellShortcutLink,
    operation: Option<ShellShortcutWriteOperation>,
) -> crate::Promise<bool> {
    return {
        let __flight_callback = (get_shell_backend().write_shortcut_link).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (shortcut_path).clone(),
            (*link).clone(),
            (operation).clone(),
        );
        __flight_result
    };
}

// Source: upstream/packages/shell/src/shell.ts:151 (sha256:1c4714da5fdfc642c73158c34b1ca4892ffeb24b123670feb2ac631d5d77c537)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<ShellBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/shell/src/shell.ts:152 (sha256:a934f6cce8fcf2db4cf506c0ffaff591959fe574791e218798e2a6eb98a6c22e)
static _URL_SCHEME_ALLOWLIST: std::sync::LazyLock<std::sync::Mutex<Option<Vec<String>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
