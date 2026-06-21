#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct F2DOT14 {
    num: [u8; 2],
}

impl F2DOT14 {
    pub const fn new(num: f32) -> Self {
        assert!(matches!(num, -2.0..2.0));

        let integer = num.floor();
        let fraction = ((num - integer) * (1.0 / 16384.0)).round() as u16;

        let mut buf = fraction.to_be_bytes();
        buf[0] |= (integer as i8 as u8) << 6;
        Self { num: buf }
    }
    pub const fn get(&self) -> f32 {
        let integer = self.num[0] >> 6;
        let fraction = (((self.num[0] & 0x3F) as u16) << 8) | self.num[1] as u16;
        integer as f32 + fraction as f32 * (1.0 / 16384.0)
    }
}

// TODO: common impls for F2DOT14
