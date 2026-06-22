#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(const_result_trait_fn)]
#![feature(const_option_ops)]
#![feature(const_closures)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_cmp)]

pub mod tables;
pub mod types;

pub trait ByteRepr {
    type Owned: IntoByteRepr<Repr = Self>;
    fn to_owned(&self) -> Self::Owned;
}
pub trait IntoByteRepr {
    type Repr: ByteRepr<Owned = Self>;
    fn write_repr<'a>(&self, buf: &'a mut Vec<u8>) -> &'a Self::Repr;
}
