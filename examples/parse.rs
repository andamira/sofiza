//! This is a simple example that loads an sfz instrument,
//! and prints the parsed result
//!

use std::path::Path;

use sofiza::Instrument;

fn main() {
    let filepath = Path::new("instruments/Nylon Guitar.sfz");

    let i = Instrument::from_file(filepath).expect(
        "Couldn't parse the instrument. \
        Please run the example from the examples/ directory",
    );

    println!("{:#?}", i);

    println!("groups: {}\nregions: {}", i.groups(), i.regions());
}
