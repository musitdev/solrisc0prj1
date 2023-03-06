use core::{cell::UnsafeCell, default::Default, mem::MaybeUninit, ptr, slice};
use risc0_zkvm::guest::env;
use serde::Deserialize;

pub(crate) struct Once<T> {
    data: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send + Sync> Sync for Once<T> {}

impl<T: Default> Once<T> {
    pub const fn new() -> Self {
        Once {
            data: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn init(&self, value: T) {
        unsafe { &mut *(self.data.get()) }.write(value);
    }

    pub fn get(&self) -> &mut T {
        unsafe {
            self.data
                .get()
                .as_mut()
                .unwrap_unchecked()
                .assume_init_mut()
        }
    }
}

#[derive(Default)]
pub struct ZkContext;
unsafe impl Sync for ZkContext {}

impl ZkContext {
    pub fn read<T: Deserialize<'static>>() -> T {
        env::read()
    }
}
