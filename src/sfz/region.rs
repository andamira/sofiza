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
#[derive(Clone, Debug, Default)]
pub struct Region {
    /// The opcodes of this group are applied and will override the defaults.
    pub group: Option<usize>,

    /// This list of opcodes will override both the default and inherited opcodes.
    pub opcodes: OpcodeMap,
}

impl Region {
    /// New region.
    pub fn new() -> Self {
        Self::default()
    }

    /// New region with some group.
    // FIXME (add group at posteriori)
    pub fn with_group(group: usize) -> Self {
        Self {
            group: Some(group),
            opcodes: HashMap::new(),
        }
    }

    /// Add an opcode to this Region
    pub fn add_opcode(&mut self, opcode: &Opcode) {
        self.opcodes.insert(opcode.str_name(), opcode.clone());
    }

    /// Set the group of this Region
    pub fn set_group(&mut self, group: Option<usize>) {
        self.group = group;
    }

    /// Get the group of this Region
    pub fn group(&self) -> Option<usize> {
        self.group
    }
}
