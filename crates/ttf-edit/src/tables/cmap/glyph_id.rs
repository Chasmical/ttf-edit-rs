use crate::types::impl_fmt_from_getter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GlyphId(u32);

impl GlyphId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl_fmt_from_getter! { Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for GlyphId }

const impl From<GlyphId> for usize {
    fn from(value: GlyphId) -> Self {
        value.0 as usize
    }
}
const impl From<GlyphId> for u32 {
    fn from(value: GlyphId) -> Self {
        value.0
    }
}
const impl TryFrom<GlyphId> for u16 {
    type Error = ();
    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        value.0.try_into().or(Err(()))
    }
}
const impl TryFrom<GlyphId> for u8 {
    type Error = ();
    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        value.0.try_into().or(Err(()))
    }
}

const impl From<u32> for GlyphId {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}
const impl From<u16> for GlyphId {
    fn from(value: u16) -> Self {
        Self::new(value as u32)
    }
}
const impl From<u8> for GlyphId {
    fn from(value: u8) -> Self {
        Self::new(value as u32)
    }
}
