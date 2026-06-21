use zerocopy::network_endian::I64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LONGDATETIME {
    num: I64, // secs since 1904-01-01 0:00 UTC
}

// TODO: common impls for LONGDATETIME
