use crate::types::{Version16Dot16, uint16};

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct MaxpTableRepr {
    // v0.5+
    version: Version16Dot16,
    pub(crate) num_glyphs: uint16,
    // v1.0+
    max_points: uint16,
    max_contours: uint16,
    max_composite_points: uint16,
    max_composite_contours: uint16,
    max_zones: uint16,
    max_twilight_points: uint16,
    max_storage: uint16,
    max_function_defs: uint16,
    max_instruction_defs: uint16,
    max_stack_elements: uint16,
    max_size_of_instructions: uint16,
    max_component_elements: uint16,
    max_component_depth: uint16,
}
