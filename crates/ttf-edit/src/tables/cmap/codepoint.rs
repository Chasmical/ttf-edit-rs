use crate::types::impl_fmt_from_getter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Codepoint(u32);

impl Codepoint {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl_fmt_from_getter! { Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for Codepoint }

impl From<Codepoint> for u32 {
    fn from(value: Codepoint) -> Self {
        value.get()
    }
}
impl From<Codepoint> for usize {
    fn from(value: Codepoint) -> Self {
        value.get() as _
    }
}

impl From<u32> for Codepoint {
    fn from(value: u32) -> Self {
        Codepoint::new(value)
    }
}
impl From<u16> for Codepoint {
    fn from(value: u16) -> Self {
        Codepoint::new(value as u32)
    }
}
impl From<u8> for Codepoint {
    fn from(value: u8) -> Self {
        Codepoint::new(value as u32)
    }
}
