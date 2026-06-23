use std::ops::Deref;

pub trait ByteRepr {
    type Owned: for<'a> WriteByteRepr<'a>;
    fn read_to_owned(&self) -> Self::Owned;
}
pub trait WriteByteRepr<'a> {
    type Repr;
    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr;
}

impl<T: ByteRepr> ByteRepr for Box<T> {
    type Owned = T::Owned;
    fn read_to_owned(&self) -> Self::Owned {
        T::read_to_owned(self.as_ref())
    }
}
impl<'a, T: ByteRepr + 'a> WriteByteRepr<'a> for T {
    type Repr = &'a T;
    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr {
        w.write(self)
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

pub enum BCow<'a, Repr: ByteRepr> {
    Borrowed(&'a Repr),
    Owned(Repr::Owned),
}

impl<'a, T: ByteRepr> BCow<'a, T> {
    pub const fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }
    pub const fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }

    pub fn to_mut(&mut self) -> &mut T::Owned {
        match *self {
            Self::Borrowed(borrowed) => {
                *self = Self::Owned(borrowed.read_to_owned());
                match *self {
                    Self::Borrowed(..) => unreachable!(),
                    Self::Owned(ref mut owned) => owned,
                }
            },
            Self::Owned(ref mut owned) => owned,
        }
    }

    pub fn into_owned(self) -> T::Owned {
        match self {
            Self::Borrowed(b) => b.read_to_owned(),
            Self::Owned(o) => o,
        }
    }
}

const impl<'a, T: ByteRepr<Owned: [const] Deref<Target = T>>> Deref for BCow<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Borrowed(b) => b,
            Self::Owned(ref o) => o.deref(),
        }
    }
}

const impl<'a, T: ByteRepr<Owned: [const] Clone>> Clone for BCow<'a, T> {
    fn clone(&self) -> Self {
        match *self {
            Self::Borrowed(b) => Self::Borrowed(b),
            Self::Owned(ref o) => Self::Owned(o.clone()),
        }
    }
}

impl<'b: 'a, 'a, T: ByteRepr + 'a> WriteByteRepr<'a> for BCow<'b, T>
where T::Owned: WriteByteRepr<'a, Repr = &'a T>
{
    type Repr = &'a T;
    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr {
        match *self {
            Self::Borrowed(b) => b.write_to_repr(w),
            Self::Owned(ref o) => o.write_to_repr(w),
        }
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
