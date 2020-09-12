//! The basic component of an instrument is a region.
//!
//! An instrument then, is defined by one or more regions.
//! Multiple regions can be arranged in a group.
//! Groups allow entering common parameters for multiple regions.

pub mod headers;
pub mod instrument;
pub mod group;
pub mod opcodes;
pub mod region;

pub mod types;
pub mod utils;

pub use group::Group;
pub use headers::Header;
pub use instrument::Instrument;
pub use opcodes::Opcode;
pub use region::Region;
pub use types::{OpcodeMap, OpcodeType};

pub(crate) use opcodes::SfzToken;
