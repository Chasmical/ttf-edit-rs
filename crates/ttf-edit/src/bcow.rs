use std::ops::Deref;

pub trait ByteRepr {}
pub trait ReadByteRepr {
    type Owned;
    fn read_to_owned(&self) -> Self::Owned;
}
pub trait WriteByteRepr<'a> {
    type Repr;
    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr;
}

impl<T: ReadByteRepr> ReadByteRepr for &T {
    type Owned = T::Owned;
    fn read_to_owned(&self) -> Self::Owned {
        T::read_to_owned(*self)
    }
}
impl<'a, T: 'a + ByteRepr> WriteByteRepr<'a> for T {
    type Repr = &'a T;
    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr {
        w.write::<T>(self)
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub enum BCow<Ptr: ReadByteRepr> {
    Borrowed(Ptr),
    Owned(Ptr::Owned),
}

impl<Ptr: ReadByteRepr> BCow<Ptr> {
    pub const fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }
    pub const fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }

    pub fn to_mut(&mut self) -> &mut Ptr::Owned {
        match self {
            Self::Borrowed(borrowed) => {
                *self = Self::Owned(borrowed.read_to_owned());
                match *self {
                    Self::Borrowed(_) => unreachable!(),
                    Self::Owned(ref mut owned) => owned,
                }
            },
            Self::Owned(owned) => owned,
        }
    }

    pub fn into_owned(self) -> Ptr::Owned {
        match self {
            Self::Borrowed(b) => b.read_to_owned(),
            Self::Owned(o) => o,
        }
    }
}

const impl<T: ReadByteRepr<Owned = T>> Deref for BCow<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(b) => b,
            Self::Owned(o) => o,
        }
    }
}
impl<T: ReadByteRepr<Owned: Clone>> ReadByteRepr for BCow<T> {
    type Owned = T::Owned;
    fn read_to_owned(&self) -> T::Owned {
        match self {
            Self::Borrowed(b) => b.read_to_owned(),
            Self::Owned(o) => o.clone(),
        }
    }
}
impl<'a, T: WriteByteRepr<'a> + ReadByteRepr<Owned: WriteByteRepr<'a, Repr = T::Repr>>>
    WriteByteRepr<'a> for BCow<T>
{
    type Repr = T::Repr;
    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr {
        match self {
            Self::Borrowed(b) => b.write_to_repr(w),
            Self::Owned(o) => o.write_to_repr(w),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ByteReprWriter(Vec<u8>);

impl ByteReprWriter {
    pub const fn new(buf: Vec<u8>) -> Self {
        Self(buf)
    }
    pub const fn get(self) -> Vec<u8> {
        self.0
    }
    pub const fn len(&self) -> usize {
        self.0.len()
    }
    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }

    pub const unsafe fn offset_cast<T>(&self, offset: usize) -> &T {
        unsafe { &*self.0.as_ptr().add(offset).cast() }
    }

    pub fn write<'a, Repr: ByteRepr>(&'a mut self, repr: &Repr) -> &'a Repr {
        let repr_ptr = std::ptr::from_ref(repr).cast::<u8>();
        let bytes = unsafe { std::slice::from_raw_parts(repr_ptr, size_of::<Repr>()) };
        self.0.extend_from_slice(bytes);

        unsafe { &*self.0.as_ptr().add(self.0.len() - size_of::<Repr>()).cast() }
    }
}

#[macro_export]
macro_rules! bcow_match {
    ($expr:expr, |$arg:ident| $body:expr) => {
        match $expr {
            BCow::Borrowed($arg) => $body,
            BCow::Owned($arg) => $body,
        }
    };
}

pub use bcow_match;
