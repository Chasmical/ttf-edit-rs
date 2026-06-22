use crate::types::impl_fmt_from_getter;

macro_rules! impl_fixed_point_number {
    (
        $(#[$outer:meta])*
        $vis:vis struct $Name:ident($int:ty as $bytes:ty; $integer_bits:literal | $fraction_bits:literal);
    ) => {
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq)]
        #[repr(transparent)]
        $vis struct $Name($bytes);

        const _: () = {
            assert!(size_of::<$int>() == size_of::<$bytes>());
            assert!($integer_bits + $fraction_bits == <$int>::BITS);
        };

        impl $Name {
            const STEP: f32 = 1.0 / (1 << $fraction_bits) as f32;
            const MIN: f32 = -(1 << $integer_bits) as f32;
            const MAX: f32 = ((1 << $integer_bits) - 1) as f32;

            pub const fn new(num: f32) -> Self {
                assert!(matches!(num, Self::MIN..Self::MAX));
                let num = (num / Self::STEP).round() as $int;
                Self(num.to_be_bytes())
            }

            pub const fn to_be_bytes(&self) -> $bytes {
                self.0
            }
            pub const fn to_frac_num(&self) -> $int {
                <$int>::from_be_bytes(self.0)
            }

            pub const fn get(&self) -> f32 {
                self.to_frac_num() as f32 * Self::STEP
            }
        }

        impl_fmt_from_getter! { Debug, Display, LowerExp, UpperExp for $Name }

        const impl PartialOrd for $Name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        const impl Ord for $Name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.to_frac_num().cmp(&other.to_frac_num())
            }
        }

        const impl PartialEq<f32> for $Name {
            fn eq(&self, other: &f32) -> bool {
                self.get().eq(other)
            }
        }
        const impl PartialOrd<f32> for $Name {
            fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
                self.get().partial_cmp(other)
            }
        }

        impl std::str::FromStr for $Name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                f32::from_str(s).or(Err(())).map(Self::from)
            }
        }
        const impl From<f32> for $Name {
            fn from(value: f32) -> $Name {
                Self::new(value)
            }
        }
    }
}

impl_fixed_point_number! {
    pub struct Fixed(i32 as [u8; 4]; 16|16);
}
impl_fixed_point_number! {
    pub struct F2DOT14(i16 as [u8; 2]; 2|14);
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write tests for Fixed

    #[test]
    fn f2dot14() {
        let nums: [(f32, u16); _] = [
            (1.999939, 0x7FFF),
            (1.750000, 0x7000),
            (0.008118, 0x0085),
            (0.000122, 0x0002),
            (0.000061, 0x0001),
            (0.000000, 0x0000),
            (-0.000061, 0xFFFF),
            (-0.000122, 0xFFFE),
            (-0.008118, 0xFF7B),
            (-2.000000, 0x8000),
        ];

        for (fp, raw) in nums {
            let real = F2DOT14::new(fp).to_frac_num() as u16;
            assert_eq!(real, raw, "{real} != {raw} ({fp})");

            let real = F2DOT14::new(fp).get();
            let diff = (real - fp).abs();
            assert!(diff <= 0.1 * F2DOT14::STEP, "{real} != {fp} (Δ={diff})");
        }
    }
}
