use crate::types::{Fixed, LONGDATETIME, int16, uint16, uint32};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HeadTableRepr {
    major_version: uint16,
    minor_version: uint16,
    font_revision: Fixed,
    checksum_adjustment: uint32,
    magic_number: uint32,
    flags: uint16,
    units_per_em: uint16,
    created: LONGDATETIME,
    modified: LONGDATETIME,
    x_min: int16,
    y_min: int16,
    x_max: int16,
    y_max: int16,
    mac_style: uint16,
    lower_rec_ppem: uint16,
    font_direction_hint: int16,
    index_to_loc_format: int16,
    glyph_data_format: int16,
}
