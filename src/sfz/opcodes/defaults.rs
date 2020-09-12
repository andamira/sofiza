use phf::phf_map;

use crate::sfz::types::{fil_type, trigger, OpcodeType};

/// This map returns the optional default value of an opcode type
pub(crate) static OPCODE_DEFAULT: phf::Map<&'static str, OpcodeType> = phf_map! {

    // Opcodes in SFZ format version 1.0 --------------------------------

    "amp_random" => OpcodeType::f32(Some(0.0)),
    "amp_veltrack" => OpcodeType::f32(Some(100.0)),
    "ampeg_attack" => OpcodeType::f32(Some(0.0)),
    "ampeg_attackccN" => OpcodeType::f32(Some(0.0)), // XXX < how to store. Vec? HashMap?
    "ampeg_decay" => OpcodeType::f32(Some(0.0)),
    "ampeg_hold" => OpcodeType::f32(Some(0.0)),
    "ampeg_release" => OpcodeType::f32(Some(0.0)),
    "ampeg_sustain" => OpcodeType::f32(Some(100.0)),
    "cutoff" => OpcodeType::f32(None),
    "fil_type" => OpcodeType::fil_type(Some(fil_type::lpf_2p)),
    "fil_veltrack" => OpcodeType::i16(Some(0)),
    "hikey" => OpcodeType::u8(Some(127)),
    "hirand" => OpcodeType::f32(Some(1.0)),
    "hivel" => OpcodeType::u8(Some(127)),
    "lokey" => OpcodeType::u8(Some(0)),
    "loop_mode" => OpcodeType::loop_mode(None), // no_loop for samples without a loop defined,
                                                // loop_continuous for samples w defined loop(s)
    "lorand" => OpcodeType::f32(Some(0.0)),
    "lovel" => OpcodeType::u8(Some(0)),
    "off_by" => OpcodeType::f32(Some(0.0)),
    "offset" => OpcodeType::u32(Some(0)),
    "on_loccN" => OpcodeType::i8(Some(-1)), // XXX < how to store. Vec? HashMap?
    "on_hiccN" => OpcodeType::i8(Some(-1)), // XXX < how to store. Vec? HashMap?
    "pan" => OpcodeType::f32(Some(0.0)),
    "pitch_keycenter" => OpcodeType::u8(Some(60)),
    "pitch_keytrack" => OpcodeType::i16(Some(100)),
    "pitch_random" => OpcodeType::u16(Some(0)),
    "rt_decay" => OpcodeType::f32(Some(0.0)),
    "sample" => OpcodeType::PathBuf(None),
    "seq_lengh" => OpcodeType::u8(Some(1)),
    "seq_position" => OpcodeType::u8(Some(1)),
    "sw_hikey" => OpcodeType::u8(Some(127)),
    "sw_last" => OpcodeType::u8(Some(0)),
    "sw_lowkey" => OpcodeType::u8(Some(0)),
    "trigger" => OpcodeType::trigger(Some(trigger::attack)),
    "tune" => OpcodeType::i8(Some(0)),
    "volume" => OpcodeType::f32(Some(0.0)),
    "xfin_hivel" => OpcodeType::u8(Some(0)),
    "xfin_lovel" => OpcodeType::u8(Some(0)),
    "xfout_hivel" => OpcodeType::u8(Some(127)),
    "xfout_lovel" => OpcodeType::u8(Some(127)),

    // Opcodes in SFZ format version 2.0 --------------------------------

    "sw_default" => OpcodeType::u8(None),

    // Opcodes in SFZ aria extensions -----------------------------------

    "ampeg_dynamic" => OpcodeType::u8(Some(0)),
    "group_label" => OpcodeType::String(Some("")),
    "sw_label" => OpcodeType::String(None),
};
