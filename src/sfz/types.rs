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
pub const MAX_SAMPLE_RATE:f32 = 384_000.0;


/// All the possible types allowed in an Opcode
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
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


#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum loop_mode {
    no_loop,
    one_shot,
    loop_continuous,
    loop_sustain,
}
impl loop_mode {
    /// Constructor from the variant name, as a string
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "no_loop" => Some(Self::no_loop),
            "one_shot" => Some(Self::one_shot),
            "loop_continuous" => Some(Self::loop_continuous),
            "loop_sustain" => Some(Self::loop_sustain),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum trigger {
    attack,
    release,
    first,
    legato,
    release_key, // aria
}
impl trigger {
    /// Constructor from the variant name, as a string
    pub fn from_str(name: &str) -> Option<Self> {
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
/// - info: [fil_type] (https://sfzformat.com/opcodes/fil_type)
#[derive(Debug, Clone, PartialEq)]
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
impl fil_type {
    /// Constructor from the variant name, as a string
    pub fn from_str(name: &str) -> Option<Self> {
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

