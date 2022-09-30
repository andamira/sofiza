//! This module defines several types used by opcodes
//!
//! SFZ format types:
//!
//! - loop_mode
//! - trigger
//! - fil_type
//!
//! Debug types:
//!
//! - UndefinedInteger
//! - UndefinedUnsignedInteger
//! - UnknownType

use std::collections::HashMap;
use std::path::PathBuf;

use crate::sfz::Opcode;

/// Theoretical maximum sample rate possible, used for type range checks
pub const MAX_SAMPLE_RATE: f32 = 384_000.0;

/// All the possible types allowed in an Opcode
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpcodeType {
    i8(Option<i8>),
    u8(Option<u8>),
    i16(Option<i16>),
    u16(Option<u16>),
    u32(Option<u32>),
    f32(Option<f32>),
    fil_type(Option<fil_type>),
    loop_mode(Option<loop_mode>),
    trigger(Option<trigger>),
    PathBuf(Option<PathBuf>),
    String(Option<&'static str>),
}

/// A Hashmap of opcodes, in which the key is the opcode's name
pub type OpcodeMap = HashMap<String, Opcode>;

/// Allows playing samples with loops defined in the unlooped mode.
///
/// - info: [loop_mode](https://sfzformat.com/opcodes/loop_mode)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum loop_mode {
    /// no looping will be performed. Sample will play straight from start to end,
    /// or until note off, whatever reaches first.
    ///
    /// This is the default.
    no_loop,

    /// sample will play from start to end, ignoring note off. This is commonly
    /// used for drums. This mode is engaged automatically if the count opcode
    /// is defined.
    one_shot,

    /// once the player reaches sample loop point, the loop will play until note
    /// expiration. This includes looping during the release phase.
    loop_continuous,

    /// the player will play the loop while the note is held, by keeping it
    /// depressed or by using the sustain pedal (CC64). During the release phase,
    /// there’s no looping.
    loop_sustain,
}

// IMPROVE: `no_loop` for samples without a loop defined,
// `loop_continuous` for samples with defined loop(s).
impl Default for loop_mode {
    fn default() -> loop_mode {
        Self::no_loop
    }
}

impl loop_mode {
    /// Constructor from the variant name, as a string
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "no_loop" => Some(Self::no_loop),
            "one_shot" => Some(Self::one_shot),
            "loop_continuous" => Some(Self::loop_continuous),
            "loop_sustain" => Some(Self::loop_sustain),
            _ => None,
        }
    }
}

/// Sets the trigger which will be used for the sample to play.
///
/// - info: [trigger](https://sfzformat.com/opcodes/trigger)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum trigger {
    /// (Default): Region will play on note-on.
    attack,

    /// Region will play on note-off or sustain pedal off. The velocity used to
    /// play the note-off sample is the velocity value of the corresponding
    /// (previous) note-on message.
    release,

    /// Region will play on note-on, but if there’s no other note going on
    /// (comoonly used for or first note in a legato phrase).
    first,

    /// Region will play on note-on, but only if there’s a note going on
    /// (notes after first note in a legato phrase).
    legato,

    /// Region will play on note-off. Ignores sustain pedal.
    release_key, // aria
}

impl Default for trigger {
    fn default() -> trigger {
        Self::attack
    }
}

impl trigger {
    /// Constructor from the variant name, as a string
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "attack" => Some(Self::attack),
            "release" => Some(Self::release),
            "first" => Some(Self::first),
            "legato" => Some(Self::legato),
            "release_key" => Some(Self::legato),
            _ => None,
        }
    }
}

