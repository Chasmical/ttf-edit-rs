use crate::{
    bcow::{ByteRepr, ByteReprWriter, ReadByteRepr, WriteByteRepr},
    get_dyn_array, get_dyn_array_offset,
    tables::{TableDirectoryRepr, cmap::GlyphId},
    types::{FWORD, Tag, UFWORD},
};
use std::{iter::FusedIterator, marker::PhantomData, ops::Range};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HmtxTableRepr {
    h_metrics: [LongHorMetricRepr; 0],
    left_side_bearings: [FWORD; 0],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl ByteRepr for HmtxTableRepr {}

impl<'a> ReadByteRepr for HmtxTableHandle<'a> {
    type Owned = HmtxTable;
    fn read_to_owned(&self) -> Self::Owned {
        HmtxTable::new(self)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HmtxTable {
    metrics: Vec<LongHorMetric>,
}

impl HmtxTable {
    pub fn new(h: &HmtxTableHandle<'_>) -> Self {
        let mut this = Self::default();
        let mut last_aw = None;

        for metric in h.h_metrics() {
            last_aw = Some(metric.advance_width.get());
            this.metrics.push(LongHorMetric { advance_width: last_aw, lsb: metric.lsb.get() });
        }
        for lsb in h.left_side_bearings() {
            this.metrics.push(LongHorMetric { advance_width: last_aw, lsb: lsb.get() });
        }

        this
    }
    pub fn metrics(&self) -> &[LongHorMetric] {
        &self.metrics
    }
    pub fn metric(&self, glyph_id: GlyphId) -> Option<LongHorMetric> {
        self.metrics.get(glyph_id.get() as usize).copied()
    }
}

impl<'a> WriteByteRepr<'a> for HmtxTableHandle<'a> {
    type Repr = Self;

    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr {
        let hmtx_start_offset = w.len();

        Self {
            hmtx: unsafe { w.offset_cast(hmtx_start_offset) },
            num_h_metrics: self.num_h_metrics,
            num_glyphs: self.num_glyphs,
        }
    }
}

impl<'a> WriteByteRepr<'a> for HmtxTable {
    type Repr = HmtxTableHandle<'a>;

    fn write_to_repr(&self, w: &'a mut ByteReprWriter) -> Self::Repr {
        let same_last_aw_count = {
            let mut it = self.metrics.iter().rev();

            it.next()
                .map(|metric| metric.advance_width)
                .map_or(0, |last_aw| it.filter(|x| x.advance_width == last_aw).count())
        };

        let long_rec_count = self.metrics.len() - same_last_aw_count;

        let v = w.as_mut_vec();
        let hmtx_start_offset = v.len();
        let byte_len = (long_rec_count + self.metrics.len()) * size_of::<FWORD>();
        v.reserve(byte_len);

        unsafe {
            let dst: *mut LongHorMetricRepr = v.as_mut_ptr_range().end.cast();

            for i in 0..long_rec_count {
                let metric = self.metrics.get_unchecked(i);

                *dst.add(i) = LongHorMetricRepr {
                    advance_width: metric.advance_width.unwrap_or(0).into(),
                    lsb: metric.lsb.into(),
                };
            }

            let dst: *mut FWORD = dst.add(long_rec_count).cast();

            for i in long_rec_count..self.metrics.len() {
                *dst.add(i) = self.metrics.get_unchecked(i).lsb.into();
            }

            v.set_len(v.len() + byte_len);
        }

        HmtxTableHandle {
            hmtx: unsafe { w.offset_cast(hmtx_start_offset) },
            num_h_metrics: long_rec_count.try_into().unwrap(),
            num_glyphs: self.metrics.len().try_into().unwrap(),
        }
    }
}
