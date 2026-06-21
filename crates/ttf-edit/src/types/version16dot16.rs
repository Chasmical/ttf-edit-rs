use crate::types::uint16;

#[repr(C, packed)]
pub struct Version16Dot16 {
    major: uint16,
    minor: uint16,
}

// TODO: common impls for Version16Dot16
