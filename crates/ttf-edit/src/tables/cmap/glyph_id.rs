use crate::types::impl_fmt_from_getter;
use std::num::NonZero;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GlyphId(NonZero<u32>);

impl GlyphId {
    pub const fn new(id: u32) -> Option<Self> {
        NonZero::new(id).map(GlyphId)
    }
    pub const unsafe fn new_unchecked(id: u32) -> Self {
        GlyphId(unsafe { NonZero::new_unchecked(id) })
    }
    pub const fn get_nonzero(self) -> NonZero<u32> {
        self.0
    }
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl_fmt_from_getter! { Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for GlyphId }

impl From<GlyphId> for u32 {
    fn from(value: GlyphId) -> Self {
        value.0.get()
    }
}
impl TryFrom<GlyphId> for u16 {
    type Error = ();
    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        value.0.get().try_into().map_err(|_| ())
    }
}
impl TryFrom<GlyphId> for u8 {
    type Error = ();
    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        value.0.get().try_into().map_err(|_| ())
    }
}
