// @generated from upstream/packages/ipc/src/ipc.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    HostIpcHandleCapability, HostIpcInvokeCapability, HostIpcMessageCapability,
    HostIpcSendCapability, HostIpcTargetedSendCapability,
};

// Source: upstream/packages/ipc/src/ipc.ts:13 (sha256:dd01f4ab92651047bbb54634d9a9a3ea157b21d100b2d02988a0dcce7caf106d)
pub fn invoke_ipc(
    host_ipc_invoke: &HostIpcInvokeCapability,
    channel: String,
    args: Vec<crate::FlightValue>,
) -> crate::FlightTask<crate::FlightValue> {
    return {
        let __flight_callback = (host_ipc_invoke.invoke).clone();
        let __flight_result = __flight_callback.lock().unwrap()((channel).clone(), (args).clone());
        __flight_result
    };
}

// Source: upstream/packages/ipc/src/ipc.ts:26 (sha256:e59edb7d4a7a4742e8e1f00bea0cc4576466318ca4d96dca6993182443af22a0)
pub fn once_ipc_message(
    host_ipc_message: &HostIpcMessageCapability,
    channel: String,
    listener: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>>,
    >,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    let unsubscribe: std::sync::Arc<
        std::sync::Mutex<
            Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(None));
    let done: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut release: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut done = done.clone();
            let mut unsubscribe = unsubscribe.clone();
            move || -> () {
                if (*done.lock().unwrap()).clone() {
                    return;
                }
                (*done.lock().unwrap()) = true;
                {
                    let __flight_callback = (*unsubscribe.lock().unwrap()).clone();
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()())
                };
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    (*unsubscribe.lock().unwrap()) = Some({
        let __flight_callback = (host_ipc_message.subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (channel).clone(),
            std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let mut done = done.clone();
                let listener = listener.clone();
                let release = release.clone();
                move |args: Vec<crate::FlightValue>| -> () {
                    if (*done.lock().unwrap()).clone() {
                        return;
                    }
                    {
                        let __flight_callback = (release).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    };
                    {
                        let __flight_callback = (listener).clone();
                        let __flight_result = __flight_callback.lock().unwrap()((args).clone());
                        __flight_result
                    };
                }
            })
                as Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>)),
        );
        __flight_result
    });
    if (*done.lock().unwrap()).clone() {
        (*unsubscribe.lock().unwrap())
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()();
    }
    return release;
}

// Source: upstream/packages/ipc/src/ipc.ts:50 (sha256:81e3cecb5c12e0c7a0dbe6ffb92669dc880c77828de696536c927849aba2f309)
pub fn on_ipc_invoke(
    host_ipc_handle: &HostIpcHandleCapability,
    channel: String,
    handler: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Vec<crate::FlightValue>,
                    ) -> crate::FlightUnion2<
                        crate::FlightValue,
                        crate::FlightTask<crate::FlightValue>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    return {
        let __flight_callback = (host_ipc_handle.handle).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((channel).clone(), (handler).clone());
        __flight_result
    };
}

// Source: upstream/packages/ipc/src/ipc.ts:60 (sha256:ca7e39a2eb2f30e76d272e0b21d00c47f0aa5e94876e9429a51ea6fa4fc288de)
pub fn on_ipc_message(
    host_ipc_message: &HostIpcMessageCapability,
    channel: String,
    listener: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>>,
    >,
) -> std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> {
    return {
        let __flight_callback = (host_ipc_message.subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (channel).clone(),
            std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let listener = listener.clone();
                move |args: Vec<crate::FlightValue>| -> () {
                    {
                        let __flight_callback = (listener).clone();
                        let __flight_result = __flight_callback.lock().unwrap()((args).clone());
                        __flight_result
                    }
                }
            })
                as Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>)),
        );
        __flight_result
    };
}

// Source: upstream/packages/ipc/src/ipc.ts:68 (sha256:065683ebe54502a503d8bfd5014ab5fb8b1e6f4c7f946227568752bebddc138a)
pub fn send_ipc_message(
    host_ipc_send: &HostIpcSendCapability,
    channel: String,
    args: Vec<crate::FlightValue>,
) -> () {
    {
        let __flight_callback = (host_ipc_send.send).clone();
        let __flight_result = __flight_callback.lock().unwrap()((channel).clone(), (args).clone());
        __flight_result
    };
}

// Source: upstream/packages/ipc/src/ipc.ts:76 (sha256:e5c03c25e1fa393c504d09051e5d490b3b604ff17b38b85d2f24aa99a00bc59f)
pub fn send_ipc_message_to<Target: Clone>(
    host_ipc_targeted_send: &HostIpcTargetedSendCapability<Target>,
    target: NoInfer<Target>,
    channel: String,
    args: Vec<crate::FlightValue>,
) -> () {
    {
        let __flight_callback = (host_ipc_targeted_send.send).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()(target, (channel).clone(), (args).clone());
        __flight_result
    };
}
