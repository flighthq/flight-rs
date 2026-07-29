// @generated from upstream/packages/types/src/Signal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Signal.ts:3 (sha256:244c27c32b1b9d017992bedada49bbdd64b3fd34a9f53f0bbeb51d3600e5f0fb)
#[derive(Clone)]
pub struct Signal<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<SignalData<T>>,
    pub emit: T,
}
impl<T> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Signal.ts:8 (sha256:e88b26e0aa626a28c90fb4b79d03451b809f71842206c804f5b9e3c00859f81a)
pub struct SignalData<T> {
    #[doc(hidden)]
    pub inner: std::sync::Arc<std::sync::Mutex<SignalDataStorage<T>>>,
}
impl<T> Clone for SignalData<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
#[doc(hidden)]
pub struct SignalDataStorage<T> {
    pub slots: Vec<T>,
    pub priorities: Vec<f64>,
    pub repeat: Vec<bool>,
    pub cancelled: bool,
}
impl<T> SignalData<T> {
    pub fn new(slots: Vec<T>, priorities: Vec<f64>, repeat: Vec<bool>, cancelled: bool) -> Self {
        Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(SignalDataStorage {
                slots,
                priorities,
                repeat,
                cancelled,
            })),
        }
    }
}
