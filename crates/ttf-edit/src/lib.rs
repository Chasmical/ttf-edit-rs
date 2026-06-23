#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(const_result_trait_fn)]
#![feature(const_option_ops)]
#![feature(const_closures)]
#![feature(const_destruct)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_clone)]
#![feature(const_index)]
#![feature(const_cmp)]

use crate::types::{uint16, uint24, uint32};

pub mod bcow;
pub mod tables;
pub mod types;

// TODO: figure out how to simplify these helper functions

pub(crate) const unsafe fn get_dyn_array<T>(loc: &[T; 0], count: impl [const] IntoOffset) -> &[T] {
    unsafe { std::slice::from_raw_parts(loc.as_ptr(), count.offset()) }
}
pub(crate) const unsafe fn get_dyn_array_offset<T, Anchor>(
    anchor: &Anchor,
    byte_offset: impl [const] IntoOffset,
    count: impl [const] IntoOffset,
) -> &[T] {
    unsafe {
        let array_start = std::ptr::from_ref(anchor).cast::<u8>().byte_add(byte_offset.offset());
        std::slice::from_raw_parts(array_start.cast(), count.offset())
    }
}

pub(crate) const trait IntoOffset {
    fn offset(self) -> usize;
}

macro_rules! impl_into_offset {
    ($($t:ty),*) => ($(
        const impl IntoOffset for $t {
            fn offset(self) -> usize {
                self as _
            }
        }
    )*);
    (get: $($t:ty),*) => ($(
        const impl IntoOffset for $t {
            fn offset(self) -> usize {
                self.get() as _
            }
        }
    )*);
}

impl_into_offset! { u8, u16, u32, usize, i32 }
impl_into_offset! { get: uint16, uint24, uint32 }
