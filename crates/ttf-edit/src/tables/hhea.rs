use crate::{
    bcow::{ByteRepr, ReadByteRepr},
    types::{FWORD, UFWORD, int16, uint16},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HheaTableRepr {
    pub major_version: uint16,
    pub minor_version: uint16,
    pub ascender: FWORD,
    pub descender: FWORD,
    pub line_gap: FWORD,
    pub advance_width_max: UFWORD,
    pub min_left_side_bearing: FWORD,
    pub min_right_side_bearing: FWORD,
    pub x_max_extent: FWORD,
    pub caret_slope_rise: int16,
    pub caret_slope_run: int16,
    pub caret_offset: int16,
    pub reserved0: int16,
    pub reserved1: int16,
    pub reserved2: int16,
    pub reserved3: int16,
    pub metric_data_format: int16,
    pub number_of_h_metrics: uint16,
}

const impl Default for HheaTableRepr {
    fn default() -> Self {
        Self {
            major_version: uint16::new(1),
            minor_version: uint16::new(0),
            reserved0: int16::new(0),
            reserved1: int16::new(0),
            reserved2: int16::new(0),
            reserved3: int16::new(0),
            metric_data_format: int16::new(0),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

impl ByteRepr for HheaTableRepr {}
impl ReadByteRepr for HheaTableRepr {
    type Owned = Self;
    fn read_to_owned(&self) -> Self::Owned {
        *self
    }
}
