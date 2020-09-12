//! This is a simple example that loads an sfz instrument,
//! and prints the parsed result
//!

use std::path::Path;

use sofiza::Instrument;


fn main() {
    let filepath = shellexpand::tilde("instrument_harp.sfz").to_string();

    // open and parse the file, creating a new instrument
    let instrument = Instrument::from_file(Path::new(&filepath));

    println!("{:#?}", instrument);
}
