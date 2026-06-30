use crate::{
    bcow::{ByteRepr, ReadByteRepr},
    tables::cmap::{CmapSubtableTrait, Codepoint, GlyphId},
};
use std::iter::{Enumerate, FusedIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CmapSubtableFormat0 {
    glyph_id_array: [u8; 256],
}

const impl Default for CmapSubtableFormat0 {
    fn default() -> Self {
        Self { glyph_id_array: [0; 256] }
    }
}

impl ByteRepr for CmapSubtableFormat0 {}
impl ReadByteRepr for CmapSubtableFormat0 {
    type Owned = Self;
    fn read_to_owned(&self) -> Self::Owned {
        *self
    }
}

impl<'a> CmapSubtableTrait for &'a CmapSubtableFormat0 {
    type Iter = Iter<'a>;
    fn iter(&self) -> Self::Iter {
        Iter(self.glyph_id_array.iter().enumerate())
    }
    fn glyph_id(&self, codepoint: Codepoint) -> Option<GlyphId> {
        self.glyph_id_array.get(codepoint.get() as usize).map(|id| GlyphId::new(*id as u32))
    }
    fn codepoint(&self, glyph_id: GlyphId) -> Option<Codepoint> {
        let glyph_id: u8 = glyph_id.try_into().ok()?;
        self.glyph_id_array.iter().position(|id| *id == glyph_id).map(|idx| (idx as u32).into())
    }
}

#[derive(Default, Debug, Clone)]
pub struct Iter<'a>(Enumerate<std::slice::Iter<'a, u8>>);

impl<'a> Iterator for Iter<'a> {
    type Item = (Codepoint, GlyphId);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.find_map(|(idx, &id)| {
            (id != 0).then(|| ((idx as u32).into(), GlyphId::new(id as u32)))
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.0.size_hint().1)
    }
}
impl<'a> FusedIterator for Iter<'a> {}
