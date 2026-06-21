use crate::types::{Offset32, uint16};

mod codepoint;
mod glyph_id;

pub use codepoint::*;
pub use glyph_id::*;

pub mod format0;
pub mod format10;
pub mod format12;
pub mod format13;
pub mod format14;
pub mod format2;
pub mod format4;
pub mod format6;
pub mod format8;

use format0::*;
use format2::*;
use format4::*;
use format6::*;
use format8::*;
use format10::*;
use format12::*;
use format13::*;
use format14::*;

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CmapTableRepr {
    version: uint16,
    num_tables: uint16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EncodingRecordRepr {
    platform_id: uint16,
    encoding_id: uint16,
    subtable_offset: Offset32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CmapSubtableRepr {
    format: uint16,
    length: uint16,
    language: uint16,
}

impl CmapTableRepr {
    pub const fn encodings(&self) -> &[EncodingRecordRepr] {
        let records_start = unsafe { std::ptr::from_ref(self).byte_add(size_of::<Self>()) };
        unsafe { std::slice::from_raw_parts(records_start.cast(), self.num_tables.get() as _) }
    }
}
impl EncodingRecordRepr {
    pub const fn subtable(&self, cmap: &CmapTableRepr) -> &CmapSubtableRepr {
        unsafe { &*std::ptr::from_ref(cmap).byte_add(self.subtable_offset.get() as _).cast() }
    }
}
impl CmapSubtableRepr {
    pub const fn data(&self) -> &[u8] {
        let data_start = unsafe { std::ptr::from_ref(self).byte_add(size_of::<Self>()) };
        unsafe { std::slice::from_raw_parts(data_start.cast(), self.length.get() as _) }
    }
}

pub trait CmapSubtableTrait {
    type Iter: Iterator<Item = (Codepoint, GlyphId)>;
    fn iter(&self) -> Self::Iter;
    fn glyph_id(&self, codepoint: Codepoint) -> Option<GlyphId>;
    fn codepoint(&self, glyph_id: GlyphId) -> Option<Codepoint>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PlatformId(uint16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UnicodeEncodingId(uint16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MacintoshEncodingId(uint16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct IsoEncodingId(uint16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct WindowsEncodingId(uint16);

#[allow(non_upper_case_globals)]
impl PlatformId {
    pub const fn new(id: u16) -> Self {
        Self(uint16::new(id))
    }
    pub const fn get(self) -> u16 {
        self.0.get()
    }

    pub const Unicode: Self = Self::new(0);
    pub const Macintosh: Self = Self::new(1);
    pub const Iso: Self = Self::new(2);
    pub const Windows: Self = Self::new(3);
    pub const Custom: Self = Self::new(4);
}

#[allow(non_upper_case_globals)]
impl UnicodeEncodingId {
    pub const fn new(id: u16) -> Self {
        Self(uint16::new(id))
    }
    pub const fn get(self) -> u16 {
        self.0.get()
    }

    pub const Unicode1_0: Self = Self::new(0);
    pub const Unicode1_1: Self = Self::new(1);
    pub const IsoIec10646: Self = Self::new(2);
    pub const Unicode2_0BmpOnly: Self = Self::new(3);
    pub const Unicode2_0Full: Self = Self::new(4);
    pub const UnicodeVariations: Self = Self::new(5);
    pub const UnicodeFull: Self = Self::new(6);
}

#[allow(non_upper_case_globals)]
impl MacintoshEncodingId {
    pub const fn new(id: u16) -> Self {
        Self(uint16::new(id))
    }
    pub const fn get(self) -> u16 {
        self.0.get()
    }

    pub const Roman: Self = Self::new(0);
    pub const Japanese: Self = Self::new(1);
    pub const ChineseTraditional: Self = Self::new(2);
    pub const Korean: Self = Self::new(3);
    pub const Arabic: Self = Self::new(4);
    pub const Hebrew: Self = Self::new(5);
    pub const Greek: Self = Self::new(6);
    pub const Russian: Self = Self::new(7);
    pub const RSymbol: Self = Self::new(8);
    pub const Devanagari: Self = Self::new(9);
    pub const Gurmukhi: Self = Self::new(10);
    pub const Gujarati: Self = Self::new(11);
    pub const Odia: Self = Self::new(12);
    pub const Bangla: Self = Self::new(13);
    pub const Tamil: Self = Self::new(14);
    pub const Telugu: Self = Self::new(15);
    pub const Kannada: Self = Self::new(16);
    pub const Malayalam: Self = Self::new(17);
    pub const Sinhalese: Self = Self::new(18);
    pub const Burmese: Self = Self::new(19);
    pub const Khmer: Self = Self::new(20);
    pub const Thai: Self = Self::new(21);
    pub const Laotian: Self = Self::new(22);
    pub const Georgian: Self = Self::new(23);
    pub const Armenian: Self = Self::new(24);
    pub const ChineseSimplified: Self = Self::new(25);
    pub const Tibetan: Self = Self::new(26);
    pub const Mongolian: Self = Self::new(27);
    pub const Geez: Self = Self::new(28);
    pub const Slavic: Self = Self::new(29);
    pub const Vietnamese: Self = Self::new(30);
    pub const Sindhi: Self = Self::new(31);
    pub const Uninterpreted: Self = Self::new(32);
}

#[allow(non_upper_case_globals)]
impl IsoEncodingId {
    pub const fn new(id: u16) -> Self {
        Self(uint16::new(id))
    }
    pub const fn get(self) -> u16 {
        self.0.get()
    }

    pub const SevenBitAscii: Self = Self::new(0);
    pub const Iso10646: Self = Self::new(1);
    pub const Iso8859_1: Self = Self::new(2);
}

#[allow(non_upper_case_globals)]
impl WindowsEncodingId {
    pub const fn new(id: u16) -> Self {
        Self(uint16::new(id))
    }
    pub const fn get(self) -> u16 {
        self.0.get()
    }

    pub const Symbol: Self = Self::new(0);
    pub const UnicodeBmp: Self = Self::new(1);
    pub const ShiftJis: Self = Self::new(2);
    pub const Prc: Self = Self::new(3);
    pub const Big5: Self = Self::new(4);
    pub const Wansung: Self = Self::new(5);
    pub const Johab: Self = Self::new(6);
    pub const Reserved7: Self = Self::new(7);
    pub const Reserved8: Self = Self::new(8);
    pub const Reserved9: Self = Self::new(9);
    pub const UnicodeFull: Self = Self::new(10);
}
