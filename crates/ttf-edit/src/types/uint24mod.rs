use super::impl_fmt_from_getter;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct uint24([u8; 3]);

impl uint24 {
    pub const BITS: u32 = 24;
    pub const MIN: Self = Self::new(0x000000);
    pub const MAX: Self = Self::new(0xFFFFFF);

    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self(bytes)
    }
    pub const fn new(num: u32) -> Self {
        assert!(num <= 0xFFFFFF);
        let buf = num.to_be_bytes();
        Self(*buf.last_chunk::<3>().unwrap())
    }
    pub const fn get(&self) -> u32 {
        let mut buf = [0; 4];
        buf.last_chunk_mut::<3>().unwrap().copy_from_slice(&self.0);
        u32::from_be_bytes(buf)
    }
}

impl PartialOrd for uint24 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for uint24 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}

impl_fmt_from_getter! { Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for uint24 }
