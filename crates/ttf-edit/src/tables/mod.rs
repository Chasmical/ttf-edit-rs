use crate::{
    tables::cmap::CmapTableRepr,
    types::{Offset32, Tag, uint16, uint24, uint32},
};

pub mod cmap;

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

    const fn find_table<TableRepr>(&self, tag: Tag) -> Option<&TableRepr> {
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

pub(crate) const unsafe fn get_dyn_array<T>(loc: &[T; 0], count: impl [const] IntoOffset) -> &[T] {
    unsafe { std::slice::from_raw_parts(loc.as_ptr(), count.offset()) }
}
pub(crate) const unsafe fn get_dyn_array_offset<T, Anchor>(
    anchor: &Anchor,
    offset: impl [const] IntoOffset,
    count: impl [const] IntoOffset,
) -> &[T] {
    unsafe {
        let array_start = std::ptr::from_ref(anchor).cast::<u8>().byte_add(offset.offset());
        std::slice::from_raw_parts(array_start.cast(), count.offset())
    }
}

pub(crate) const trait IntoOffset {
    fn offset(self) -> usize;
}

macro_rules! impl_into_offset {
    ($($t:ty),*) => ($(
        const impl IntoOffset for $t {
            fn offset(self) -> usize {
                self as _
            }
        }
    )*);
    (get: $($t:ty),*) => ($(
        const impl IntoOffset for $t {
            fn offset(self) -> usize {
                self.get() as _
            }
        }
    )*);
}

impl_into_offset! { u8, u16, u32, usize, i32 }
impl_into_offset! { get: uint16, uint24, uint32 }
