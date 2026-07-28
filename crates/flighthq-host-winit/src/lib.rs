#![forbid(unsafe_code)]

//! Cultivated native-host assembly over mechanically generated Flight backend seams.
//!
//! The crate is intentionally a compile canary before it owns a concrete `winit` event loop:
//! changes to any generated backend contract fail this crate immediately. Event translation and
//! `winit` ownership stay handwritten here while the package implementations remain generated.

use flighthq_types::{
    DeviceBackend, HapticsBackend, InputManager, LifecycleBackend, LoopBackend, PlatformBackend,
    PowerBackend, ScreenBackend, SoftKeyboardBackend, WindowBackend,
};

/// Complete native backend bundle installed before generated package APIs are used.
pub struct NativeHostBackends {
    pub device: DeviceBackend,
    pub haptics: HapticsBackend,
    pub lifecycle: LifecycleBackend,
    pub loop_backend: LoopBackend,
    pub platform: PlatformBackend,
    pub power: PowerBackend,
    pub screen: ScreenBackend,
    pub soft_keyboard: SoftKeyboardBackend,
    pub window: WindowBackend,
}

/// Installs the native backend bundle and creates the generated input state machine.
///
/// A concrete winit runner owns the returned manager and feeds translated window/device events
/// into its generated signals. Keeping installation in one function makes partial host setup
/// impossible at the cultivated boundary.
pub fn install_native_host(backends: NativeHostBackends) -> InputManager {
    flighthq_application::set_loop_backend(Some(backends.loop_backend));
    flighthq_application::set_window_backend(Some(backends.window));
    flighthq_device::set_device_backend(Some(backends.device));
    flighthq_haptics::set_haptics_backend(Some(backends.haptics));
    flighthq_keyboard::set_soft_keyboard_backend(Some(backends.soft_keyboard));
    flighthq_lifecycle::set_lifecycle_backend(Some(backends.lifecycle));
    flighthq_platform::set_platform_backend(Some(backends.platform));
    flighthq_power::set_power_backend(Some(backends.power));
    flighthq_screen::set_screen_backend(Some(backends.screen));
    flighthq_input::create_input_manager()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_native_seams_are_linked() {
        let _: fn(Option<LoopBackend>) = flighthq_application::set_loop_backend;
        let _: fn(Option<WindowBackend>) = flighthq_application::set_window_backend;
        let _: fn(Option<DeviceBackend>) = flighthq_device::set_device_backend;
        let _: fn(Option<HapticsBackend>) = flighthq_haptics::set_haptics_backend;
        let _: fn() -> InputManager = flighthq_input::create_input_manager;
        let _: fn(Option<SoftKeyboardBackend>) = flighthq_keyboard::set_soft_keyboard_backend;
        let _: fn(Option<LifecycleBackend>) = flighthq_lifecycle::set_lifecycle_backend;
        let _: fn(Option<PlatformBackend>) = flighthq_platform::set_platform_backend;
        let _: fn(Option<PowerBackend>) = flighthq_power::set_power_backend;
        let _: fn(Option<ScreenBackend>) = flighthq_screen::set_screen_backend;
    }
}
