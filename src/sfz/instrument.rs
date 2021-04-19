use std::{
    fmt::Debug,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

use logos::Logos;

use crate::{
    error::{Error, Result},
    sfz::{types::OpcodeMap, Group, Header, Opcode, Region, SfzToken},
};

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
    /// The default opcodes for this instrument.
    pub global: OpcodeMap,

    /// The list of groups.
    ///
    /// The opcodes in a group overrides those in global, for the associated region.
    pub groups: Vec<Group>,

    /// The list of regions.
    ///
    /// The opcodes in a region overrides those in global and in its group.
    pub regions: Vec<Region>, // these opcodes override global, and their group ones

    /// The default path.
    // maybe make this later a: struct Control
    // https://sfzformat.com/headers/control
    pub default_path: PathBuf,

    last_header_created: Header,
}

// constructors:
// - new
// - from_file
// - from_sfz
//
// - add_opcode
// - add_opcode_global
// - add_opcode_to_group
// - add_opcode_to_region
// - groups
// - regions
// - regions_in
// - new_group
// - new_region
// - set_region_group
//
impl Instrument {
    /// Creates an empty Instrument
    ///
    pub fn new() -> Instrument {
        Instrument {
            global: OpcodeMap::new(),
            groups: Vec::<Group>::new(),
            regions: Vec::<Region>::new(),
            default_path: PathBuf::new(),
            last_header_created: Header::Global,
        }
    }

