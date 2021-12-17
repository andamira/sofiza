use std::fmt::Debug;

use crate::sfz::{Opcode, OpcodeMap};

/// Groups allow entering common parameters for multiple regions.
///
/// A group is defined with the <group> opcode, and the parameters enumerated
/// on it last till the next group opcode, or till the end of the file.
///
#[derive(Debug, Default)]
pub struct Group {
    /// This list of opcodes overwrites the default ones.
    pub opcodes: OpcodeMap,

    /// The label of this group.
    pub label: String,
}

impl Group {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_opcode(&mut self, o: &Opcode) {
        self.opcodes.insert(o.str_name(), o.clone());
    }
}
