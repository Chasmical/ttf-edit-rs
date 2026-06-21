#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Fixed {
    num: [u8; 4],
}

// TODO: common impls for Fixed
