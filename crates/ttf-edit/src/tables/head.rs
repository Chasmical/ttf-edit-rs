use crate::{
    bcow::ByteRepr,
    types::{Fixed, LONGDATETIME, int16, uint16, uint32},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HeadTableRepr {
    pub major_version: uint16,
    pub minor_version: uint16,
    pub font_revision: Fixed,
    pub checksum_adjustment: uint32,
    pub magic_number: uint32,
    pub flags: uint16,
    pub units_per_em: uint16,
    pub created: LONGDATETIME,
    pub modified: LONGDATETIME,
    pub x_min: int16,
    pub y_min: int16,
    pub x_max: int16,
    pub y_max: int16,
    pub mac_style: uint16,
    pub lower_rec_ppem: uint16,
    pub font_direction_hint: int16,
    pub index_to_loc_format: int16,
    pub glyph_data_format: int16,
}

const impl Default for HeadTableRepr {
    fn default() -> Self {
        Self {
            major_version: uint16::new(1),
            minor_version: uint16::new(0),
            magic_number: uint32::new(0x5F0F3CF5),
            font_direction_hint: int16::new(2),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

impl ByteRepr for HeadTableRepr {
    type Owned = Box<Self>;
    fn read_to_owned(&self) -> Self::Owned {
        Box::new(*self)
    }
}