    /// Creates an Instrument via loading and parsing some SFZ code in a file
    ///
    pub fn from_file(sfz_path: &Path) -> Result<Self> {
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
    pub fn from_sfz(sfz: &str, sfz_path: &Path) -> Result<Self> {
        #[cfg(debug)]
        println!("DEBUG: Instrument::from_sfz()\n-----------------------------");

        // Initializes an instrument for construction
        let mut instrument = Instrument {
            global: OpcodeMap::new(),
            groups: Vec::<Group>::new(),
            regions: Vec::<Region>::new(),
            default_path: sfz_path.to_path_buf(),
            last_header_created: Header::Global, // not used in this constructor
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
                            #[cfg(debug)]
                            println!("\nFound a new group:");

                            status.new_group();
                            instrument.groups.push(Group::new());
                        }
                        Header::Region => {
                            #[cfg(debug)]
                            println!("\nFound a new region:");

                            status.new_region();

                            // FIXME: SPEC: discard an empty region (or without a sample)
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
                        #[cfg(debug)]
                        println!("global OP {:?}", o);

                        instrument.add_opcode_global(o);

                    // an opcode for <control>
                    } else if status.is_header_control {
                        match o {
                            Opcode::default_path(p) => instrument.default_path.push(p),
                            _ => (),
                        }
                    } else {
                        // an opcode for the <region>
                        if status.are_regions_in_current_group() {
                            #[cfg(debug)]
                            println!(
                                "  - new region opcode: {:?} (g{} r{})",
                                o,
                                status.group_counter.unwrap(),
                                status.region_counter.unwrap()
                            );

                            instrument.add_opcode_to_region(o, status.region_counter.unwrap())?;
                            instrument.set_region_group(
                                status.region_counter.unwrap(),
                                status.group_counter,
                            )?;

                        // an opcode for the <group>
                        } else if status.are_groups() {
                            #[cfg(debug)]
                            println!("  - new group opcode: {:?}", o);

                            instrument.add_opcode_to_group(o, status.group_counter.unwrap())?;
                        } else {
                            unreachable!();
                        }
                    }
                }
                _ => (),
            }
        }

        #[cfg(debug)]
        println!("-----------------------------------\n");

        Ok(instrument)
    }

    /// Add an opcode, depending on context, to either the last created region,
    /// the last created group, or the global header (in that priority order)
    ///
    pub fn add_opcode(&mut self, opcode: &Opcode) -> Result<()> {
        match self.last_header_created {
            Header::Global => Ok(self.add_opcode_global(opcode)),
            Header::Group => self.add_opcode_to_group(opcode, self.groups() - 1),
            Header::Region => self.add_opcode_to_region(opcode, self.regions() - 1),
            _ => Err(Error::Generic),
        }
    }

    /// Add an opcode to the global header
    ///
    pub fn add_opcode_global(&mut self, opcode: &Opcode) {
        self.global.insert(opcode.str_name(), opcode.clone());
    }

    /// Add an opcode to a group
    pub fn add_opcode_to_group(&mut self, opcode: &Opcode, group: usize) -> Result<()> {
        if group >= self.groups() {
            return Err(Error::OutOfBounds(format![
                "Tried to add an Opcode to Group `{0}`, but the last group is `{1}`",
                group,
                self.groups() - 1
            ]));
        }
        self.groups[group].add_opcode(opcode);
        Ok(())
    }

    /// Add an opcode to a region
    pub fn add_opcode_to_region(&mut self, opcode: &Opcode, region: usize) -> Result<()> {
        #[cfg(debug)]
        println!(
            "    status.add_opcode_to_region() opcode: {:?} region: {}",
            opcode, region
        );

        if region >= self.regions() {
            return Err(Error::OutOfBounds(format![
                "Tried to add an Opcode to Region `{0}`, but the last region is `{1}`",
                region,
                self.regions() - 1
            ]));
        }
        self.regions[region].add_opcode(opcode);
        Ok(())
    }

    /// Get the number of groups
    pub fn groups(&self) -> usize {
        self.groups.len()
    }

    /// Get the number of regions
    pub fn regions(&self) -> usize {
        self.regions.len()
    }

    /// Get the number of regions in a group
    pub fn regions_in(&self, group: usize) -> Result<usize> {
        if group >= self.groups() {
            return Err(Error::OutOfBounds(format![
                "There's no group `{0}`, the last group is `{1}`",
                group,
                self.groups() - 1
            ]));
        }

        let mut count = 0;
        for region in self.regions.iter() {
            if region.group() == Some(group) {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Create a new empty group header in the Instrument
    pub fn new_group(&mut self) {
        self.groups.push(Group::new());
        self.last_header_created = Header::Group;
    }

    /// Create a new empty region header in the Instrument
    ///
    /// The region gets associated with the last group created (if any)
    pub fn new_region(&mut self) {
        let num_groups = self.groups();

        if num_groups > 0 {
            self.regions.push(Region::with_group(num_groups - 1));
        } else {
            self.regions.push(Region::new());
        }

        self.last_header_created = Header::Region;

        // NOTE: needs a Opcode::sample in order to be valid
        // maybe if the previous region doesn't have sample code, delete it?
        // I'm not sure that's a good idea. Maybe a sample can be added later.
        //
        // Maybe add a method to check and delete all the regions without a sample
    }

    /// Set the group of a region (group can be None)
    // FIXME: rethink receiving Option for group
    pub fn set_region_group(&mut self, region: usize, group: Option<usize>) -> Result<()> {
        #[cfg(debug)]
        println!(
            "    status.set_region_group() region: {} group: {:?}",
            region, group
        );
        if region >= self.regions() {
            return Err(Error::OutOfBounds(format![
                "Tried set the group of Region `{0}`, but the last region is `{1}`",
                region,
                self.regions() - 1
            ]));
        }
        if let Some(g) = group {
            if g >= self.groups() {
                return Err(Error::OutOfBounds(
                    format!["Tried to set Region `{0}` to have the Group `{1}`, but the last group is `{2}`",
                    region, g, self.groups()-1]
                ));
            }
        }
        // FIXME
        self.regions[region].set_group(group);
        Ok(())
    }

    // TODO:
    pub fn groups_iter(&self) -> () {}
}

/// The current status of the parsing of the instrument
#[derive(Debug)]
struct InstrumentParsingStatus {
    is_header_control: bool,
    is_header_global: bool,
    // counts groups (first one is 0, valid as index)
    group_counter: Option<usize>,
    // counts regions inside last group (first one is 0, valid as index)
    region_counter_in_group: Option<usize>,
    // counts all regions
    region_counter: Option<usize>,
}
impl InstrumentParsingStatus {
    pub fn init() -> Self {
        Self {
            is_header_control: false,
            is_header_global: false,
            group_counter: None,
            region_counter_in_group: None,
            region_counter: None,
        }
    }

    /// A new group header appears
    ///
    pub fn new_group(&mut self) {
        #[cfg(debug)]
        println!("  status.new_group()");
        // ensure we are out of the <control> header
        self.is_header_control = false;
        // ensure we are out of the <global> header
        self.is_header_global = false;
        // ensure we reset the region counter for the current group
        self.region_reset_in_current_group();
        // increment the group counter
        self.group_increment();
    }

    /// A new region header appears
    ///
    pub fn new_region(&mut self) {
        #[cfg(debug)]
        println!("  status.new_region()");
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
        match self.region_counter_in_group {
            Some(c) => self.region_counter_in_group = Some(c + 1),
            None => self.region_counter_in_group = Some(0),
        }
        #[cfg(debug)]
        println!(
            "  status.region_increment() g{}→rig{} (r{})",
            self.group_counter.unwrap(),
            self.region_counter_in_group.unwrap(),
            self.region_counter.unwrap()
        );
    }

    /// Resets the region counter for the group
    fn region_reset_in_current_group(&mut self) {
        #[cfg(debug)]
        println!("  status.region_reset_in_current_group()");
        self.region_counter_in_group = None;
    }

    /// Increments the group counter
    fn group_increment(&mut self) {
        match self.group_counter {
            Some(c) => self.group_counter = Some(c + 1),
            None => self.group_counter = Some(0),
        }
        #[cfg(debug)]
        println!(
            "  status.group_increment() (→g{})",
            self.group_counter.unwrap()
        );
    }

    /// Are there any groups already defined?
    pub fn are_groups(&self) -> bool {
        if self.group_counter == None {
            return false;
        }
        true
    }

    /// Are there any regions already defined for the current group?
    pub fn are_regions_in_current_group(&self) -> bool {
        if self.region_counter_in_group == None {
            return false;
        }
        true
    }
}
