use crate::{
    get_dyn_array, get_dyn_array_offset,
    types::{Offset32, Tag, uint16, uint32},
};

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;

use {
    cmap::CmapTableRepr, head::HeadTableRepr, hhea::HheaTableRepr, hmtx::HmtxTableHandle,
    maxp::MaxpTableRepr,
};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TableDirectoryRepr {
    sfnt_version: uint32,
    num_tables: uint16,
    search_range: uint16,
    entry_selector: uint16,
    range_shift: uint16,
    table_records: [TableRecordRepr; 0],
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TableRecordRepr {
    table_tag: Tag,
    checksum: uint32,
    offset: Offset32,
    length: uint32,
}

impl TableDirectoryRepr {
    pub const unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        unsafe { &*bytes.as_ptr().cast() }
    }

    pub const fn tables(&self) -> &[TableRecordRepr] {
        unsafe { get_dyn_array(&self.table_records, self.num_tables) }
    }

    pub(crate) const fn find_table<TableRepr>(&self, tag: Tag) -> Option<&TableRepr> {
        let tables = self.tables();
        let mut idx = 0;
        while idx < tables.len() {
            let table = &tables[idx];
            if table.table_tag == tag {
                return Some(unsafe { table.data_cast(self) });
            }
            idx += 1;
        }
        None
    }

    pub const fn cmap(&self) -> &CmapTableRepr {
        self.find_table(Tag::cmap).unwrap()
    }
    pub const fn head(&self) -> &HeadTableRepr {
        self.find_table(Tag::head).unwrap()
    }
    pub const fn hhea(&self) -> &HheaTableRepr {
        self.find_table(Tag::hhea).unwrap()
    }
    pub const fn hmtx(&self) -> HmtxTableHandle<'_> {
        HmtxTableHandle::new(self)
    }
    pub const fn maxp(&self) -> &MaxpTableRepr {
        self.find_table(Tag::maxp).unwrap()
    }
}

impl TableRecordRepr {
    pub const fn tag(&self) -> Tag {
        self.table_tag
    }
    pub const fn checksum(&self) -> u32 {
        self.checksum.get()
    }
    pub const fn data<'a>(&self, dir: &'a TableDirectoryRepr) -> &'a [u8] {
        unsafe { get_dyn_array_offset(dir, self.offset, self.length) }
    }
    pub const unsafe fn data_cast<'a, T>(&self, dir: &'a TableDirectoryRepr) -> &'a T {
        unsafe { &*self.data(dir).as_ptr().cast() }
    }
}
