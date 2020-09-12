use std::collections::HashMap;
use std::fmt::Debug;

use crate::sfz::{Opcode, OpcodeMap};

/// A Region is the basic component of an instrument.
/// An instrument is defined by one or more regions.
///
/// A region starts with the <region> header, and finishes with
/// the next <region> header, <group> header, or the end of the file.
///
/// Following the <region> header one or more opcodes can be defined.
/// The opcodes are special keywords which instruct the player on what,
/// when and how to play a sample.
///
/// Opcodes within a region can appear in any order, and they have to be
/// separated by one or more spaces or tabulation controls.
/// Opcodes can appear in separated lines within a region.
///
/// All Input Controls defined in a region act using the AND boolean operator.
/// Consequently, all conditions must be matched for the region to play.
///
#[derive(Debug)]
pub struct Region {
    group: Option<usize>,   // inherits the opcodes in this group
    opcodes: OpcodeMap,     // these opcodes overwrites the defaults, and the inherited
}

impl Region {
    pub fn new() -> Self {
        Self {
            group: None,
            opcodes: HashMap::new(),
        }
    }

    // FIXME (add group at posteriori)
    pub fn with_group(group: usize) -> Self {
        Self {
            group: Some(group),
            opcodes: HashMap::new(),
        }
    }

    pub fn add(&mut self, o: &Opcode, group: Option<usize>) {
        self.opcodes.insert(o.str_name(), o.clone());
        self.group = group;
    }
}

