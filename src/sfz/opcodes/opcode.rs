use std::path::PathBuf;

use crate::sfz::opcodes::defaults::OPCODE_DEFAULT;
use crate::sfz::types::{
    fil_type, loop_mode, trigger, OpcodeType, UndefinedInteger, UndefinedUnsignedInteger,
    UnknownType,
};

/// Opcodes are special keywords which instruct the player
/// on what, when and how to play a sample.
///
/// Opcodes within a region can appear in any order,
/// and they have to be separated by one or more spaces or tabulation controls.
/// Opcodes can appear in separated lines within a region.
///
/// Opcodes and assigned opcode values are separated by the equal to sign (=),
/// without spaces between the opcode and the sign.
///
/// ## Links
///
/// - [All Opcodes](https://sfzformat.com/opcodes/)
///  - [SFZ v1 Opcodes](https://sfzformat.com/misc/sfz1)
///  - [SFZ v2 Opcodes](https://sfzformat.com/misc/sfz2)
///  - [Aria extension Opcodes](https://sfzformat.com/extensions/aria/)
///  - [Cakewalk SFZ v2 Opcodes](https://sfzformat.com/misc/cakewalk)
///  - [Cakewalk Extensions Opcodes](https://sfzformat.com/extensions/cakewalk/)
///
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
    // sfz v1 opcodes -----------------------------------------------------------
    // https://sfzformat.com/misc/sfz2
    ///
    /// - range: 0 to u32::MAX
    /// - default: 0
    /// - version: v1
    /// - info: [count](https://sfzformat.com/opcodes/count)
    ///
    count(u32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [delay](https://sfzformat.com/opcodes/delay)
    ///
    delay(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [delay_ccN](https://sfzformat.com/opcodes/delay_ccN)
    ///
    delay_ccN(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [delay_random](https://sfzformat.com/opcodes/delay_random)
    ///
    delay_random(f32),

    ///
    /// - range: 0 to u32::MAX sample units
    /// - default: unspecified
    /// - version: v1
    /// - info: [end](https://sfzformat.com/opcodes/end)
    ///
    end(u32),

    ///
    /// - range: no_loop, one_shot, loop_continuous, loop_sustain
    /// - default:
    ///     - no_loop for samples without a loop defined
    ///     - loop_continuous for samples with defined loop(s)
    /// - version: v1
    /// - info: [loop_mode](https://sfzformat.com/opcodes/loop_mode)
    ///
    loop_mode(loop_mode),

    ///
    /// - range: 0 to u32::MAX
    /// - default: 0
    /// - version: v1
    /// - info: [loop_start](https://sfzformat.com/opcodes/loop_start)
    ///
    loop_start(u32),

    ///
    /// - range: 0 to u32::MAX
    /// - default: 0
    /// - version: v1
    /// - info: [loop_end](https://sfzformat.com/opcodes/loop_end)
    ///
    loop_end(u32),

    /// The offset used to play the sample.
    ///
    ///
    /// - range: 0 to u32::MAX sample units
    /// - default: 0
    /// - version: v1
    /// - info: [offset](https://sfzformat.com/opcodes/offset)
    ///
    offset(u32),

    ///
    /// - range: 0 to u32::MAX sample units
    /// - default: 0
    /// - version: v1
    /// - info: [offset_ccN](https://sfzformat.com/opcodes/offset_ccN)
    ///
    offset_ccN(u32),

    ///
    /// - range: 0 to u32::MAX sample units
    /// - default: 0
    /// - version: v1
    /// - info: [offset_random](https://sfzformat.com/opcodes/offset_random)
    ///
    offset_random(u32),

    /// Defines which sample file the region will play.
    ///
    /// If the sample file is not found, the player will ignore the whole region
    /// contents as there’s nothing to play.
    ///
    /// Long names and names with blank spaces and other special characters
    /// (excepting the = character) are allowed in the sample definition.
    ///
    /// - range: None
    /// - default: None
    /// - version: v1
    /// - info: [sample](https://sfzformat.com/opcodes/sample)
    ///
    sample(PathBuf),

    ///
    /// - range: 0 to 32 beats
    /// - default: 0
    /// - version: v1
    /// - info: [sync_beats](https://sfzformat.com/opcodes/sync_beats)
    ///
    sync_beats(f32),

    ///
    /// - range: 0 to 32 beats
    /// - default: 0
    /// - version: v1
    /// - info: [sync_offset](https://sfzformat.com/opcodes/sync_offset)
    ///
    sync_offset(f32),

    ///
    /// - range: 0 to u32::MAX
    /// - default: 0
    /// - version: v1
    /// - info: [group](https://sfzformat.com/opcodes/group)
    ///
    group(u32),

    ///
    /// - range: 0 to u32::MAX
    /// - default: 0
    /// - version: v1
    /// - info: [off_by](https://sfzformat.com/opcodes/off_by)
    ///
    off_by(u32),

    ///
    /// - range: fast, normal, time
    /// - default: fast
    /// - version: v1
    /// - info: [off_mode](https://sfzformat.com/opcodes/off_mode)
    ///
    off_mode(String),

    ///
    /// - range: 0 to 1024
    /// - default: 0
    /// - version: v1
    /// - info: [output](https://sfzformat.com/opcodes/output)
    ///
    output(u16),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v1
    /// - info: [key](https://sfzformat.com/opcodes/key)
    ///
    key(u8),

    /// Determine the low boundary of a certain region.
    ///
    /// The region will play if the note played is >= lokey && <= hikey.
    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [lokey](https://sfzformat.com/opcodes/lokey)
    ///
    lokey(u8),

    /// Determine the high boundary of a certain region.
    ///
    /// The region will play if the note played is >= lokey && <= hikey.
    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [hikey](https://sfzformat.com/opcodes/hikey)
    ///
    hikey(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [lovel](https://sfzformat.com/opcodes/lovel)
    ///
    lovel(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [hivel](https://sfzformat.com/opcodes/hivel)
    ///
    hivel(u8),

    /// If incoming notes have a MIDI channel between lochan and hichan,
    /// the region will play.
    ///
    /// - range: 1 to 16
    /// - default: 1
    /// - version: v1
    /// - info: [lochan](https://sfzformat.com/opcodes/lochan)
    ///
    lochan(u8),

    /// If incoming notes have a MIDI channel between lochan and hichan,
    /// the region will play.
    ///
    /// - range: 1 to 16
    /// - default: 16
    /// - version: v1
    /// - info: [hichan](https://sfzformat.com/opcodes/hichan)
    ///
    hichan(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [loccN](https://sfzformat.com/opcodes/loccN)
    ///
    loccN(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [hiccN](https://sfzformat.com/opcodes/hiccN)
    ///
    hiccN(u8),

    ///
    /// - range: -8192 to 8192
    /// - default: -8192
    /// - version: v1
    /// - info: [lobend](https://sfzformat.com/opcodes/lobend)
    ///
    lobend(i16),

    ///
    /// - range: -8192 to 8192
    /// - default: 8192
    /// - version: v1
    /// - info: [hibend](https://sfzformat.com/opcodes/hibend)
    ///
    hibend(i16),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [sw_lokey](https://sfzformat.com/opcodes/sw_lokey)
    ///
    sw_lokey(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [sw_hikey](https://sfzformat.com/opcodes/sw_hikey)
    ///
    sw_hikey(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [sw_last](https://sfzformat.com/opcodes/sw_last)
    ///
    sw_last(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [sw_down](https://sfzformat.com/opcodes/sw_down)
    ///
    sw_down(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [sw_up](https://sfzformat.com/opcodes/sw_up)
    ///
    sw_up(u8),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v1
    /// - info: [sw_previous](https://sfzformat.com/opcodes/sw_previous)
    ///
    sw_previous(u8),

    ///
    /// - range: current, previous
    /// - default: current
    /// - version: v1
    /// - info: [sw_vel](https://sfzformat.com/opcodes/sw_vel)
    ///
    sw_vel(String),

    ///
    /// - range: 0 to 500 bpm
    /// - default: 0
    /// - version: v1
    /// - info: [lobpm](https://sfzformat.com/opcodes/lobpm)
    ///
    lobpm(f32),

    ///
    /// - range: 0 to 500 bpm
    /// - default: 500
    /// - version: v1
    /// - info: [hibpm](https://sfzformat.com/opcodes/hibpm)
    ///
    hibpm(f32),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [lochanaft](https://sfzformat.com/opcodes/lochanaft)
    ///
    lochanaft(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [hichanaft](https://sfzformat.com/opcodes/hichanaft)
    ///
    hichanaft(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [lopolyaft](https://sfzformat.com/opcodes/lopolyaft)
    ///
    lopolyaft(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [hipolyaft](https://sfzformat.com/opcodes/hipolyaft)
    ///
    hipolyaft(u8),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v1
    /// - info: [lorand](https://sfzformat.com/opcodes/lorand)
    ///
    lorand(f32),

    ///
    /// - range: 0 to 1
    /// - default: 1
    /// - version: v1
    /// - info: [hirand](https://sfzformat.com/opcodes/hirand)
    ///
    hirand(f32),

    ///
    /// - range: 1 to 100
    /// - default: 1
    /// - version: v1
    /// - info: [seq_length](https://sfzformat.com/opcodes/seq_length)
    ///
    seq_length(u8),

    ///
    /// - range: 1 to 100
    /// - default: 1
    /// - version: v1
    /// - info: [seq_position](https://sfzformat.com/opcodes/seq_position)
    ///
    seq_position(u8),

    ///
    /// - range: attack, release, first, legato, release_key
    /// - default: attack
    /// - version: v1
    /// - info: [trigger](https://sfzformat.com/opcodes/trigger)
    ///
    trigger(trigger),

    ///
    /// - range: 0 to 127
    /// - default: -1
    /// - version: v1
    /// - info: [on_loccN](https://sfzformat.com/opcodes/on_loccN)
    ///
    on_loccN(i8),

    ///
    /// - range: 0 to 127
    /// - default: -1
    /// - version: v1
    /// - info: [on_hiccN](https://sfzformat.com/opcodes/on_hiccN)
    ///
    on_hiccN(i8),

    /// The panoramic position for the region.
    ///
    /// If a mono sample is used, pan value defines the position in the stereo
    /// image where the sample will be placed. When a stereo sample is used,
    /// the pan value the relative amplitude of one channel respect the other.
    ///
    /// A value of zero means centered, negative values move the panoramic
    /// to the left, positive to the right.
    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [pan](https://sfzformat.com/opcodes/pan)
    ///
    pan(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [position](https://sfzformat.com/opcodes/position)
    ///
    position(f32),

    ///
    /// - range: -144.0 to 6.0 dB
    /// - default: 0
    /// - version: v1
    /// - info: [volume](https://sfzformat.com/opcodes/volume)
    ///
    volume(f32),

    ///
    /// - range: -144 to 48 dB
    /// - default: 0
    /// - version: v1
    /// - info: [gain_ccN](https://sfzformat.com/opcodes/gain_ccN)
    ///
    gain_ccN(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 100
    /// - version: v1
    /// - info: [width](https://sfzformat.com/opcodes/width)
    ///
    width(f32),

    ///
    /// - range: 0 to 127
    /// - default: 60
    /// - version: v1
    /// - info: [amp_keycenter](https://sfzformat.com/opcodes/amp_keycenter)
    ///
    amp_keycenter(u8),

    ///
    /// - range: -96 to 12 dB
    /// - default: 0
    /// - version: v1
    /// - info: [amp_keytrack](https://sfzformat.com/opcodes/amp_keytrack)
    ///
    amp_keytrack(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 100
    /// - version: v1
    /// - info: [amp_veltrack](https://sfzformat.com/opcodes/amp_veltrack)
    ///
    amp_veltrack(f32),

    ///
    /// - range: 0 to 1
    /// - default: Standard curve (see amp_veltrack)
    /// - version: v1
    /// - info: [amp_velcurve_N](https://sfzformat.com/opcodes/amp_velcurve_N)
    ///
    amp_velcurve_N(f32),

    ///
    /// - range: 0 to 24 dB
    /// - default: 0
    /// - version: v1
    /// - info: [amp_random](https://sfzformat.com/opcodes/amp_random)
    ///
    amp_random(f32),

    ///
    /// - range: 0 to 200 dB
    /// - default: 0
    /// - version: v1
    /// - info: [rt_decay](https://sfzformat.com/opcodes/rt_decay)
    ///
    rt_decay(f32),

    ///
    /// - range: gain, power
    /// - default: power
    /// - version: v1
    /// - info: [xf_cccurve](https://sfzformat.com/opcodes/xf_cccurve)
    ///
    xf_cccurve(String),

    ///
    /// - range: gain, power
    /// - default: power
    /// - version: v1
    /// - info: [xf_keycurve](https://sfzformat.com/opcodes/xf_keycurve)
    ///
    xf_keycurve(String),

    ///
    /// - range: gain, power
    /// - default: power
    /// - version: v1
    /// - info: [xf_velcurve](https://sfzformat.com/opcodes/xf_velcurve)
    ///
    xf_velcurve(String),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfin_loccN](https://sfzformat.com/opcodes/xfin_loccN)
    ///
    xfin_loccN(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfin_hiccN](https://sfzformat.com/opcodes/xfin_hiccN)
    ///
    xfin_hiccN(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfout_loccN](https://sfzformat.com/opcodes/xfout_loccN)
    ///
    xfout_loccN(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfout_hiccN](https://sfzformat.com/opcodes/xfout_hiccN)
    ///
    xfout_hiccN(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfin_lokey](https://sfzformat.com/opcodes/xfin_lokey)
    ///
    xfin_lokey(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfin_hikey](https://sfzformat.com/opcodes/xfin_hikey)
    ///
    xfin_hikey(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [xfout_lokey](https://sfzformat.com/opcodes/xfout_lokey)
    ///
    xfout_lokey(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [xfout_hikey](https://sfzformat.com/opcodes/xfout_hikey)
    ///
    xfout_hikey(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfin_lovel](https://sfzformat.com/opcodes/xfin_lovel)
    ///
    xfin_lovel(u8),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [xfin_hivel](https://sfzformat.com/opcodes/xfin_hivel)
    ///
    xfin_hivel(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [xfout_lovel](https://sfzformat.com/opcodes/xfout_lovel)
    ///
    xfout_lovel(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v1
    /// - info: [xfout_hivel](https://sfzformat.com/opcodes/xfout_hivel)
    ///
    xfout_hivel(u8),

    ///
    /// - range: 0.001 to 4 octaves
    /// - default: 1
    /// - version: v1
    /// - info: [eqN_bw](https://sfzformat.com/opcodes/eqN_bw)
    ///
    eqN_bw(f32),

    ///
    /// - range: -4 to 4 octaves
    /// - default: 0
    /// - version: v1
    /// - info: [eqN_bwccX](https://sfzformat.com/opcodes/eqN_bwccX)
    ///
    eqN_bwccX(f32),

    ///
    /// - range: 0 to 30000 Hz
    /// - default: eq1_freq=50 | eq1_freq=500 | eq1_freq=5000
    /// - version: v1
    /// - info: [eqN_freq](https://sfzformat.com/opcodes/eqN_freq)
    ///
    eqN_freq(f32),

    ///
    /// - range: -30000 to 30000 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [eqN_freqccX](https://sfzformat.com/opcodes/eqN_freqccX)
    ///
    eqN_freqccX(f32),

    ///
    /// - range: -30000 to 30000 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [eqN_vel2freq](https://sfzformat.com/opcodes/eqN_vel2freq)
    ///
    eqN_vel2freq(f32),

    ///
    /// - range: -96 to 24 dB
    /// - default: 0
    /// - version: v1
    /// - info: [eqN_gain](https://sfzformat.com/opcodes/eqN_gain)
    ///
    eqN_gain(f32),

    ///
    /// - range: -96 to 24 dB
    /// - default: 0
    /// - version: v1
    /// - info: [eqN_gainccX](https://sfzformat.com/opcodes/eqN_gainccX)
    ///
    eqN_gainccX(f32),

    ///
    /// - range: -96 to 24 dB
    /// - default: 0
    /// - version: v1
    /// - info: [eqN_vel2gain](https://sfzformat.com/opcodes/eqN_vel2gain)
    ///
    eqN_vel2gain(f32),

    ///
    /// - range: 0 to SampleRate / 2 Hz
    /// - default: filter disabled
    /// - version: v1
    /// - info: [cutoff](https://sfzformat.com/opcodes/cutoff)
    ///
    cutoff(f32),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 0
    /// - version: v1
    /// - info: [cutoff_ccN](https://sfzformat.com/opcodes/cutoff_ccN)
    ///
    cutoff_ccN(i16),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 0
    /// - version: v1
    /// - info: [cutoff_chanaft](https://sfzformat.com/opcodes/cutoff_chanaft)
    ///
    cutoff_chanaft(i16),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 0
    /// - version: v1
    /// - info: [cutoff_polyaft](https://sfzformat.com/opcodes/cutoff_polyaft)
    ///
    cutoff_polyaft(i16),

    ///
    /// - range: 0 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fil_keytrack](https://sfzformat.com/opcodes/fil_keytrack)
    ///
    fil_keytrack(i16),

    ///
    /// - range: 0 to 127
    /// - default: 60
    /// - version: v1
    /// - info: [fil_keycenter](https://sfzformat.com/opcodes/fil_keycenter)
    ///
    fil_keycenter(u8),

    ///
    /// - range: 0 to 9600 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fil_random](https://sfzformat.com/opcodes/fil_random)
    ///
    fil_random(u16),

    ///
    /// - range: lpf_1p, hpf_1p, lpf_2p, hpf_2p, bpf_2p, brf_2p, bpf_1p, brf_1p,
    ///          apf_1p, lpf_2p_sv, hpf_2p_sv, bpf_2p_sv, brf_2p_sv, pkf_2p,
    ///          lpf_4p, hpf_4p, lpf_6p, hpf_6p, comb, pink, lsh, hsh, peq
    /// - default: lpf_2p
    /// - version: v1
    /// - info: [fil_type](https://sfzformat.com/opcodes/fil_type)
    ///
    fil_type(fil_type),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fil_veltrack](https://sfzformat.com/opcodes/fil_veltrack)
    ///
    fil_veltrack(i16),

    ///
    /// - range: 0 to 40 dB
    /// - default: 0
    /// - version: v1
    /// - info: [resonance](https://sfzformat.com/opcodes/resonance)
    ///
    resonance(f32),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 200
    /// - version: v1
    /// - info: [bend_up](https://sfzformat.com/opcodes/bend_up)
    ///
    bend_up(i16),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: -200
    /// - version: v1
    /// - info: [bend_down](https://sfzformat.com/opcodes/bend_down)
    ///
    bend_down(i16),

    ///
    /// - range: 1 to 1200 cents
    /// - default: 1
    /// - version: v1
    /// - info: [bend_step](https://sfzformat.com/opcodes/bend_step)
    ///
    bend_step(u16),

    /// For samples which only need to be played at their natural pitch and
    /// triggered by one specific MIDI note, it’s generally easier to use key instead.
    ///
    /// - range: 0 to 127
    /// - default: 60
    /// - version: v1
    /// - info: [pitch_keycenter](https://sfzformat.com/opcodes/pitch_keycenter)
    ///
    pitch_keycenter(u8),

    ///
    /// - range: -1200 to 1200
    /// - default: 100
    /// - version: v1
    /// - info: [pitch_keytrack](https://sfzformat.com/opcodes/pitch_keytrack)
    ///
    pitch_keytrack(i16),

    ///
    /// - range: 0 to 9600 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitch_random](https://sfzformat.com/opcodes/pitch_random)
    ///
    pitch_random(u16),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitch_veltrack](https://sfzformat.com/opcodes/pitch_veltrack)
    ///
    pitch_veltrack(i16),

    ///
    /// - range: -127 to 127
    /// - default: 0
    /// - version: v1
    /// - info: [transpose](https://sfzformat.com/opcodes/transpose)
    ///
    transpose(i8),

    /// The fine tuning for the sample, in cents.
    /// Range of tune in the SFZ1 spec is ±1 semitone, from -100 to 100,
    ///
    /// - range: -100 to 100 cents
    /// - default: 0
    /// - version: v1
    /// - info: [tune](https://sfzformat.com/opcodes/tune)
    ///
    tune(i8),

    ///
    /// - range: 0.0 to 100.0 seconds
    /// - default: 0.0
    /// - version: v1
    /// - info: [ampeg_attack](https://sfzformat.com/opcodes/ampeg_attack)
    ///
    ampeg_attack(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_attackccN](https://sfzformat.com/opcodes/ampeg_attackccN)
    ///
    ampeg_attackccN(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_vel2attack](https://sfzformat.com/opcodes/ampeg_vel2attack)
    ///
    ampeg_vel2attack(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_decay](https://sfzformat.com/opcodes/ampeg_decay)
    ///
    ampeg_decay(f32),

    ///
    /// - range: -100 to 100
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_decayccN](https://sfzformat.com/opcodes/ampeg_decayccN)
    ///
    ampeg_decayccN(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_vel2decay](https://sfzformat.com/opcodes/ampeg_vel2decay)
    ///
    ampeg_vel2decay(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_delay](https://sfzformat.com/opcodes/ampeg_delay)
    ///
    ampeg_delay(f32),

    ///
    /// - range: -100 to 100
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_delayccN](https://sfzformat.com/opcodes/ampeg_delayccN)
    ///
    ampeg_delayccN(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_vel2delay](https://sfzformat.com/opcodes/ampeg_vel2delay)
    ///
    ampeg_vel2delay(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_hold](https://sfzformat.com/opcodes/ampeg_hold)
    ///
    ampeg_hold(f32),

    ///
    /// - range: -100 to 100
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_holdccN](https://sfzformat.com/opcodes/ampeg_holdccN)
    ///
    ampeg_holdccN(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_vel2hold](https://sfzformat.com/opcodes/ampeg_vel2hold)
    ///
    ampeg_vel2hold(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_release](https://sfzformat.com/opcodes/ampeg_release)
    ///
    ampeg_release(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_releaseccN](https://sfzformat.com/opcodes/ampeg_releaseccN)
    ///
    ampeg_releaseccN(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_vel2release](https://sfzformat.com/opcodes/ampeg_vel2release)
    ///
    ampeg_vel2release(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 100
    /// - version: v1
    /// - info: [ampeg_sustain](https://sfzformat.com/opcodes/ampeg_sustain)
    ///
    ampeg_sustain(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_sustainccN](https://sfzformat.com/opcodes/ampeg_sustainccN)
    ///
    ampeg_sustainccN(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_vel2sustain](https://sfzformat.com/opcodes/ampeg_vel2sustain)
    ///
    ampeg_vel2sustain(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_start](https://sfzformat.com/opcodes/ampeg_start)
    ///
    ampeg_start(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [ampeg_startccN](https://sfzformat.com/opcodes/ampeg_startccN)
    ///
    ampeg_startccN(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_attack](https://sfzformat.com/opcodes/fileg_attack)
    ///
    fileg_attack(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_vel2attack](https://sfzformat.com/opcodes/fileg_vel2attack)
    ///
    fileg_vel2attack(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_decay](https://sfzformat.com/opcodes/fileg_decay)
    ///
    fileg_decay(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_vel2decay](https://sfzformat.com/opcodes/fileg_vel2decay)
    ///
    fileg_vel2decay(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_delay](https://sfzformat.com/opcodes/fileg_delay)
    ///
    fileg_delay(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_vel2delay](https://sfzformat.com/opcodes/fileg_vel2delay)
    ///
    fileg_vel2delay(f32),

    ///
    /// - range: -12000 to 12000 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_depth](https://sfzformat.com/opcodes/fileg_depth)
    ///
    fileg_depth(i16),

    ///
    /// - range: -12000 to 12000 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_vel2depth](https://sfzformat.com/opcodes/fileg_vel2depth)
    ///
    fileg_vel2depth(i16),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_hold](https://sfzformat.com/opcodes/fileg_hold)
    ///
    fileg_hold(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_vel2hold](https://sfzformat.com/opcodes/fileg_vel2hold)
    ///
    fileg_vel2hold(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_release](https://sfzformat.com/opcodes/fileg_release)
    ///
    fileg_release(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_vel2release](https://sfzformat.com/opcodes/fileg_vel2release)
    ///
    fileg_vel2release(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_start](https://sfzformat.com/opcodes/fileg_start)
    ///
    fileg_start(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_sustain](https://sfzformat.com/opcodes/fileg_sustain)
    ///
    fileg_sustain(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [fileg_vel2sustain](https://sfzformat.com/opcodes/fileg_vel2sustain)
    ///
    fileg_vel2sustain(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_attack](https://sfzformat.com/opcodes/pitcheg_attack)
    ///
    pitcheg_attack(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_vel2attack](https://sfzformat.com/opcodes/pitcheg_vel2attack)
    ///
    pitcheg_vel2attack(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_decay](https://sfzformat.com/opcodes/pitcheg_decay)
    ///
    pitcheg_decay(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_vel2decay](https://sfzformat.com/opcodes/pitcheg_vel2decay)
    ///
    pitcheg_vel2decay(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_delay](https://sfzformat.com/opcodes/pitcheg_delay)
    ///
    pitcheg_delay(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_vel2delay](https://sfzformat.com/opcodes/pitcheg_vel2delay)
    ///
    pitcheg_vel2delay(f32),

    ///
    /// - range: -12000 to 12000 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_depth](https://sfzformat.com/opcodes/pitcheg_depth)
    ///
    pitcheg_depth(i16),

    ///
    /// - range: -12000 to 12000 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_vel2depth](https://sfzformat.com/opcodes/pitcheg_vel2depth)
    ///
    pitcheg_vel2depth(i16),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_hold](https://sfzformat.com/opcodes/pitcheg_hold)
    ///
    pitcheg_hold(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_vel2hold](https://sfzformat.com/opcodes/pitcheg_vel2hold)
    ///
    pitcheg_vel2hold(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_release](https://sfzformat.com/opcodes/pitcheg_release)
    ///
    pitcheg_release(f32),

    ///
    /// - range: -100 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_vel2release](https://sfzformat.com/opcodes/pitcheg_vel2release)
    ///
    pitcheg_vel2release(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_start](https://sfzformat.com/opcodes/pitcheg_start)
    ///
    pitcheg_start(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_sustain](https://sfzformat.com/opcodes/pitcheg_sustain)
    ///
    pitcheg_sustain(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [pitcheg_vel2sustain](https://sfzformat.com/opcodes/pitcheg_vel2sustain)
    ///
    pitcheg_vel2sustain(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_delay](https://sfzformat.com/opcodes/amplfo_delay)
    ///
    amplfo_delay(f32),

    ///
    /// - range: -10 to 10 dB
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_depth](https://sfzformat.com/opcodes/amplfo_depth)
    ///
    amplfo_depth(f32),

    ///
    /// - range: -10 to 10 dB
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_depthccN](https://sfzformat.com/opcodes/amplfo_depthccN)
    ///
    amplfo_depthccN(f32),

    ///
    /// - range: -10 to 10 dB
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_depthchanaft](https://sfzformat.com/opcodes/amplfo_depthchanaft)
    ///
    amplfo_depthchanaft(f32),

    ///
    /// - range: -10 to 10 dB
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_depthpolyaft](https://sfzformat.com/opcodes/amplfo_depthpolyaft)
    ///
    amplfo_depthpolyaft(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_fade](https://sfzformat.com/opcodes/amplfo_fade)
    ///
    amplfo_fade(f32),

    ///
    /// - range: 0 to 20 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_freq](https://sfzformat.com/opcodes/amplfo_freq)
    ///
    amplfo_freq(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_freqccN](https://sfzformat.com/opcodes/amplfo_freqccN)
    ///
    amplfo_freqccN(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_freqchanaft](https://sfzformat.com/opcodes/amplfo_freqchanaft)
    ///
    amplfo_freqchanaft(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [amplfo_freqpolyaft](https://sfzformat.com/opcodes/amplfo_freqpolyaft)
    ///
    amplfo_freqpolyaft(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_delay](https://sfzformat.com/opcodes/fillfo_delay)
    ///
    fillfo_delay(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_depth](https://sfzformat.com/opcodes/fillfo_depth)
    ///
    fillfo_depth(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_depthccN](https://sfzformat.com/opcodes/fillfo_depthccN)
    ///
    fillfo_depthccN(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_depthchanaft](https://sfzformat.com/opcodes/fillfo_depthchanaft)
    ///
    fillfo_depthchanaft(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_depthpolyaft](https://sfzformat.com/opcodes/fillfo_depthpolyaft)
    ///
    fillfo_depthpolyaft(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_fade](https://sfzformat.com/opcodes/fillfo_fade)
    ///
    fillfo_fade(f32),

    ///
    /// - range: 0 to 20 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_freq](https://sfzformat.com/opcodes/fillfo_freq)
    ///
    fillfo_freq(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_freqccN](https://sfzformat.com/opcodes/fillfo_freqccN)
    ///
    fillfo_freqccN(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_freqchanaft](https://sfzformat.com/opcodes/fillfo_freqchanaft)
    ///
    fillfo_freqchanaft(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [fillfo_freqpolyaft](https://sfzformat.com/opcodes/fillfo_freqpolyaft)
    ///
    fillfo_freqpolyaft(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_delay](https://sfzformat.com/opcodes/pitchlfo_delay)
    ///
    pitchlfo_delay(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_depth](https://sfzformat.com/opcodes/pitchlfo_depth)
    ///
    pitchlfo_depth(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_depthccN](https://sfzformat.com/opcodes/pitchlfo_depthccN)
    ///
    pitchlfo_depthccN(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_depthchanaft](https://sfzformat.com/opcodes/pitchlfo_depthchanaft)
    ///
    pitchlfo_depthchanaft(f32),

    ///
    /// - range: -1200 to 1200 cents
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_depthpolyaft](https://sfzformat.com/opcodes/pitchlfo_depthpolyaft)
    ///
    pitchlfo_depthpolyaft(f32),

    ///
    /// - range: 0 to 100 seconds
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_fade](https://sfzformat.com/opcodes/pitchlfo_fade)
    ///
    pitchlfo_fade(f32),

    ///
    /// - range: 0 to 20 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_freq](https://sfzformat.com/opcodes/pitchlfo_freq)
    ///
    pitchlfo_freq(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_freqccN](https://sfzformat.com/opcodes/pitchlfo_freqccN)
    ///
    pitchlfo_freqccN(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_freqchanaft](https://sfzformat.com/opcodes/pitchlfo_freqchanaft)
    ///
    pitchlfo_freqchanaft(f32),

    ///
    /// - range: -200 to 200 Hz
    /// - default: 0
    /// - version: v1
    /// - info: [pitchlfo_freqpolyaft](https://sfzformat.com/opcodes/pitchlfo_freqpolyaft)
    ///
    pitchlfo_freqpolyaft(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [effect1](https://sfzformat.com/opcodes/effect1)
    ///
    effect1(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v1
    /// - info: [effect2](https://sfzformat.com/opcodes/effect2)
    ///
    effect2(f32),

    // sfz v2 opcodes -----------------------------------------------------------
    // https://sfzformat.com/misc/sfz2
    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_samples](https://sfzformat.com/opcodes/delay_samples)
    ///
    delay_samples(u32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_samples_onccN](https://sfzformat.com/opcodes/delay_samples_onccN)
    ///
    delay_samples_onccN(u32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_beats](https://sfzformat.com/opcodes/delay_beats)
    ///
    delay_beats(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [stop_beats](https://sfzformat.com/opcodes/stop_beats)
    ///
    stop_beats(f32),

    ///
    /// - range: forward, reverse
    /// - default: forward
    /// - version: v2
    /// - info: [direction](https://sfzformat.com/opcodes/direction)
    ///
    direction(String),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [loop_count](https://sfzformat.com/opcodes/loop_count)
    ///
    loop_count(UndefinedUnsignedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [loop_crossfade](https://sfzformat.com/opcodes/loop_crossfade)
    ///
    loop_crossfade(f32),

    ///
    /// - range: forward, backward, alternate
    /// - default: forward
    /// - version: v2
    /// - info: [loop_type](https://sfzformat.com/opcodes/loop_type)
    ///
    loop_type(String),

    ///
    /// - range: None
    /// - default: null
    /// - version: v2
    /// - info: [md5](https://sfzformat.com/opcodes/md5)
    ///
    md5(String),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v2
    /// - info: [reverse_loccN](https://sfzformat.com/opcodes/reverse_loccN)
    ///
    reverse_loccN(u8),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v2
    /// - info: [reverse_hiccN](https://sfzformat.com/opcodes/reverse_hiccN)
    ///
    reverse_hiccN(u8),

    ///
    /// - range: on, off
    /// - default: None
    /// - version: v2
    /// - info: [waveguide](https://sfzformat.com/opcodes/waveguide)
    ///
    waveguide(String),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [#define](https://sfzformat.com/opcodes/#define)
    ///
    define(String), // #define

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [default_path](https://sfzformat.com/opcodes/default_path)
    ///
    default_path(PathBuf),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [note_offset](https://sfzformat.com/opcodes/note_offset)
    ///
    note_offset(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [octave_offset](https://sfzformat.com/opcodes/octave_offset)
    ///
    octave_offset(UndefinedInteger),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v2
    /// - info: [set_ccN](https://sfzformat.com/opcodes/set_ccN)
    ///
    set_ccN(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [polyphony](https://sfzformat.com/opcodes/polyphony)
    ///
    polyphony(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [note_polyphony](https://sfzformat.com/opcodes/note_polyphony)
    ///
    note_polyphony(UndefinedInteger),

    ///
    /// - range: on, off
    /// - default: on
    /// - version: v2
    /// - info: [note_selfmask](https://sfzformat.com/opcodes/note_selfmask)
    ///
    note_selfmask(String),

    ///
    /// - range: on, off
    /// - default: off
    /// - version: v2
    /// - info: [rt_dead](https://sfzformat.com/opcodes/rt_dead)
    ///
    rt_dead(String),

    ///
    /// - range: on, off
    /// - default: None
    /// - version: v2
    /// - info: [sostenuto_sw](https://sfzformat.com/opcodes/sostenuto_sw)
    ///
    sostenuto_sw(String),

    ///
    /// - range: on, off
    /// - default: None
    /// - version: v2
    /// - info: [sustain_sw](https://sfzformat.com/opcodes/sustain_sw)
    ///
    sustain_sw(String),

    ///
    /// - range: 0 to 127
    /// - default: 0
    /// - version: v2
    /// - info: [loprog](https://sfzformat.com/opcodes/loprog)
    ///
    loprog(u8),

    ///
    /// - range: 0 to 127
    /// - default: 127
    /// - version: v2
    /// - info: [hiprog](https://sfzformat.com/opcodes/hiprog)
    ///
    hiprog(u8),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v2
    /// - info: [sw_default](https://sfzformat.com/opcodes/sw_default)
    ///
    sw_default(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lotimer](https://sfzformat.com/opcodes/lotimer)
    ///
    lotimer(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [hitimer](https://sfzformat.com/opcodes/hitimer)
    ///
    hitimer(f32),

    ///
    /// - range: 0 to 127
    /// - default: -1
    /// - version: v2
    /// - info: [start_loccN](https://sfzformat.com/opcodes/start_loccN)
    ///
    start_loccN(i8),

    ///
    /// - range: 0 to 127
    /// - default: -1
    /// - version: v2
    /// - info: [start_hiccN](https://sfzformat.com/opcodes/start_hiccN)
    ///
    start_hiccN(i8),

    ///
    /// - range: 0 to 127
    /// - default: -1
    /// - version: v2
    /// - info: [stop_loccN](https://sfzformat.com/opcodes/stop_loccN)
    ///
    stop_loccN(i8),

    ///
    /// - range: 0 to 127
    /// - default: -1
    /// - version: v2
    /// - info: [stop_hiccN](https://sfzformat.com/opcodes/stop_hiccN)
    ///
    stop_hiccN(i8),

    ///
    /// - range: normal, invert
    /// - default: normal
    /// - version: v2
    /// - info: [phase](https://sfzformat.com/opcodes/phase)
    ///
    phase(String),

    ///
    /// - range: 0 to 127
    /// - default: 60
    /// - version: v2
    /// - info: [pan_keycenter](https://sfzformat.com/opcodes/pan_keycenter)
    ///
    pan_keycenter(u8),

    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v2
    /// - info: [pan_keytrack](https://sfzformat.com/opcodes/pan_keytrack)
    ///
    pan_keytrack(f32),

    ///
    /// - range: -100 to 100 %
    /// - default: 0
    /// - version: v2
    /// - info: [pan_veltrack](https://sfzformat.com/opcodes/pan_veltrack)
    ///
    pan_veltrack(f32),

    ///
    /// - range: peak, lshelf, hshelf
    /// - default: peak
    /// - version: v2
    /// - info: [eqN_type](https://sfzformat.com/opcodes/eqN_type)
    ///
    eqN_type(String),

    ///
    /// - range: 0 to SampleRate / 2 Hz
    /// - default: filter disabled
    /// - version: v2
    /// - info: [cutoff2](https://sfzformat.com/opcodes/cutoff2)
    ///
    cutoff2(f32),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 0
    /// - version: v2
    /// - info: [cutoff2_onccN](https://sfzformat.com/opcodes/cutoff2_onccN)
    ///
    cutoff2_onccN(i16),

    ///
    /// - range: 0 to 255
    /// - default: None
    /// - version: v2
    /// - info: [cutoff2_curveccN](https://sfzformat.com/opcodes/cutoff2_curveccN)
    ///
    cutoff2_curveccN(u8),

    ///
    /// - range: 0 to ms
    /// - default: 0
    /// - version: v2
    /// - info: [cutoff2_smoothccN](https://sfzformat.com/opcodes/cutoff2_smoothccN)
    ///
    cutoff2_smoothccN(f32),

    ///
    /// - range: 0 to ...?
    /// - default: 0
    /// - version: v2
    /// - info: [cutoff2_stepccN](https://sfzformat.com/opcodes/cutoff2_stepccN)
    ///
    cutoff2_stepccN(UndefinedUnsignedInteger),

    ///
    /// - range: 0 to 127
    /// - default: 60
    /// - version: v2
    /// - info: [fil2_keycenter](https://sfzformat.com/opcodes/fil2_keycenter)
    ///
    fil2_keycenter(u8),

    ///
    /// - range: 0 to 1200 cents
    /// - default: 0
    /// - version: v2
    /// - info: [fil2_keytrack](https://sfzformat.com/opcodes/fil2_keytrack)
    ///
    fil2_keytrack(u16),

    ///
    /// - range: lpf_1p, hpf_1p, lpf_2p, hpf_2p, bpf_2p, brf_2p, bpf_1p, brf_1p, apf_1p, lpf_2p_sv, hpf_2p_sv, bpf_2p_sv, brf_2p_sv, pkf_2p, lpf_4p, hpf_4p, lpf_6p, hpf_6p, comb, pink
    /// - default: lpf_2p
    /// - version: v2
    /// - info: [fil2_type](https://sfzformat.com/opcodes/fil2_type)
    ///
    fil2_type(String),

    ///
    /// - range: -9600 to 9600 cents
    /// - default: 0
    /// - version: v2
    /// - info: [fil2_veltrack](https://sfzformat.com/opcodes/fil2_veltrack)
    ///
    fil2_veltrack(i16),

    ///
    /// - range: 0 to 40 dB
    /// - default: 0
    /// - version: v2
    /// - info: [resonance2](https://sfzformat.com/opcodes/resonance2)
    ///
    resonance2(f32),

    ///
    /// - range: 0 to 40 dB
    /// - default: 0
    /// - version: v2
    /// - info: [resonance2_onccN](https://sfzformat.com/opcodes/resonance2_onccN)
    ///
    resonance2_onccN(f32),

    ///
    /// - range: 0 to 255
    /// - default: None
    /// - version: v2
    /// - info: [resonance2_curveccN](https://sfzformat.com/opcodes/resonance2_curveccN)
    ///
    resonance2_curveccN(u8),

    ///
    /// - range: 0 to ms
    /// - default: 0
    /// - version: v2
    /// - info: [resonance2_smoothccN](https://sfzformat.com/opcodes/resonance2_smoothccN)
    ///
    resonance2_smoothccN(f32),

    ///
    /// - range: 0 to ...?
    /// - default: 0
    /// - version: v2
    /// - info: [resonance2_stepccN](https://sfzformat.com/opcodes/resonance2_stepccN)
    ///
    resonance2_stepccN(UndefinedUnsignedInteger),

    ///
    /// - range: 0 to ms
    /// - default: 0
    /// - version: v2
    /// - info: [bend_smooth](https://sfzformat.com/opcodes/bend_smooth)
    ///
    bend_smooth(f32),

    ///
    /// - range: 1 to 1200 cents
    /// - default: 1
    /// - version: v2
    /// - info: [bend_stepup](https://sfzformat.com/opcodes/bend_stepup)
    ///
    bend_stepup(u16),

    ///
    /// - range: 1 to 1200 cents
    /// - default: 1
    /// - version: v2
    /// - info: [bend_stepdown](https://sfzformat.com/opcodes/bend_stepdown)
    ///
    bend_stepdown(u16),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_points](https://sfzformat.com/opcodes/egN_points)
    ///
    egN_points(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_timeX](https://sfzformat.com/opcodes/egN_timeX)
    ///
    egN_timeX(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_timeX_onccY](https://sfzformat.com/opcodes/egN_timeX_onccY)
    ///
    egN_timeX_onccY(f32),

    ///
    /// - range: -1 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [egN_levelX](https://sfzformat.com/opcodes/egN_levelX)
    ///
    egN_levelX(f32),

    ///
    /// - range: -1 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [egN_levelX_onccY](https://sfzformat.com/opcodes/egN_levelX_onccY)
    ///
    egN_levelX_onccY(f32),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [egN_shapeX](https://sfzformat.com/opcodes/egN_shapeX)
    ///
    egN_shapeX(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_curveX](https://sfzformat.com/opcodes/egN_curveX)
    ///
    egN_curveX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_sustain](https://sfzformat.com/opcodes/egN_sustain)
    ///
    egN_sustain(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_loop](https://sfzformat.com/opcodes/egN_loop)
    ///
    egN_loop(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_loop_count](https://sfzformat.com/opcodes/egN_loop_count)
    ///
    egN_loop_count(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_volume](https://sfzformat.com/opcodes/egN_volume)
    ///
    egN_volume(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_volume_onccX](https://sfzformat.com/opcodes/egN_volume_onccX)
    ///
    egN_volume_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_amplitude](https://sfzformat.com/opcodes/egN_amplitude)
    ///
    egN_amplitude(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_amplitude_onccX](https://sfzformat.com/opcodes/egN_amplitude_onccX)
    ///
    egN_amplitude_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_pan](https://sfzformat.com/opcodes/egN_pan)
    ///
    egN_pan(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_pan_onccX](https://sfzformat.com/opcodes/egN_pan_onccX)
    ///
    egN_pan_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_width](https://sfzformat.com/opcodes/egN_width)
    ///
    egN_width(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_width_onccX](https://sfzformat.com/opcodes/egN_width_onccX)
    ///
    egN_width_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_pan_curve](https://sfzformat.com/opcodes/egN_pan_curve)
    ///
    egN_pan_curve(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_pan_curveccX](https://sfzformat.com/opcodes/egN_pan_curveccX)
    ///
    egN_pan_curveccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_freq_lfoX](https://sfzformat.com/opcodes/egN_freq_lfoX)
    ///
    egN_freq_lfoX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_depth_lfoX](https://sfzformat.com/opcodes/egN_depth_lfoX)
    ///
    egN_depth_lfoX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_depthadd_lfoX](https://sfzformat.com/opcodes/egN_depthadd_lfoX)
    ///
    egN_depthadd_lfoX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_pitch](https://sfzformat.com/opcodes/egN_pitch)
    ///
    egN_pitch(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_pitch_onccX](https://sfzformat.com/opcodes/egN_pitch_onccX)
    ///
    egN_pitch_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_cutoff](https://sfzformat.com/opcodes/egN_cutoff)
    ///
    egN_cutoff(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_cutoff_onccX](https://sfzformat.com/opcodes/egN_cutoff_onccX)
    ///
    egN_cutoff_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_cutoff2](https://sfzformat.com/opcodes/egN_cutoff2)
    ///
    egN_cutoff2(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_cutoff2_onccX](https://sfzformat.com/opcodes/egN_cutoff2_onccX)
    ///
    egN_cutoff2_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_resonance](https://sfzformat.com/opcodes/egN_resonance)
    ///
    egN_resonance(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_resonance_onccX](https://sfzformat.com/opcodes/egN_resonance_onccX)
    ///
    egN_resonance_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_resonance2](https://sfzformat.com/opcodes/egN_resonance2)
    ///
    egN_resonance2(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_resonance2_onccX](https://sfzformat.com/opcodes/egN_resonance2_onccX)
    ///
    egN_resonance2_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_eqXfreq](https://sfzformat.com/opcodes/egN_eqXfreq)
    ///
    egN_eqXfreq(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_eqXfreq_onccY](https://sfzformat.com/opcodes/egN_eqXfreq_onccY)
    ///
    egN_eqXfreq_onccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_eqXbw](https://sfzformat.com/opcodes/egN_eqXbw)
    ///
    egN_eqXbw(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_eqXbw_onccY](https://sfzformat.com/opcodes/egN_eqXbw_onccY)
    ///
    egN_eqXbw_onccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_eqXgain](https://sfzformat.com/opcodes/egN_eqXgain)
    ///
    egN_eqXgain(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_eqXgain_onccY](https://sfzformat.com/opcodes/egN_eqXgain_onccY)
    ///
    egN_eqXgain_onccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_freq](https://sfzformat.com/opcodes/lfoN_freq)
    ///
    lfoN_freq(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_freq_onccX](https://sfzformat.com/opcodes/lfoN_freq_onccX)
    ///
    lfoN_freq_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_freq_smoothccX](https://sfzformat.com/opcodes/lfoN_freq_smoothccX)
    ///
    lfoN_freq_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_freq_stepccX](https://sfzformat.com/opcodes/lfoN_freq_stepccX)
    ///
    lfoN_freq_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [lfoN_delay](https://sfzformat.com/opcodes/lfoN_delay)
    ///
    lfoN_delay(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_delay_onccX](https://sfzformat.com/opcodes/lfoN_delay_onccX)
    ///
    lfoN_delay_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_fade](https://sfzformat.com/opcodes/lfoN_fade)
    ///
    lfoN_fade(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_fade_onccX](https://sfzformat.com/opcodes/lfoN_fade_onccX)
    ///
    lfoN_fade_onccX(f32),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [lfoN_phase](https://sfzformat.com/opcodes/lfoN_phase)
    ///
    lfoN_phase(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_phase_onccX](https://sfzformat.com/opcodes/lfoN_phase_onccX)
    ///
    lfoN_phase_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_count](https://sfzformat.com/opcodes/lfoN_count)
    ///
    lfoN_count(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_wave](https://sfzformat.com/opcodes/lfoN_wave)
    ///
    lfoN_wave(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_steps](https://sfzformat.com/opcodes/lfoN_steps)
    ///
    lfoN_steps(UndefinedInteger),

    ///
    /// - range: -100 to 100 percent
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_stepX](https://sfzformat.com/opcodes/lfoN_stepX)
    ///
    lfoN_stepX(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_stepX_onccY](https://sfzformat.com/opcodes/lfoN_stepX_onccY)
    ///
    lfoN_stepX_onccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_smooth](https://sfzformat.com/opcodes/lfoN_smooth)
    ///
    lfoN_smooth(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_smooth_onccX](https://sfzformat.com/opcodes/lfoN_smooth_onccX)
    ///
    lfoN_smooth_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_volume](https://sfzformat.com/opcodes/lfoN_volume)
    ///
    lfoN_volume(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_volume_onccX](https://sfzformat.com/opcodes/lfoN_volume_onccX)
    ///
    lfoN_volume_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_volume_smoothccX](https://sfzformat.com/opcodes/lfoN_volume_smoothccX)
    ///
    lfoN_volume_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_volume_stepccX](https://sfzformat.com/opcodes/lfoN_volume_stepccX)
    ///
    lfoN_volume_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_amplitude](https://sfzformat.com/opcodes/lfoN_amplitude)
    ///
    lfoN_amplitude(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_amplitude_onccX](https://sfzformat.com/opcodes/lfoN_amplitude_onccX)
    ///
    lfoN_amplitude_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_amplitude_smoothccX](https://sfzformat.com/opcodes/lfoN_amplitude_smoothccX)
    ///
    lfoN_amplitude_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_amplitude_stepccX](https://sfzformat.com/opcodes/lfoN_amplitude_stepccX)
    ///
    lfoN_amplitude_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pan](https://sfzformat.com/opcodes/lfoN_pan)
    ///
    lfoN_pan(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pan_onccX](https://sfzformat.com/opcodes/lfoN_pan_onccX)
    ///
    lfoN_pan_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pan_smoothccX](https://sfzformat.com/opcodes/lfoN_pan_smoothccX)
    ///
    lfoN_pan_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pan_stepccX](https://sfzformat.com/opcodes/lfoN_pan_stepccX)
    ///
    lfoN_pan_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_width](https://sfzformat.com/opcodes/lfoN_width)
    ///
    lfoN_width(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_width_onccX](https://sfzformat.com/opcodes/lfoN_width_onccX)
    ///
    lfoN_width_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_width_smoothccX](https://sfzformat.com/opcodes/lfoN_width_smoothccX)
    ///
    lfoN_width_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_width_stepccX](https://sfzformat.com/opcodes/lfoN_width_stepccX)
    ///
    lfoN_width_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_freq_lfoX](https://sfzformat.com/opcodes/lfoN_freq_lfoX)
    ///
    lfoN_freq_lfoX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_depth_lfoX](https://sfzformat.com/opcodes/lfoN_depth_lfoX)
    ///
    lfoN_depth_lfoX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_depthadd_lfoX](https://sfzformat.com/opcodes/lfoN_depthadd_lfoX)
    ///
    lfoN_depthadd_lfoX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pitch](https://sfzformat.com/opcodes/lfoN_pitch)
    ///
    lfoN_pitch(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pitch_onccX](https://sfzformat.com/opcodes/lfoN_pitch_onccX)
    ///
    lfoN_pitch_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pitch_smoothccX](https://sfzformat.com/opcodes/lfoN_pitch_smoothccX)
    ///
    lfoN_pitch_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_pitch_stepccX](https://sfzformat.com/opcodes/lfoN_pitch_stepccX)
    ///
    lfoN_pitch_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff](https://sfzformat.com/opcodes/lfoN_cutoff)
    ///
    lfoN_cutoff(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff_onccX](https://sfzformat.com/opcodes/lfoN_cutoff_onccX)
    ///
    lfoN_cutoff_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff_smoothccX](https://sfzformat.com/opcodes/lfoN_cutoff_smoothccX)
    ///
    lfoN_cutoff_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff_stepccX](https://sfzformat.com/opcodes/lfoN_cutoff_stepccX)
    ///
    lfoN_cutoff_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff2](https://sfzformat.com/opcodes/lfoN_cutoff2)
    ///
    lfoN_cutoff2(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff2_onccX](https://sfzformat.com/opcodes/lfoN_cutoff2_onccX)
    ///
    lfoN_cutoff2_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff2_smoothccX](https://sfzformat.com/opcodes/lfoN_cutoff2_smoothccX)
    ///
    lfoN_cutoff2_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_cutoff2_stepccX](https://sfzformat.com/opcodes/lfoN_cutoff2_stepccX)
    ///
    lfoN_cutoff2_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance](https://sfzformat.com/opcodes/lfoN_resonance)
    ///
    lfoN_resonance(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance_onccX](https://sfzformat.com/opcodes/lfoN_resonance_onccX)
    ///
    lfoN_resonance_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance_smoothccX](https://sfzformat.com/opcodes/lfoN_resonance_smoothccX)
    ///
    lfoN_resonance_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance_stepccX](https://sfzformat.com/opcodes/lfoN_resonance_stepccX)
    ///
    lfoN_resonance_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance2](https://sfzformat.com/opcodes/lfoN_resonance2)
    ///
    lfoN_resonance2(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance2_onccX](https://sfzformat.com/opcodes/lfoN_resonance2_onccX)
    ///
    lfoN_resonance2_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance2_smoothccX](https://sfzformat.com/opcodes/lfoN_resonance2_smoothccX)
    ///
    lfoN_resonance2_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_resonance2_stepccX](https://sfzformat.com/opcodes/lfoN_resonance2_stepccX)
    ///
    lfoN_resonance2_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXfreq](https://sfzformat.com/opcodes/lfoN_eqXfreq)
    ///
    lfoN_eqXfreq(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXfreq_onccY](https://sfzformat.com/opcodes/lfoN_eqXfreq_onccY)
    ///
    lfoN_eqXfreq_onccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXfreq_smoothccY](https://sfzformat.com/opcodes/lfoN_eqXfreq_smoothccY)
    ///
    lfoN_eqXfreq_smoothccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXfreq_stepccY](https://sfzformat.com/opcodes/lfoN_eqXfreq_stepccY)
    ///
    lfoN_eqXfreq_stepccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXbw](https://sfzformat.com/opcodes/lfoN_eqXbw)
    ///
    lfoN_eqXbw(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXbw_onccY](https://sfzformat.com/opcodes/lfoN_eqXbw_onccY)
    ///
    lfoN_eqXbw_onccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXbw_smoothccY](https://sfzformat.com/opcodes/lfoN_eqXbw_smoothccY)
    ///
    lfoN_eqXbw_smoothccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXbw_stepccY](https://sfzformat.com/opcodes/lfoN_eqXbw_stepccY)
    ///
    lfoN_eqXbw_stepccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXgain](https://sfzformat.com/opcodes/lfoN_eqXgain)
    ///
    lfoN_eqXgain(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXgain_onccY](https://sfzformat.com/opcodes/lfoN_eqXgain_onccY)
    ///
    lfoN_eqXgain_onccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXgain_smoothccY](https://sfzformat.com/opcodes/lfoN_eqXgain_smoothccY)
    ///
    lfoN_eqXgain_smoothccY(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_eqXgain_stepccY](https://sfzformat.com/opcodes/lfoN_eqXgain_stepccY)
    ///
    lfoN_eqXgain_stepccY(UnknownType),

    ///
    /// - range: -1 to 1
    /// - default: None
    /// - version: v2
    /// - info: [vN](https://sfzformat.com/opcodes/vN)
    ///
    vN(f32),

    ///
    /// - range: main, aux1, aux2, aux3, aux4, aux5, aux6, aux7, aux8, fx1, fx2, fx3, fx4, midi
    /// - default: main
    /// - version: v2
    /// - info: [bus](https://sfzformat.com/opcodes/bus)
    ///
    bus(String),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v2
    /// - info: [effect3](https://sfzformat.com/opcodes/effect3)
    ///
    effect3(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v2
    /// - info: [effect4](https://sfzformat.com/opcodes/effect4)
    ///
    effect4(f32),

    ///
    /// - range: apan, comp, delay, disto, eq, filter, fverb, gate, limiter, lofi, mverb, phaser, static, strings, tdfir, com.mda.Limiter, com.mda.Overdrive, com.mda.Leslie, com.mda.RingMod, com.mda.Delay, com.mda.Bandisto, com.mda.Ambience, com.mda.DubDelay, com.mda.Detune, com.mda.Dither, com.mda.Combo, com.mda.Degrade, com.mda.SubSynth, com.mda.RezFilter
    /// - default: None
    /// - version: v2
    /// - info: [type](https://sfzformat.com/opcodes/type)
    ///
    r#type(String),

    // aria extension opcodes --------------------------------------------------
    // https://sfzformat.com/extensions/aria/
    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [label_ccN](https://sfzformat.com/opcodes/label_ccN)
    ///
    label_ccN(String),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [#include](https://sfzformat.com/opcodes/#include)
    ///
    include(String),

    /// ARIA supports specific opcodes in ‹control› which start with “hint”,
    /// these should be ignored by any other SFZ parser.
    /// Other engines could implement other hints as they wished.
    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [hint_*](https://sfzformat.com/opcodes/hint_)
    ///
    hint_(),

    /// Determines whether a parameter is modulated by addition or multiplication.
    ///
    /// - range: add, mult
    /// - default: None
    /// - version: v2
    /// - info: [*_mod](https://sfzformat.com/opcodes/_mod)
    ///
    _mod(String),

    ///
    /// - range: 0 to 1
    /// - default: None
    /// - version: v2
    /// - info: [set_hdccN](https://sfzformat.com/opcodes/set_hdccN)
    ///
    set_hdccN(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [sw_note_offset](https://sfzformat.com/opcodes/sw_note_offset)
    ///
    sw_note_offset(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [sw_octave_offset](https://sfzformat.com/opcodes/sw_octave_offset)
    ///
    sw_octave_offset(UndefinedInteger),

    ///
    /// - range: None
    /// - default: null
    /// - version: v2
    /// - info: [global_label](https://sfzformat.com/opcodes/global_label)
    ///
    global_label(String),

    ///
    /// - range: None
    /// - default: null
    /// - version: v2
    /// - info: [master_label](https://sfzformat.com/opcodes/master_label)
    ///
    master_label(String),

    ///
    /// - range: None
    /// - default: null
    /// - version: v2
    /// - info: [group_label](https://sfzformat.com/opcodes/group_label)
    ///
    group_label(String),

    ///
    /// - range: None
    /// - default: null
    /// - version: v2
    /// - info: [region_label](https://sfzformat.com/opcodes/region_label)
    ///
    region_label(String),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [polyphony_stealing](https://sfzformat.com/opcodes/polyphony_stealing)
    ///
    polyphony_stealing(UndefinedInteger),

    ///
    /// - range: -2 to 10
    /// - default: 10
    /// - version: v2
    /// - info: [off_curve](https://sfzformat.com/opcodes/off_curve)
    ///
    off_curve(i8),

    ///
    /// - range: None
    /// - default: -103616
    /// - version: v2
    /// - info: [off_shape](https://sfzformat.com/opcodes/off_shape)
    ///
    off_shape(f32),

    ///
    /// - range: None
    /// - default: 6
    /// - version: v2
    /// - info: [off_time](https://sfzformat.com/opcodes/off_time)
    ///
    off_time(f32),

    ///
    /// - range: 0 to u32::MAX
    /// - default: 0
    /// - version: v2
    /// - info: [polyphony_group](https://sfzformat.com/opcodes/polyphony_group)
    ///
    polyphony_group(u32),

    ///
    /// - range: 0 to 127
    /// - default: 66
    /// - version: v2
    /// - info: [sostenuto_cc](https://sfzformat.com/opcodes/sostenuto_cc)
    ///
    sostenuto_cc(f32),

    ///
    /// - range: 0 to 127
    /// - default: 0.5
    /// - version: v2
    /// - info: [sostenuto_lo](https://sfzformat.com/opcodes/sostenuto_lo)
    ///
    sostenuto_lo(f32),

    ///
    /// - range: 0 to 127
    /// - default: 64
    /// - version: v2
    /// - info: [sustain_cc](https://sfzformat.com/opcodes/sustain_cc)
    ///
    sustain_cc(f32),

    ///
    /// - range: 0 to 127
    /// - default: 0.5
    /// - version: v2
    /// - info: [sustain_lo](https://sfzformat.com/opcodes/sustain_lo)
    ///
    sustain_lo(f32),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [lohdccN](https://sfzformat.com/opcodes/lohdccN)
    ///
    lohdccN(f32),

    ///
    /// - range: 0 to 1
    /// - default: 1
    /// - version: v2
    /// - info: [hihdccN](https://sfzformat.com/opcodes/hihdccN)
    ///
    hihdccN(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [sw_label](https://sfzformat.com/opcodes/sw_label)
    ///
    sw_label(String),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v2
    /// - info: [sw_lolast](https://sfzformat.com/opcodes/sw_lolast)
    ///
    sw_lolast(u8),

    ///
    /// - range: 0 to 127
    /// - default: None
    /// - version: v2
    /// - info: [sw_hilast](https://sfzformat.com/opcodes/sw_hilast)
    ///
    sw_hilast(u8),

    ///
    /// - range: mult, add
    /// - default: None
    /// - version: v2
    /// - info: [varNN_mod](https://sfzformat.com/opcodes/varNN_mod)
    ///
    varNN_mod(String),

    ///
    /// - range: 0 to 1
    /// - default: None
    /// - version: v2
    /// - info: [varNN_onccX](https://sfzformat.com/opcodes/varNN_onccX)
    ///
    varNN_onccX(f32),

    ///
    /// - range: 0 to 255
    /// - default: None
    /// - version: v2
    /// - info: [varNN_curveccX](https://sfzformat.com/opcodes/varNN_curveccX)
    ///
    varNN_curveccX(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [varNN_target](https://sfzformat.com/opcodes/varNN_target)
    ///
    varNN_target(),

    ///
    /// - range: 0 to 1
    /// - default: -1
    /// - version: v2
    /// - info: [on_lohdccN](https://sfzformat.com/opcodes/on_lohdccN)
    ///
    on_lohdccN(f32),

    ///
    /// - range: 0 to 1
    /// - default: -1
    /// - version: v2
    /// - info: [on_hihdccN](https://sfzformat.com/opcodes/on_hihdccN)
    ///
    on_hihdccN(f32),

    ///
    /// - range: 0 to 1
    /// - default: -1
    /// - version: v2
    /// - info: [start_lohdccN](https://sfzformat.com/opcodes/start_lohdccN)
    ///
    start_lohdccN(f32),

    ///
    /// - range: 0 to 1
    /// - default: -1
    /// - version: v2
    /// - info: [start_hihdccN](https://sfzformat.com/opcodes/start_hihdccN)
    ///
    start_hihdccN(f32),

    ///
    /// - range: 0 to 1
    /// - default: -1
    /// - version: v2
    /// - info: [stop_lohdccN](https://sfzformat.com/opcodes/stop_lohdccN)
    ///
    stop_lohdccN(f32),

    ///
    /// - range: 0 to 1
    /// - default: -1
    /// - version: v2
    /// - info: [stop_hihdccN](https://sfzformat.com/opcodes/stop_hihdccN)
    ///
    stop_hihdccN(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [position_veltrack](https://sfzformat.com/opcodes/position_veltrack)
    ///
    position_veltrack(),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [amp_veltrack_random](https://sfzformat.com/opcodes/amp_veltrack_random)
    ///
    amp_veltrack_random(),

    ///
    /// - range: 0 to 100 %
    /// - default: 100
    /// - version: v2
    /// - info: [amplitude](https://sfzformat.com/opcodes/amplitude)
    ///
    amplitude(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: None
    /// - version: v2
    /// - info: [amplitude_onccN](https://sfzformat.com/opcodes/amplitude_onccN)
    ///
    amplitude_onccN(f32),

    ///
    /// - range: 0 to 255
    /// - default: None
    /// - version: v2
    /// - info: [amplitude_curveccN](https://sfzformat.com/opcodes/amplitude_curveccN)
    ///
    amplitude_curveccN(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [amplitude_smoothccN](https://sfzformat.com/opcodes/amplitude_smoothccN)
    ///
    amplitude_smoothccN(),

    ///
    /// - range: 0 to 100 %
    /// - default: 100
    /// - version: v2
    /// - info: [global_amplitude](https://sfzformat.com/opcodes/global_amplitude)
    ///
    global_amplitude(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 100
    /// - version: v2
    /// - info: [master_amplitude](https://sfzformat.com/opcodes/master_amplitude)
    ///
    master_amplitude(f32),

    ///
    /// - range: 0 to 100 %
    /// - default: 100
    /// - version: v2
    /// - info: [group_amplitude](https://sfzformat.com/opcodes/group_amplitude)
    ///
    group_amplitude(f32),

    ///
    /// - range: mma, balance
    /// - default: None
    /// - version: v2
    /// - info: [pan_law](https://sfzformat.com/opcodes/pan_law)
    ///
    pan_law(String),

    ///
    /// - range: -144 to 6 dB
    /// - default: 0
    /// - version: v2
    /// - info: [global_volume](https://sfzformat.com/opcodes/global_volume)
    ///
    global_volume(f32),

    ///
    /// - range: -144 to 6 dB
    /// - default: 0
    /// - version: v2
    /// - info: [master_volume](https://sfzformat.com/opcodes/master_volume)
    ///
    master_volume(f32),

    ///
    /// - range: -144 to 6 dB
    /// - default: 0
    /// - version: v2
    /// - info: [group_volume](https://sfzformat.com/opcodes/group_volume)
    ///
    group_volume(f32),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [eqN_dynamic](https://sfzformat.com/opcodes/eqN_dynamic)
    ///
    eqN_dynamic(u8),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [fil_gain](https://sfzformat.com/opcodes/fil_gain)
    ///
    fil_gain(f32),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [fil2_gain](https://sfzformat.com/opcodes/fil2_gain)
    ///
    fil2_gain(f32),

    ///
    /// - range: -100 to 100 cents
    /// - default: 0
    /// - version: v2
    /// - info: [pitch](https://sfzformat.com/opcodes/pitch)
    ///
    pitch(i8),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [ampeg_attack_shape](https://sfzformat.com/opcodes/ampeg_attack_shape)
    ///
    ampeg_attack_shape(f32),

    ///
    /// - range: None
    /// - default: -103616
    /// - version: v2
    /// - info: [ampeg_decay_shape](https://sfzformat.com/opcodes/ampeg_decay_shape)
    ///
    ampeg_decay_shape(f32),

    ///
    /// - range: 0 to 1
    /// - default: 1
    /// - version: v2
    /// - info: [ampeg_decay_zero](https://sfzformat.com/opcodes/ampeg_decay_zero)
    ///
    ampeg_decay_zero(u8),

    /// When 1, causes envelope durations to be recalculated when a MIDI CC
    /// message modulating those envelopes is received.
    /// When 0, envelope durations are calculated only at the start of the note.
    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [ampeg_dynamic](https://sfzformat.com/opcodes/ampeg_dynamic)
    ///
    ampeg_dynamic(u8),

    ///
    /// - range: None
    /// - default: -103616
    /// - version: v2
    /// - info: [ampeg_release_shape](https://sfzformat.com/opcodes/ampeg_release_shape)
    ///
    ampeg_release_shape(f32),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [ampeg_release_zero](https://sfzformat.com/opcodes/ampeg_release_zero)
    ///
    ampeg_release_zero(u8),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [fileg_attack_shape](https://sfzformat.com/opcodes/fileg_attack_shape)
    ///
    fileg_attack_shape(f32),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [fileg_decay_shape](https://sfzformat.com/opcodes/fileg_decay_shape)
    ///
    fileg_decay_shape(f32),

    ///
    /// - range: 0 to 1
    /// - default: 1
    /// - version: v2
    /// - info: [fileg_decay_zero](https://sfzformat.com/opcodes/fileg_decay_zero)
    ///
    fileg_decay_zero(u8),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [fileg_release_shape](https://sfzformat.com/opcodes/fileg_release_shape)
    ///
    fileg_release_shape(f32),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [fileg_release_zero](https://sfzformat.com/opcodes/fileg_release_zero)
    ///
    fileg_release_zero(u8),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [fileg_dynamic](https://sfzformat.com/opcodes/fileg_dynamic)
    ///
    fileg_dynamic(u8),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [pitcheg_attack_shape](https://sfzformat.com/opcodes/pitcheg_attack_shape)
    ///
    pitcheg_attack_shape(f32),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [pitcheg_decay_shape](https://sfzformat.com/opcodes/pitcheg_decay_shape)
    ///
    pitcheg_decay_shape(f32),

    ///
    /// - range: 0 to 1
    /// - default: 1
    /// - version: v2
    /// - info: [pitcheg_decay_zero](https://sfzformat.com/opcodes/pitcheg_decay_zero)
    ///
    pitcheg_decay_zero(u8),

    ///
    /// - range: None
    /// - default: 0
    /// - version: v2
    /// - info: [pitcheg_release_shape](https://sfzformat.com/opcodes/pitcheg_release_shape)
    ///
    pitcheg_release_shape(f32),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [pitcheg_release_zero](https://sfzformat.com/opcodes/pitcheg_release_zero)
    ///
    pitcheg_release_zero(u8),

    ///
    /// - range: 0 to 1
    /// - default: 0
    /// - version: v2
    /// - info: [pitcheg_dynamic](https://sfzformat.com/opcodes/pitcheg_dynamic)
    ///
    pitcheg_dynamic(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_ampeg](https://sfzformat.com/opcodes/egN_ampeg)
    ///
    egN_ampeg(UnknownType),

    ///
    /// - range: None
    /// - default: 1
    /// - version: v2
    /// - info: [lfoN_waveX](https://sfzformat.com/opcodes/lfoN_waveX)
    ///
    lfoN_waveX(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_offset](https://sfzformat.com/opcodes/lfoN_offset)
    ///
    lfoN_offset(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_ratio](https://sfzformat.com/opcodes/lfoN_ratio)
    ///
    lfoN_ratio(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_scale](https://sfzformat.com/opcodes/lfoN_scale)
    ///
    lfoN_scale(f32),

    ///
    /// - range: 0 to 255
    /// - default: None
    /// - version: v2
    /// - info: [curve_index](https://sfzformat.com/opcodes/curve_index)
    ///
    curve_index(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [param_offset](https://sfzformat.com/opcodes/param_offset)
    ///
    param_offset(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [vendor_specific](https://sfzformat.com/opcodes/vendor_specific)
    ///
    vendor_specific(String),

    // Cakewalk SFZ v2 Opcodes -----------------------------------------------
    // https://sfzformat.com/misc/cakewalk
    ///
    /// - range: on, off, lpf_1p, hpf_1p, bpf_1p, brf_1p, apf_1p, lpf_2p, hpf_2p,
    ///   bpf_2p, brf_2p, pkf_2p, lpf_4p, hpf_4p, lpf_6p, hpf_6p, comb, pink
    /// - default: None
    /// - version: v2
    /// - info: [noise_filter](https://sfzformat.com/opcodes/noise_filter)
    ///
    noise_filter(String),

    ///
    /// - range: on, off
    /// - default: None
    /// - version: v2
    /// - info: [noise_stereo](https://sfzformat.com/opcodes/noise_stereo)
    ///
    noise_stereo(String),

    ///
    /// - range: -96 to 24 dB
    /// - default: None
    /// - version: v2
    /// - info: [noise_level](https://sfzformat.com/opcodes/noise_level)
    ///
    noise_level(f32),

    ///
    /// - range: -96 to 24 dB
    /// - default: None
    /// - version: v2
    /// - info: [noise_level_onccN](https://sfzformat.com/opcodes/noise_level_onccN)
    ///
    noise_level_onccN(f32),

    ///
    /// - range: 0 to ms
    /// - default: 0
    /// - version: v2
    /// - info: [noise_level_smoothccN](https://sfzformat.com/opcodes/noise_level_smoothccN)
    ///
    noise_level_smoothccN(f32),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [noise_step](https://sfzformat.com/opcodes/noise_step)
    ///
    noise_step(u8),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [noise_step_onccN](https://sfzformat.com/opcodes/noise_step_onccN)
    ///
    noise_step_onccN(u8),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [noise_tone](https://sfzformat.com/opcodes/noise_tone)
    ///
    noise_tone(u8),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [noise_tone_onccN](https://sfzformat.com/opcodes/noise_tone_onccN)
    ///
    noise_tone_onccN(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_decim](https://sfzformat.com/opcodes/egN_decim)
    ///
    egN_decim(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_decim_onccX](https://sfzformat.com/opcodes/egN_decim_onccX)
    ///
    egN_decim_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_bitred](https://sfzformat.com/opcodes/egN_bitred)
    ///
    egN_bitred(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_bitred_onccX](https://sfzformat.com/opcodes/egN_bitred_onccX)
    ///
    egN_bitred_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_rectify](https://sfzformat.com/opcodes/egN_rectify)
    ///
    egN_rectify(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_rectify_onccX](https://sfzformat.com/opcodes/egN_rectify_onccX)
    ///
    egN_rectify_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_ringmod](https://sfzformat.com/opcodes/egN_ringmod)
    ///
    egN_ringmod(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_ringmod_onccX](https://sfzformat.com/opcodes/egN_ringmod_onccX)
    ///
    egN_ringmod_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_noiselevel](https://sfzformat.com/opcodes/egN_noiselevel)
    ///
    egN_noiselevel(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_noiselevel_onccX](https://sfzformat.com/opcodes/egN_noiselevel_onccX)
    ///
    egN_noiselevel_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_noisestep](https://sfzformat.com/opcodes/egN_noisestep)
    ///
    egN_noisestep(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_noisestep_onccX](https://sfzformat.com/opcodes/egN_noisestep_onccX)
    ///
    egN_noisestep_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_noisetone](https://sfzformat.com/opcodes/egN_noisetone)
    ///
    egN_noisetone(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_noisetone_onccX](https://sfzformat.com/opcodes/egN_noisetone_onccX)
    ///
    egN_noisetone_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_driveshape](https://sfzformat.com/opcodes/egN_driveshape)
    ///
    egN_driveshape(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [egN_driveshape_onccX](https://sfzformat.com/opcodes/egN_driveshape_onccX)
    ///
    egN_driveshape_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_decim](https://sfzformat.com/opcodes/lfoN_decim)
    ///
    lfoN_decim(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_decim_onccX](https://sfzformat.com/opcodes/lfoN_decim_onccX)
    ///
    lfoN_decim_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_decim_smoothccX](https://sfzformat.com/opcodes/lfoN_decim_smoothccX)
    ///
    lfoN_decim_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_decim_stepccX](https://sfzformat.com/opcodes/lfoN_decim_stepccX)
    ///
    lfoN_decim_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_bitred](https://sfzformat.com/opcodes/lfoN_bitred)
    ///
    lfoN_bitred(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_bitred_onccX](https://sfzformat.com/opcodes/lfoN_bitred_onccX)
    ///
    lfoN_bitred_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_bitred_smoothccX](https://sfzformat.com/opcodes/lfoN_bitred_smoothccX)
    ///
    lfoN_bitred_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_bitred_stepccX](https://sfzformat.com/opcodes/lfoN_bitred_stepccX)
    ///
    lfoN_bitred_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noiselevel](https://sfzformat.com/opcodes/lfoN_noiselevel)
    ///
    lfoN_noiselevel(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noiselevel_onccX](https://sfzformat.com/opcodes/lfoN_noiselevel_onccX)
    ///
    lfoN_noiselevel_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noiselevel_smoothccX](https://sfzformat.com/opcodes/lfoN_noiselevel_smoothccX)
    ///
    lfoN_noiselevel_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noiselevel_stepccX](https://sfzformat.com/opcodes/lfoN_noiselevel_stepccX)
    ///
    lfoN_noiselevel_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisestep](https://sfzformat.com/opcodes/lfoN_noisestep)
    ///
    lfoN_noisestep(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisestep_onccX](https://sfzformat.com/opcodes/lfoN_noisestep_onccX)
    ///
    lfoN_noisestep_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisestep_smoothccX](https://sfzformat.com/opcodes/lfoN_noisestep_smoothccX)
    ///
    lfoN_noisestep_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisestep_stepccX](https://sfzformat.com/opcodes/lfoN_noisestep_stepccX)
    ///
    lfoN_noisestep_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisetone](https://sfzformat.com/opcodes/lfoN_noisetone)
    ///
    lfoN_noisetone(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisetone_onccX](https://sfzformat.com/opcodes/lfoN_noisetone_onccX)
    ///
    lfoN_noisetone_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisetone_smoothccX](https://sfzformat.com/opcodes/lfoN_noisetone_smoothccX)
    ///
    lfoN_noisetone_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_noisetone_stepccX](https://sfzformat.com/opcodes/lfoN_noisetone_stepccX)
    ///
    lfoN_noisetone_stepccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_drive](https://sfzformat.com/opcodes/lfoN_drive)
    ///
    lfoN_drive(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_drive_onccX](https://sfzformat.com/opcodes/lfoN_drive_onccX)
    ///
    lfoN_drive_onccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_drive_smoothccX](https://sfzformat.com/opcodes/lfoN_drive_smoothccX)
    ///
    lfoN_drive_smoothccX(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [lfoN_drive_stepccX](https://sfzformat.com/opcodes/lfoN_drive_stepccX)
    ///
    lfoN_drive_stepccX(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [apan_depth](https://sfzformat.com/opcodes/apan_depth)
    ///
    apan_depth(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [apan_dry](https://sfzformat.com/opcodes/apan_dry)
    ///
    apan_dry(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [apan_freq](https://sfzformat.com/opcodes/apan_freq)
    ///
    apan_freq(f32),

    ///
    /// - range: 0 to 180 °
    /// - default: None
    /// - version: v2
    /// - info: [apan_phase](https://sfzformat.com/opcodes/apan_phase)
    ///
    apan_phase(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [apan_waveform](https://sfzformat.com/opcodes/apan_waveform)
    ///
    apan_waveform(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [apan_wet](https://sfzformat.com/opcodes/apan_wet)
    ///
    apan_wet(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [bitred](https://sfzformat.com/opcodes/bitred)
    ///
    bitred(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [bitred_onccN](https://sfzformat.com/opcodes/bitred_onccN)
    ///
    bitred_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [bitred_curveccN](https://sfzformat.com/opcodes/bitred_curveccN)
    ///
    bitred_curveccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [bitred_smoothccN](https://sfzformat.com/opcodes/bitred_smoothccN)
    ///
    bitred_smoothccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [bitred_stepccN](https://sfzformat.com/opcodes/bitred_stepccN)
    ///
    bitred_stepccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [comp_attack](https://sfzformat.com/opcodes/comp_attack)
    ///
    comp_attack(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [comp_gain](https://sfzformat.com/opcodes/comp_gain)
    ///
    comp_gain(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [comp_ratio](https://sfzformat.com/opcodes/comp_ratio)
    ///
    comp_ratio(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [comp_release](https://sfzformat.com/opcodes/comp_release)
    ///
    comp_release(f32),

    ///
    /// - range: on, off
    /// - default: None
    /// - version: v2
    /// - info: [comp_stlink](https://sfzformat.com/opcodes/comp_stlink)
    ///
    comp_stlink(String),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [comp_threshold](https://sfzformat.com/opcodes/comp_threshold)
    ///
    comp_threshold(f32),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [decim](https://sfzformat.com/opcodes/decim)
    ///
    decim(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [decim_onccN](https://sfzformat.com/opcodes/decim_onccN)
    ///
    decim_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [decim_curveccN](https://sfzformat.com/opcodes/decim_curveccN)
    ///
    decim_curveccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [decim_smoothccN](https://sfzformat.com/opcodes/decim_smoothccN)
    ///
    decim_smoothccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [decim_stepccN](https://sfzformat.com/opcodes/decim_stepccN)
    ///
    decim_stepccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_cutoff](https://sfzformat.com/opcodes/delay_cutoff)
    ///
    delay_cutoff(f32),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_damphi](https://sfzformat.com/opcodes/delay_damphi)
    ///
    delay_damphi(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_damplo](https://sfzformat.com/opcodes/delay_damplo)
    ///
    delay_damplo(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_dry](https://sfzformat.com/opcodes/delay_dry)
    ///
    delay_dry(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_feedback](https://sfzformat.com/opcodes/delay_feedback)
    ///
    delay_feedback(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_filter](https://sfzformat.com/opcodes/delay_filter)
    ///
    delay_filter(String),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_input](https://sfzformat.com/opcodes/delay_input)
    ///
    delay_input(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_levelc](https://sfzformat.com/opcodes/delay_levelc)
    ///
    delay_levelc(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_levell](https://sfzformat.com/opcodes/delay_levell)
    ///
    delay_levell(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_levelr](https://sfzformat.com/opcodes/delay_levelr)
    ///
    delay_levelr(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_lfofreq](https://sfzformat.com/opcodes/delay_lfofreq)
    ///
    delay_lfofreq(f32),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_moddepth](https://sfzformat.com/opcodes/delay_moddepth)
    ///
    delay_moddepth(UnknownType),

    ///
    /// - range: detune, chorus, cross, flanger, lrc, mod, multimod, panning, ping, rlc, stereo, tlcr
    /// - default: None
    /// - version: v2
    /// - info: [delay_mode](https://sfzformat.com/opcodes/delay_mode)
    ///
    delay_mode(String),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_panc](https://sfzformat.com/opcodes/delay_panc)
    ///
    delay_panc(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_panl](https://sfzformat.com/opcodes/delay_panl)
    ///
    delay_panl(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_panr](https://sfzformat.com/opcodes/delay_panr)
    ///
    delay_panr(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_resonance](https://sfzformat.com/opcodes/delay_resonance)
    ///
    delay_resonance(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_spread](https://sfzformat.com/opcodes/delay_spread)
    ///
    delay_spread(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_syncc_onccN](https://sfzformat.com/opcodes/delay_syncc_onccN)
    ///
    delay_syncc_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_syncl_onccN](https://sfzformat.com/opcodes/delay_syncl_onccN)
    ///
    delay_syncl_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_syncr_onccN](https://sfzformat.com/opcodes/delay_syncr_onccN)
    ///
    delay_syncr_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_time_tap](https://sfzformat.com/opcodes/delay_time_tap)
    ///
    delay_time_tap(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_timec](https://sfzformat.com/opcodes/delay_timec)
    ///
    delay_timec(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_timel](https://sfzformat.com/opcodes/delay_timel)
    ///
    delay_timel(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [delay_timer](https://sfzformat.com/opcodes/delay_timer)
    ///
    delay_timer(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [delay_wet](https://sfzformat.com/opcodes/delay_wet)
    ///
    delay_wet(UnknownType),

    ///
    /// - range: 0 to 100 %
    /// - default: 100
    /// - version: v2
    /// - info: [directtomain](https://sfzformat.com/opcodes/directtomain)
    ///
    directtomain(f32),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [disto_depth](https://sfzformat.com/opcodes/disto_depth)
    ///
    disto_depth(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [disto_dry](https://sfzformat.com/opcodes/disto_dry)
    ///
    disto_dry(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [disto_stages](https://sfzformat.com/opcodes/disto_stages)
    ///
    disto_stages(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [disto_tone](https://sfzformat.com/opcodes/disto_tone)
    ///
    disto_tone(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [disto_wet](https://sfzformat.com/opcodes/disto_wet)
    ///
    disto_wet(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [eq_bw](https://sfzformat.com/opcodes/eq_bw)
    ///
    eq_bw(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [eq_freq](https://sfzformat.com/opcodes/eq_freq)
    ///
    eq_freq(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [eq_gain](https://sfzformat.com/opcodes/eq_gain)
    ///
    eq_gain(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [eq_type](https://sfzformat.com/opcodes/eq_type)
    ///
    eq_type(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [filter_cutoff](https://sfzformat.com/opcodes/filter_cutoff)
    ///
    filter_cutoff(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [filter_resonance](https://sfzformat.com/opcodes/filter_resonance)
    ///
    filter_resonance(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [filter_type](https://sfzformat.com/opcodes/filter_type)
    ///
    filter_type(String),

    ///
    /// - range: 0 to 100 %
    /// - default: 0
    /// - version: v2
    /// - info: [fxNtomain](https://sfzformat.com/opcodes/fxNtomain)
    ///
    fxNtomain(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [gate_onccN](https://sfzformat.com/opcodes/gate_onccN)
    ///
    gate_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [gate_attack](https://sfzformat.com/opcodes/gate_attack)
    ///
    gate_attack(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [gate_release](https://sfzformat.com/opcodes/gate_release)
    ///
    gate_release(UnknownType),

    ///
    /// - range: on, off
    /// - default: None
    /// - version: v2
    /// - info: [gate_stlink](https://sfzformat.com/opcodes/gate_stlink)
    ///
    gate_stlink(String),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [gate_threshold](https://sfzformat.com/opcodes/gate_threshold)
    ///
    gate_threshold(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [phaser_depth](https://sfzformat.com/opcodes/phaser_depth)
    ///
    phaser_depth(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [phaser_feedback](https://sfzformat.com/opcodes/phaser_feedback)
    ///
    phaser_feedback(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [phaser_freq](https://sfzformat.com/opcodes/phaser_freq)
    ///
    phaser_freq(f32),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [phaser_phase_onccN](https://sfzformat.com/opcodes/phaser_phase_onccN)
    ///
    phaser_phase_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [phaser_stages](https://sfzformat.com/opcodes/phaser_stages)
    ///
    phaser_stages(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [phaser_waveform](https://sfzformat.com/opcodes/phaser_waveform)
    ///
    phaser_waveform(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [phaser_wet](https://sfzformat.com/opcodes/phaser_wet)
    ///
    phaser_wet(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [reverb_damp](https://sfzformat.com/opcodes/reverb_damp)
    ///
    reverb_damp(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [reverb_dry](https://sfzformat.com/opcodes/reverb_dry)
    ///
    reverb_dry(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [reverb_input](https://sfzformat.com/opcodes/reverb_input)
    ///
    reverb_input(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [reverb_predelay](https://sfzformat.com/opcodes/reverb_predelay)
    ///
    reverb_predelay(f32),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [reverb_size](https://sfzformat.com/opcodes/reverb_size)
    ///
    reverb_size(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [reverb_tone](https://sfzformat.com/opcodes/reverb_tone)
    ///
    reverb_tone(UnknownType),

    ///
    /// - range: chamber, large_hall, large_room, mid_hall, mid_room, small_hall, small_room
    /// - default: None
    /// - version: v2
    /// - info: [reverb_type](https://sfzformat.com/opcodes/reverb_type)
    ///
    reverb_type(String),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [reverb_wet](https://sfzformat.com/opcodes/reverb_wet)
    ///
    reverb_wet(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [static_cyclic_level](https://sfzformat.com/opcodes/static_cyclic_level)
    ///
    static_cyclic_level(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [static_cyclic_time](https://sfzformat.com/opcodes/static_cyclic_time)
    ///
    static_cyclic_time(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [static_filter](https://sfzformat.com/opcodes/static_filter)
    ///
    static_filter(String),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [static_level](https://sfzformat.com/opcodes/static_level)
    ///
    static_level(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [static_random_level](https://sfzformat.com/opcodes/static_random_level)
    ///
    static_random_level(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [static_random_maxtime](https://sfzformat.com/opcodes/static_random_maxtime)
    ///
    static_random_maxtime(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [static_random_mintime](https://sfzformat.com/opcodes/static_random_mintime)
    ///
    static_random_mintime(f32),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [static_stereo](https://sfzformat.com/opcodes/static_stereo)
    ///
    static_stereo(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [static_tone](https://sfzformat.com/opcodes/static_tone)
    ///
    static_tone(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [strings_number](https://sfzformat.com/opcodes/strings_number)
    ///
    strings_number(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [strings_wet_onccN](https://sfzformat.com/opcodes/strings_wet_onccN)
    ///
    strings_wet_onccN(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [tdfir_dry](https://sfzformat.com/opcodes/tdfir_dry)
    ///
    tdfir_dry(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [tdfir_gain](https://sfzformat.com/opcodes/tdfir_gain)
    ///
    tdfir_gain(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [tdfir_impulse](https://sfzformat.com/opcodes/tdfir_impulse)
    ///
    tdfir_impulse(UnknownType),

    ///
    /// - range: 0 to 100
    /// - default: None
    /// - version: v2
    /// - info: [tdfir_wet](https://sfzformat.com/opcodes/tdfir_wet)
    ///
    tdfir_wet(UnknownType),

    ///
    /// - range: 0 to 1
    /// - default: None
    /// - version: v2
    /// - info: [load_mode](https://sfzformat.com/opcodes/load_mode)
    ///
    load_mode(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [load_start](https://sfzformat.com/opcodes/load_start)
    ///
    load_start(UndefinedInteger),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [load_end](https://sfzformat.com/opcodes/load_end)
    ///
    load_end(UndefinedInteger),

    ///
    /// - range: 1 to 10
    /// - default: None
    /// - version: v2
    /// - info: [sample_quality](https://sfzformat.com/opcodes/sample_quality)
    ///
    sample_quality(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [image](https://sfzformat.com/opcodes/image)
    ///
    image(String),

    ///
    /// - range: on, off
    /// - default: None
    /// - version: v2
    /// - info: [oscillator](https://sfzformat.com/opcodes/oscillator)
    ///
    oscillator(String),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_detune](https://sfzformat.com/opcodes/oscillator_detune)
    ///
    oscillator_detune(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_detune_onccN](https://sfzformat.com/opcodes/oscillator_detune_onccN)
    ///
    oscillator_detune_onccN(UnknownType),

    ///
    /// - range: 0 to 2
    /// - default: 0
    /// - version: v2
    /// - info: [oscillator_mode](https://sfzformat.com/opcodes/oscillator_mode)
    ///
    oscillator_mode(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_mod_depth](https://sfzformat.com/opcodes/oscillator_mod_depth)
    ///
    oscillator_mod_depth(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_mod_depth_onccN](https://sfzformat.com/opcodes/oscillator_mod_depth_onccN)
    ///
    oscillator_mod_depth_onccN(UnknownType),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_mod_smoothccN](https://sfzformat.com/opcodes/oscillator_mod_smoothccN)
    ///
    oscillator_mod_smoothccN(UnknownType),

    ///
    /// - range: 1 to 9
    /// - default: 1
    /// - version: v2
    /// - info: [oscillator_multi](https://sfzformat.com/opcodes/oscillator_multi)
    ///
    oscillator_multi(u8),

    ///
    /// - range: -1 to 360 °
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_phase](https://sfzformat.com/opcodes/oscillator_phase)
    ///
    oscillator_phase(f32),

    ///
    /// - range: 0 to 3
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_quality](https://sfzformat.com/opcodes/oscillator_quality)
    ///
    oscillator_quality(u8),

    ///
    /// - range: None
    /// - default: None
    /// - version: v2
    /// - info: [oscillator_table_size](https://sfzformat.com/opcodes/oscillator_table_size)
    ///
    oscillator_table_size(UnknownType),
    // cakewalk extension opcodes -----------------------------------------------
    // https://sfzformat.com/extensions/cakewalk/
}

impl Opcode {
    /// Returns the default value of the current opcode
    ///
    pub fn default_value(&self) -> Option<OpcodeType> {
        OPCODE_DEFAULT.get::<str>(&self.str_name()).cloned()
    }

    // Returns the name of the current opcode as a string
    //
    // used by .default_value()
    pub fn str_name(&self) -> String {
        format!("{:?}", &self).split('(').collect::<Vec<&str>>()[0].to_string()
    }

    // const stati version of str_name
    // function pointers in const fn are unstable: https://github.com/rust-lang/rust/issues/63997
    //
    // This would be nice for creating a macro to convert this:
    // "hikey" => utils::check_u8_between(value, 0, 127).map(Opcode::hikey),
    //
    // into this:
    // check_between!(Opcode::hikey, u8, 0, 127),
    //
    // pub const fn static_str_name(opcode: &Opcode) -> String {
    //     format!("{:?}", opcode).split('(').collect::<Vec<&str>>()[0].to_string()
    // }

    // Returns the default value of any opcode, given an opcode's name
    //fn default_value_static(name: &str) -> Option<OpcodeType> {
    //     OPCODE_DEFAULT.get(name).cloned()
    // }
}