/// Allows you to choose which type of filter you use if not specified
///
/// - info: [fil_type](https://sfzformat.com/opcodes/fil_type)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum fil_type {
    /// One-pole low pass filter (6dB/octave)
    ///
    /// - version: v1
    lpf_1p,

    /// One-pole high pass filter (6dB/octave)
    ///
    /// - version: v1
    hpf_1p,

    /// Two-pole low pass filter (12dB/octave)
    ///
    /// This is the default.
    ///
    /// - version: v1
    lpf_2p,

    /// Two-pole high pass filter (12dB/octave)
    ///
    /// - version: v1
    hpf_2p,

    /// Two-pole band pass filter (12dB/octave)
    ///
    /// - version: v1
    bpf_2p,

    /// Two-pole band rejection filter (12dB/octave)
    ///
    /// - version: v1
    brf_2p,

    /// One-pole band pass filter (6dB/octave)
    ///
    /// - version: v2
    bpf_1p,

    /// One-pole band rejection filter (6dB/octave)
    ///
    /// - version: v2
    brf_1p,

    /// One-pole all pass filter (6dB/octave)
    ///
    /// - version: v2
    apf_1p,

    /// Two-pole low pass state variable filter (12dB/octave)
    ///
    /// - version: v2
    lpf_2p_sv,

    /// Two-pole high pass state variable filter (12dB/octave)
    ///
    /// - version: v2
    hpf_2p_sv,

    /// Two-pole band pass state variable filter (12dB/octave)
    ///
    /// - version: v2
    bpf_2p_sv,

    /// Two-pole band rejection state variable filter (12dB/octave)
    ///
    /// - version: v2
    brf_2p_sv,

    /// Two-pole peak filter (12dB/octave)
    ///
    /// - version: v2
    pkf_2p,

    /// Four-pole low pass filter (24dB/octave)
    ///
    /// - version: v2
    lpf_4p,

    /// Four-pole high pass filter (24dB/octave)
    ///
    /// - version: v2
    hpf_4p,

    /// Six-pole low pass filter (36dB/octave)
    ///
    /// - version: v2
    lpf_6p,

    /// Six-pole high pass filter (36dB/octave)
    ///
    /// - version: v2
    hpf_6p,

    /// Comb filter
    ///
    /// - version: v2
    comb,

    /// Pink noise filter
    ///
    /// - version: v2
    pink,

    /// Low shelf
    ///
    /// - version: ARIA
    lsh,

    /// High shelf
    ///
    /// - version: ARIA
    hsh,

    /// Parametric EQ
    ///
    /// - version: ARIA
    peq,
}

impl Default for fil_type {
    fn default() -> fil_type {
        Self::lpf_2p
    }
}

impl fil_type {
    /// Constructor from the variant name, as a string
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "lpf_1p" => Some(Self::lpf_1p),
            "hpf_1p" => Some(Self::hpf_1p),
            "lpf_2p" => Some(Self::lpf_2p),
            "hpf_2p" => Some(Self::bpf_2p),
            "bpf_2p" => Some(Self::bpf_2p),
            "brf_2p" => Some(Self::brf_2p),
            "bpf_1p" => Some(Self::bpf_1p),
            "brf_1p" => Some(Self::brf_1p),
            "apf_1p" => Some(Self::apf_1p),
            "lpf_2p_sv" => Some(Self::lpf_2p_sv),
            "hpf_2p_sv" => Some(Self::hpf_2p_sv),
            "bpf_2p_sv" => Some(Self::bpf_2p_sv),
            "brf_2p_sv" => Some(Self::brf_2p_sv),
            "pkf_2p" => Some(Self::pkf_2p),
            "lpf_4p" => Some(Self::lpf_4p),
            "hpf_4p" => Some(Self::hpf_4p),
            "lpf_6p" => Some(Self::lpf_6p),
            "hpf_6p" => Some(Self::hpf_6p),
            "comb" => Some(Self::comb),
            "pink" => Some(Self::pink),
            "lsh" => Some(Self::lsh),
            "hsh" => Some(Self::hsh),
            "peq" => Some(Self::peq),
            _ => None,
        }
    }
}

/// This type is used for Opcodes with an unknown range and vague specification.
///
/// Opcodes using this type should be considered not stable, and its type
/// should be changed to a more defined one, eventually.
///
pub type UndefinedInteger = i32;

/// This type is used for Opcodes with an unknown or vague range specification,
/// but that can't be negative.
///
/// Opcodes using this type should be considered not stable, and its type
/// should be changed to a more defined one, eventually.
///
pub type UndefinedUnsignedInteger = u32;

/// This type is used for Opcodes with an unknown type.
///
/// Opcodes using this type should be considered unimplemented, and its type
/// should be changed to a more defined one, eventually.
pub type UnknownType = never::Never;
