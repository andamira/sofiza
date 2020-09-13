use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::result::Result;

use logos::Logos;

use crate::sfz::types::OpcodeMap;
use crate::sfz::{Group, Header, Opcode, Region, SfzToken};
use crate::Error;

/// Represents the SFZ instrument parsed
///
/// All units in the sfz format are in real-world values:
/// Frequencies are expressed in Hertz, pitches in cents,
/// amplitudes in percentage and volumes in decibels.
///
/// Notes are expressed in MIDI Note Numbers, or in note names according to the
/// International Pitch Notation (IPN) convention. According to this rules,
/// middle C in the keyboard is C4 and the MIDI note number 60.
///
#[derive(Debug)]
pub struct Instrument {
    global: OpcodeMap,    // these opcodes apply by default
    groups: Vec<Group>,   // these opcodes override those in global for current region
    regions: Vec<Region>, // these opcodes override global, and their group ones

    // maybe make this later a: struct Control
    // https://sfzformat.com/headers/control
    default_path: PathBuf,
}

impl Instrument {
    /// Creates an Instrument via loading and parsing some SFZ code in a file
    ///
    pub fn from_file(sfz_path: &Path) -> Result<Self, Error> {
        // open sfz file, and read it into sfz_text
        let mut sfz_file = File::open(&sfz_path)?;
        let mut sfz_text = String::new();
        sfz_file.read_to_string(&mut sfz_text)?;

        Self::from_sfz(&sfz_text, sfz_path.parent().unwrap())
    }

    /// Creates an Instrument via parsing some SFZ code in a string
    ///
    /// sfz_path would be the root location from where to find the samples
    /// and default_path opcode value is appended to it.
    ///
    pub fn from_sfz(sfz: &str, sfz_path: &Path) -> Result<Self, Error> {
        // Initializes an instrument for construction
        let mut instrument = Instrument {
            global: OpcodeMap::new(),
            groups: Vec::<Group>::new(),
            regions: Vec::<Region>::new(),
            default_path: sfz_path.to_path_buf(),
        };

        // parser loop status
        let mut status = InstrumentParsingStatus::init();

        // parser loop
        let lex = SfzToken::lexer(&sfz);
        for t in lex {
            match &t {
                SfzToken::Header(h) => {
                    match h {
                        Header::Group => {
                            status.new_group();
                            instrument.groups.push(Group::new());
                        }
                        Header::Region => {
                            status.new_region();

                            // FIXME: NOTE: an empty region (or without a sample) should be discarded
                            instrument.regions.push(Region::new());
                        }
                        Header::Control => {
                            status.new_control();
                        }
                        Header::Global => {
                            status.new_global();
                        }
                        // TBD
                        Header::Curve => println!("\n<curve>"),
                        // TBD
                        Header::Effect => println!("\n<effect>"),
                        _ => (),
                    }
                }

                SfzToken::Opcode(o) => {
                    // an opcode for <global>
                    if status.is_header_global {
                        instrument.global.insert(o.str_name(), o.clone());

                    // an opcode for <control>
                    } else if status.is_header_control {
                        match o {
                            Opcode::default_path(p) => instrument.default_path.push(p),
                            _ => (),
                        }
                    } else {
                        // an opcode for the <region>
                        if status.are_regions() {
                            instrument.add_region_opcode(&status, o);

                        // an opcode for the <group>
                        } else if status.are_groups() {
                            instrument.add_group_opcode(&status, o);
                        } else {
                            unreachable!();
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(instrument)
    }

    fn add_region_opcode(&mut self, status: &InstrumentParsingStatus, opcode: &Opcode) {
        self.regions[status.region_counter.unwrap()].add(opcode, status.group_counter);
    }

    fn add_group_opcode(&mut self, status: &InstrumentParsingStatus, opcode: &Opcode) {
        self.groups[status.group_counter.unwrap()].add(opcode);
    }
}

/// The current status of the parsing of the instrument
#[derive(Debug)]
struct InstrumentParsingStatus {
    is_header_control: bool,
    is_header_global: bool,
    // counts groups (first one is 0, valid as index)
    group_counter: Option<usize>,
    // counts regions inside a group (first one is 0, valid as index)
    region_counter: Option<usize>,
}
impl InstrumentParsingStatus {
    pub fn init() -> Self {
        Self {
            is_header_control: false,
            is_header_global: false,
            group_counter: None,
            region_counter: None,
        }
    }

    /// A new group header appears
    ///
    pub fn new_group(&mut self) {
        // ensure we are out of the <control> header
        self.is_header_control = false;
        // ensure we are out of the <global> header
        self.is_header_global = false;
        // ensure we reset the region counter for the current group
        self.region_reset();
        // increment the group counter
        self.group_increment();
    }

    /// A new region header appears
    ///
    pub fn new_region(&mut self) {
        // ensure we are out of the <control> header
        self.is_header_control = false;
        // ensure we are out of the <global> header
        self.is_header_global = false;
        // increment the region counter for the current group
        self.region_increment();
    }

    /// A new control header appears
    ///
    /// There can only be one, and must appear
    /// before the first global, group & region headers.
    ///
    // TODO: if incorrectly placed, following opcodes should be ignored
    pub fn new_control(&mut self) {
        if !self.is_header_control
            && !self.is_header_global
            && self.group_counter == None
            && self.region_counter == None
        {
            // enter the <control> header
            self.is_header_control = true;
        }
    }

    /// A new global header appears
    ///
    /// There can only be one, and must appear
    /// before the first group & region headers.
    ///
    // TODO: if incorrectly placed, following opcodes should be ignored
    pub fn new_global(&mut self) {
        if !self.is_header_global && self.group_counter == None && self.region_counter == None {
            // ensure we are out of the <control> header
            self.is_header_control = false;
            // enter the <global> header
            self.is_header_global = true;
        }
    }

    /// Increments the region counter
    fn region_increment(&mut self) {
        match self.region_counter {
            Some(c) => self.region_counter = Some(c + 1),
            None => self.region_counter = Some(0),
        }
    }
    fn region_reset(&mut self) {
        self.region_counter = None;
    }

    /// Increments the group counter
    fn group_increment(&mut self) {
        match self.region_counter {
            Some(c) => self.group_counter = Some(c + 1),
            None => self.group_counter = Some(0),
        }
    }

    /// Are there any groups already defined?
    pub fn are_groups(&self) -> bool {
        if self.group_counter == None {
            return false;
        }
        true
    }

    /// Are there any regions already defined for the current group?
    pub fn are_regions(&self) -> bool {
        if self.region_counter == None {
            return false;
        }
        true
    }
}
