use crate::{
    tables::cmap::CmapTableRepr,
    types::{Offset32, Tag, uint16, uint32},
};
use std::marker::Destruct;

pub mod cmap;

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TableDirectoryRepr {
    sfnt_version: uint32,
    num_tables: uint16,
    search_range: uint16,
    entry_selector: uint16,
    range_shift: uint16,
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
    pub const fn tables(&self) -> &[TableRecordRepr] {
        let records_start = unsafe { std::ptr::from_ref(self).byte_add(size_of::<Self>()) };
        unsafe { std::slice::from_raw_parts(records_start.cast(), self.num_tables.get() as _) }
    }

    const fn find_table<TableRepr, IntoTag>(&self, tag: IntoTag) -> Option<&TableRepr>
    where IntoTag: [const] TryInto<Tag, Error: [const] Destruct> {
        let tag: Tag = tag.try_into().ok()?;

        let tables = self.tables();
        let mut idx = 0;
        while idx < tables.len() {
            let table = &tables[idx];
            if table.table_tag == tag {
                return Some(unsafe { &*table.data(&self).as_ptr().cast::<TableRepr>() });
            }
            idx += 1;
        }
        None
    }

    pub const fn cmap(&self) -> &CmapTableRepr {
        self.find_table("cmap").unwrap()
    }
}

impl TableRecordRepr {
    pub const fn tag(&self) -> Tag {
        self.table_tag
    }
    pub const fn checksum(&self) -> u32 {
        self.checksum.get()
    }
    pub const fn data(&self, dir: &TableDirectoryRepr) -> &[u8] {
        unsafe {
            let start = std::ptr::from_ref(dir).byte_add(self.offset.get() as _);
            std::slice::from_raw_parts(start.cast(), self.length.get() as _)
        }
    }
}
