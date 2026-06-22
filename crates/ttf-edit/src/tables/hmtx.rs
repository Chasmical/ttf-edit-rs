use crate::{
    tables::{TableDirectoryRepr, cmap::GlyphId, get_dyn_array, get_dyn_array_offset},
    types::{FWORD, Tag, UFWORD},
};
use std::{iter::FusedIterator, marker::PhantomData, ops::Range};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HmtxTableRepr {
    h_metrics: [LongHorMetricRepr; 0],
    left_side_bearings: [FWORD; 0],
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LongHorMetricRepr {
    advance_width: UFWORD,
    lsb: FWORD,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HmtxTableHandle<'a> {
    hmtx: &'a HmtxTableRepr,
    num_h_metrics: u16,
    num_glyphs: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LongHorMetric {
    advance_width: Option<u16>,
    lsb: i16,
}

impl<'a> HmtxTableHandle<'a> {
    pub const fn new(dir: &'a TableDirectoryRepr) -> Self {
        Self {
            hmtx: dir.find_table(Tag::hmtx).unwrap(),
            num_h_metrics: dir.hhea().number_of_h_metrics.get(),
            num_glyphs: dir.maxp().num_glyphs.get(),
        }
    }

    pub const fn h_metrics(&self) -> &[LongHorMetricRepr] {
        unsafe { get_dyn_array(&self.hmtx.h_metrics, self.num_h_metrics) }
    }
    pub const fn left_side_bearings(&self) -> &[FWORD] {
        unsafe {
            get_dyn_array_offset(
                &self.hmtx.left_side_bearings,
                self.num_h_metrics as usize * size_of::<LongHorMetricRepr>(),
                self.num_glyphs - self.num_h_metrics,
            )
        }
    }

    pub const fn metric(&self, glyph_id: GlyphId) -> Option<LongHorMetric> {
        let id = glyph_id.get() as usize;

        if let Some(repr) = self.h_metrics().get(id) {
            return Some(LongHorMetric {
                advance_width: Some(repr.advance_width.get()),
                lsb: repr.lsb.get(),
            });
        }
        if let Some(lsb) = self.left_side_bearings().get(id - self.num_h_metrics as usize) {
            let advance_width = self.h_metrics().last().map(const |x| x.advance_width.get());
            return Some(LongHorMetric { advance_width, lsb: lsb.get() });
        }
        None
    }
}

#[derive(Default, Debug, Clone)]
pub struct Iter<'a> {
    idx: Range<u16>,
    num_h_metrics: u16,
    start: *const LongHorMetricRepr,
    default_advance_width: Option<u16>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Iter<'a> {
    pub const fn new(h: &'a HmtxTableHandle) -> Self {
        let aw = h.h_metrics().last().map(const |x| x.advance_width.get());

        Self {
            start: unsafe { &*std::ptr::from_ref(&h.hmtx.h_metrics).cast() },
            num_h_metrics: h.num_h_metrics,
            idx: 1..h.num_glyphs,
            default_advance_width: aw,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (GlyphId, LongHorMetric);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx.next()?;

        let (advance_width, lsb) = unsafe {
            if idx < self.num_h_metrics {
                let repr = self.start.add(idx as usize).read();

                (Some(repr.advance_width.get()), repr.lsb.get())
            } else if idx < self.idx.end {
                let repr = self.start.cast::<FWORD>().add((self.num_h_metrics + idx) as usize);

                (self.default_advance_width, (*repr).get())
            } else {
                return None;
            }
        };

        Some((idx.into(), LongHorMetric { advance_width, lsb }))
    }
}
impl<'a> FusedIterator for Iter<'a> {}
