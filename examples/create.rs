//! Create an instrument from scratch

use anyhow::Result;
use std::path::PathBuf;

use sofiza::{fil_type, Instrument, Opcode};

fn main() -> Result<()> {
    let mut i = Instrument::new();

    // This opcode is added to the global header,
    // because there are no regions or groups yet.
    i.add_opcode(&Opcode::fil_type(fil_type::lpf_2p))?;

    // create a new region (R0) with no group associated
    i.new_region();
    // these two opcodes are added to the newly created region
    i.add_opcode(&Opcode::cutoff(500.0))?;
    i.add_opcode(&Opcode::sample(PathBuf::from("sound1.wav")))?;

    // create a new group (G0)
    i.new_group();
    // add an opcode to this group
    i.add_opcode(&Opcode::resonance(10.0))?;
    // create a new region (R1) inside this group
    i.new_region();
    // add an opcode inside this region
    i.add_opcode(&Opcode::sample(PathBuf::from("sound2.wav")))?;

    i.new_group(); // new group (G1)
    i.new_region(); // new region (R2)

    // add new global opcode
    i.add_opcode_global(&Opcode::delay(20.));

    // add an opcode to region R1
    i.add_opcode_to_region(&Opcode::offset(400), 1)?;
    // add an opcode to group G0
    i.add_opcode_to_group(&Opcode::key(50), 0)?;

    // unsets the associated group of region R2
    i.set_region_group(2, None)?;

    // new region R3
    i.new_region();
    // sets the group of the last region (R3) to G0
    i.set_region_group(i.regions() - 1, Some(0))?;
    // add an opcode to the last region (R3)
    i.add_opcode(&Opcode::sample(PathBuf::from("sound3.wav")))?;

    // DEBUG
    println!("{:#?}", i);
    println!("groups: {}\nregions: {}", i.groups(), i.regions());

    Ok(())
}
