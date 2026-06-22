use crate::types::{FWORD, UFWORD, int16, uint16};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HheaTableRepr {
    major_version: uint16,
    minor_version: uint16,
    ascender: FWORD,
    descender: FWORD,
    line_gap: FWORD,
    advance_width_max: UFWORD,
    min_left_side_bearing: FWORD,
    min_right_side_bearing: FWORD,
    x_max_extent: FWORD,
    caret_slope_rise: int16,
    caret_slope_run: int16,
    caret_offset: int16,
    reserved0: int16,
    reserved1: int16,
    reserved2: int16,
    reserved3: int16,
    metric_data_format: int16,
    pub(crate) number_of_h_metrics: uint16,
}
