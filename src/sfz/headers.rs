use logos::Lexer;

use crate::sfz::SfzToken;


/// SFZ files are subdivided into sections by headers.
///
/// The global/group/region or global/master/group/region hierarchy contains the opcodes
/// which define which samples are played, when they are to be played, and how.
///
/// [sfzformat.com/headers/](https://sfzformat.com/headers/)

#[derive(Debug, PartialEq)]
pub enum Header {

    // sfz v1 headers

    /// The basic component of an instrument. An instrument is defined by one or more regions.
    ///
    /// - version: v1
    /// - info: [region](https://sfzformat.com/headers/region)
    Region,

    /// Multiple regions can be arranged in a group. Groups allow entering common parameters for multiple regions.
    ///
    /// - version: v1
    /// - info: [group](https://sfzformat.com/headers/group)
    Group,

    // sfz v2 headers:

    /// The control header would be found at the beginning of the file
    /// and includes special opcodes for setting up MIDI CC controls.
    ///
    /// - version: v2
    /// - info: [control](https://sfzformat.com/headers/control)
    ///
    Control,

    /// The global header (one per file), contains opcodes which apply to all regions in the file.
    ///
    /// - version: v2
    /// - info: [global](https://sfzformat.com/headers/global)
    Global,

    /// The curve headers, when used, are normally found at the end of the file,
    /// and define the curves used for shaping envelopes, parameter response etc.
    ///
    /// - version: v2
    /// - info: [curve](https://sfzformat.com/headers/curve)
    Curve,

    /// SFZ v2 header for effects controls.
    ///
    /// In SFZ v1 only effect1 and effect2 opcodes was available and only at ‹region› level.
    ///
    /// From SFZ v2 this header was added together with the addition of
    /// effect3 and effect4 opcodes also to modulate the related bus.
    /// Other opcodes listed in the book are bus, type and dsp_order.
    ///
    /// - version: v2
    /// - info: [effect](https://sfzformat.com/headers/effect)
    Effect,

    // aria headers:

    /// The master header is an extra level added inbetween group and global for the ARIA player.
    ///
    /// - version: aria extension
    /// - info: [master](https://sfzformat.com/headers/master)
    Master,

    /// ARIA extension, was added for MIDI pre-processor effects.
    /// From ARIA v1.0.8.0+ an ‹effect› section with a bus=midi can be used instead.
    ///
    /// - version: aria extension
    /// - info: [midi](https://sfzformat.com/headers/midi)
    Midi,

    // cakewalk headers:

    /// Allows to embed sample data directly in SFZ files (Rapture).
    ///
    /// - version: cakewalk extension
    /// - info: [sample](https://sfzformat.com/headers/sample)
    Sample,
}


impl Header {
    /// Parses the opcode, making it lowercase and removing the brackets
    ///
    pub(crate) fn parse_header(lex: &mut Lexer<SfzToken>) -> Option<Header> {
        let slice = lex.slice().to_ascii_lowercase();
        let name = &slice[1..slice.len()-1];
        match name {
             "region" => Some(Header::Region),
             "group" => Some(Header::Group),
             "control" => Some(Header::Control),
             "global" => Some(Header::Global),
             "curve" => Some(Header::Curve),
             "effect" => Some(Header::Effect),
             "master" => Some(Header::Master),
             "midi" => Some(Header::Midi),
             "sample" => Some(Header::Sample),
             _ => None,
        }
    }
}
