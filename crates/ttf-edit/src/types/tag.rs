use std::fmt;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Tag([TagByte; 4]);

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum TagByte {
    Space = 0x20, ExclamationMark, QuotationMark, NumberSign, DollarSign, PercentSign, Ampersand,
    Apostrophe, LeftParenthesis, RightParenthesis, Asterisk, PlusSign, Comma, HyphenMinus, FullStop,
    Solidus, Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9, Colon,
    Semicolon, LessThanSign, EqualsSign, GreaterThanSign, QuestionMark, CommercialAt, CapitalA,
    CapitalB, CapitalC, CapitalD, CapitalE, CapitalF, CapitalG, CapitalH, CapitalI, CapitalJ,
    CapitalK, CapitalL, CapitalM, CapitalN, CapitalO, CapitalP, CapitalQ, CapitalR, CapitalS,
    CapitalT, CapitalU, CapitalV, CapitalW, CapitalX, CapitalY, CapitalZ, LeftSquareBracket,
    ReverseSolidus, RightSquareBracket, CircumflexAccent, LowLine, GraveAccent, SmallA, SmallB,
    SmallC, SmallD, SmallE, SmallF, SmallG, SmallH, SmallI, SmallJ, SmallK, SmallL, SmallM, SmallN,
    SmallO, SmallP, SmallQ, SmallR, SmallS, SmallT, SmallU, SmallV, SmallW, SmallX, SmallY, SmallZ,
    LeftCurlyBracket, VerticalLine, RightCurlyBracket, Tilde, // 0x20 ..= 0x7E
}

impl Tag {
    pub const fn from_raw(bytes: [u8; 4]) -> Result<Self, ()> {
        if matches!(bytes, [0x20..=0x7E, 0x20..=0x7E, 0x20..=0x7E, 0x20..=0x7E]) {
            Ok(Self(unsafe { std::mem::transmute::<[u8; 4], [TagByte; 4]>(bytes) }))
        } else {
            Err(())
        }
    }
    pub const fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        Self::from_raw(match *bytes {
            [a, b, c, d] => [a, b, c, d],
            [a, b, c] => [a, b, c, b' '],
            _ => return Err(()),
        })
    }
    pub const fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_bytes(s.as_bytes())
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        unsafe { std::mem::transmute(self.0) }
    }
    pub const fn as_bytes(&self) -> &[u8; 4] {
        unsafe { std::mem::transmute(&self.0) }
    }
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}

macro_rules! predefined_tags {
    ($($tag:ident),* $(,)?) => ($(
        pub const $tag: Self = Tag::from_str(stringify!($tag)).ok().unwrap();
    )*);
}

#[allow(non_upper_case_globals)]
impl Tag {
    predefined_tags! {
        avar, BASE, CBDT, CBLC, CFF, CFF2, cmap, COLR, CPAL, cvar, cvt, DSIG, EBDT, EBLC, EBSC,
        fpgm, fvar, gasp, GDEF, glyf, GPOS, GSUB, gvar, hdmx, head, hhea, hmtx, HVAR, JSTF, kern,
        loca, LTSH, MATH, maxp, MERG, meta, MVAR, name, PCLT, post, prep, sbix, STAT, SVG, VDMX,
        vhea, vmtx, VORG, VVAR,
    }
    pub const OS_2: Self = Tag::from_raw(*b"OS/2").ok().unwrap();
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

const impl PartialEq<str> for Tag {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
const impl PartialEq<[u8; 4]> for Tag {
    fn eq(&self, other: &[u8; 4]) -> bool {
        self.as_bytes() == other
    }
}
const impl PartialEq<&[u8; 4]> for Tag {
    fn eq(&self, other: &&[u8; 4]) -> bool {
        self.as_bytes() == *other
    }
}

const impl std::str::FromStr for Tag {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
const impl TryFrom<&[u8]> for Tag {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}
const impl TryFrom<[u8; 4]> for Tag {
    type Error = ();
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}
const impl TryFrom<&str> for Tag {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}
