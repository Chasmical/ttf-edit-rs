use crate::{
    bcow::ByteRepr,
    types::{Version16Dot16, uint16},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct MaxpTableRepr {
    // v0.5+
    pub version: Version16Dot16,
    pub num_glyphs: uint16,
    // v1.0+
    pub max_points: uint16,
    pub max_contours: uint16,
    pub max_composite_points: uint16,
    pub max_composite_contours: uint16,
    pub max_zones: uint16,
    pub max_twilight_points: uint16,
    pub max_storage: uint16,
    pub max_function_defs: uint16,
    pub max_instruction_defs: uint16,
    pub max_stack_elements: uint16,
    pub max_size_of_instructions: uint16,
    pub max_component_elements: uint16,
    pub max_component_depth: uint16,
}

const impl Default for MaxpTableRepr {
    fn default() -> Self {
        Self { version: Version16Dot16::new(1, 0), ..unsafe { std::mem::zeroed() } }
    }
}

impl ByteRepr for MaxpTableRepr {
    type Owned = Box<Self>;
    fn read_to_owned(&self) -> Self::Owned {
        Box::new(*self)
    }
}
