use std::collections::HashMap;
use std::fmt::Debug;

use crate::sfz::{Opcode, OpcodeMap};

/// Groups allow entering common parameters for multiple regions.
///
/// A group is defined with the <group> opcode, and the parameters enumerated
/// on it last till the next group opcode, or till the end of the file.
///
#[derive(Debug)]
pub struct Group {
    opcodes: OpcodeMap, // these opcodes overwrites the defaults, and the inherited
    label: String,
}

impl Group {
    pub fn new() -> Self {
        Self {
            opcodes: HashMap::new(),
            label: String::new(),
        }
    }

    pub fn add_opcode(&mut self, o: &Opcode) {
        self.opcodes.insert(o.str_name(), o.clone());
    }
}
