use crate::types::uint16;
use core::fmt;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Version16Dot16 {
    major: [u8; 2],
    minor: [u8; 2],
}

impl Version16Dot16 {
    pub const fn from_be_bytes(major: [u8; 2], minor: [u8; 2]) -> Self {
        Self { major, minor }
    }
    pub const fn new(major: u16, minor: u16) -> Self {
        Self::from_be_bytes(major.to_be_bytes(), minor.to_be_bytes())
    }

    pub const fn major(&self) -> u16 {
        u16::from_be_bytes(self.major)
    }
    pub const fn minor(&self) -> u16 {
        u16::from_be_bytes(self.minor)
    }
    pub const fn tuple(&self) -> (u16, u16) {
        (self.major(), self.minor())
    }
}

const impl From<(u16, u16)> for Version16Dot16 {
    fn from(value: (u16, u16)) -> Self {
        Self::new(value.0, value.1)
    }
}
const impl From<(uint16, uint16)> for Version16Dot16 {
    fn from(value: (uint16, uint16)) -> Self {
        Self::from_be_bytes(value.0.to_bytes(), value.1.to_bytes())
    }
}

impl fmt::Display for Version16Dot16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.major(), self.minor())
    }
}
impl fmt::Debug for Version16Dot16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
