use std::path::PathBuf;

use logos::{Lexer, Logos};
use regex::Regex;

use crate::sfz::types::{fil_type, loop_mode, trigger, MAX_SAMPLE_RATE};
use crate::sfz::{utils, Header, Opcode};

impl Opcode {
    /// Receives an opcode name with numeric parameters, and returns the canonical
    /// opcode name (with its numeric parameters changed back to N,X,Y letters),
    /// alongside a tuple with up to 3 optional parameters
    ///
    /// For example, these opcode 'names':
    ///     "cutoff"
    ///     "hicc88"
    ///     "var22_oncc44"
    ///     "lfo111_eq44gain_oncc55"
    ///
    /// …will get processed into:
    ///     ("cutoff", vec![])
    ///     ("hiccN", vec![88])
    ///     ("varNN_onccX", vec![22, 44])
    ///     ("lfoN_eqXgain_onccY", vec![111, 44, 55])
    ///
    pub(crate) fn parse_name(name: &str) -> (String, Vec<u8>) {
        let mut new_name = String::new(); // the new constructed name
        let mut params = Vec::new(); // to store the found parameters

        // First parameter is always 'N', second is always 'X', and third is always 'Y'
        let mut par_num: usize = 0; // the current parameter number (max 3)
        let par_char = ['N', 'X', 'Y']; // ordered list of parameters as chars in the opcode name

        let mut previous_span_end = 0;
        let mut remainder = String::new(); // the remainder opcode name after the current parameter

        // Tries to find numeric parameters embedded in the name
        let lex_numbers = OpcodeParameter::lexer(&name);
        for (n, span) in lex_numbers.spanned() {
            match &n {
                // If a parameter is found
                OpcodeParameter::Parameter(p) => {
                    // constructs the new name of the opcode,
                    // with the numeric parameters being substituted by
                    // the N, X, Y chars, in that order.

                    let split_idx = span.start; // the index of current parameter in original name

                    let (first, last) = name.split_at(split_idx);
                    remainder = last[span.end - span.start..].to_string();

                    // first handle the special case of 4 opcodes with an "NN" parameter:
                    // (varNN_mod, varNN_onccX, varNN_curveccX, varNN_target)
                    if par_num == 0 && Regex::new(r"^var").unwrap().is_match(&name) {
                        new_name += &format!("{}NN", &first[previous_span_end..]);

                    // then handle the rest of the cases
                    } else {
                        new_name +=
                            &format!("{}{}", &first[previous_span_end..], par_char[par_num]);
                    }

                    //println!("{} ({})  remainder: {}", &new_name, previous_span_end, remainder); // DEBUG

                    // TODO: check for numeric boundaries
                    //
                    // TODO NOTE
                    // 1. all opcodes ending in ccN has N = 0..=127 (or ccX when N is already used)
                    //  (except some sfz2 and aria extensions, see:
                    //  https://sfzformat.com/extensions/midi_ccs
                    //
                    // 2. each of these has N = 1..=3 (and X = 0..=127)
                    //   eqN_bw
                    //   eqN_bwccX
                    //   eqN_freq
                    //   eqN_freqccX
                    //   eqN_vel2freq
                    //   eqN_gain
                    //   eqN_gainccX
                    //   eqN_vel2gain

                    // Stores the numeric parameter
                    params.push(*p as u8);

                    par_num += 1;
                    previous_span_end = span.end;
                }
                _ => (),
            }
        }

        // In case there where no opcode parameters, the opcode name is unchanged
        if par_num == 0 {
            new_name = name.to_string();

        // In case there's a remainder after the last parameter, append it
        } else if remainder.len() > 0 {
            new_name += &remainder;
        }

        //println!("NAME: {}\nPROCESSED: {:?}\n", name, (&new_name, &params)); // DEBUG
        return (new_name, params);
    }

    /// Replaces the opcode parameter letters for numbers, as they are really used,
    /// preparing them for testing the parsing.
    #[allow(dead_code)]
    pub(crate) fn numerize_pars(opcode: &str) -> String {
        let mut o_new;

        o_new = Regex::new(r"Y").unwrap().replace(opcode, "33").to_string(); //  Y > 33
        o_new = Regex::new(r"X").unwrap().replace(&o_new, "22").to_string(); //  X > 22
        o_new = Regex::new(r"NN").unwrap().replace(&o_new, "11").to_string(); // NN > 11
        o_new = Regex::new(r"N").unwrap().replace(&o_new, "11").to_string(); //  N > 11

        o_new
    }

    // IDEA:1 when some opcode is incorrect or the value is incorrect, or out of range
    // a warning should be printed with the information.

    // IDEA:2 it would be nice to have a macro to write each (full) match arm like this:
    //     check_between!(Opcode::hikey, u8, 0, 127),
    // converting it into the version below. But for that I need a const version of
    // opcode.str_name(), which is still unstable. (+ info below fn opcode.str_name()).

    // IDEA:3 Return an Opcode that can be from v1, v2, aria or cakewalk
    // which are different enumss
    // (so a nested specific opcode enum inside a version/extension enum)
    // but... for modified previous enums it could mean confusion...?
    // unless if a parsing fails, it can try to do it with the rules of another version
    // like the 0-127 range in v1 and -1-127 in v2 ....
    // (thos opcodes can even be of a different different type in each version)

    pub(crate) fn parse_opcode(lex: &mut Lexer<SfzToken>) -> Option<Opcode> {
        // TODO: return also the opcode name parameters
        // pub(crate) fn parse_opcode(lex: &mut Lexer<SfzToken>)
        //     -> (Option<Opcode>, Option<vec![u8]>) {

        let slice = lex.slice();

        let kv: Vec<&str> = slice.splitn(2, "=").collect();
        let (opcode, value) = (kv[0], kv[1]);

        let (opcode, values) = Opcode::parse_name(opcode);

        match (opcode.as_str(), values) {
            // TODO: test returning opcode name parameters
            ("eqN_bwccX", _) => utils::check_f32_between(value, -4., 4.).map(Opcode::eqN_bwccX),

            // v1
            ("amp_veltrack", _) => {
                utils::check_f32_between(value, -100., 100.).map(Opcode::amp_veltrack)
            }
            ("amp_random", _) => utils::check_f32_between(value, 0., 24.).map(Opcode::amp_random),
            ("ampeg_attack", _) => {
                utils::check_f32_between(value, 0., 100.).map(Opcode::ampeg_attack)
            }
            // NOTE: VPO: TODO: ampeg_attackcc1 (needs special parsing)
            ("ampeg_attackccN", _) => {
                utils::check_f32_between(value, 0., 100.).map(Opcode::ampeg_attackccN)
            }
            ("ampeg_decay", _) => {
                utils::check_f32_between(value, 0., 100.).map(Opcode::ampeg_decay)
            }
            ("ampeg_hold", _) => utils::check_f32_between(value, 0., 100.).map(Opcode::ampeg_hold),
            ("ampeg_release", _) => {
                utils::check_f32_between(value, 0., 100.).map(Opcode::ampeg_release)
            }
            ("ampeg_sustain", _) => {
                utils::check_f32_between(value, 0., 100.).map(Opcode::ampeg_sustain)
            }
            // NOTE: upper range is SampleRate/2 (it should be checked when sample rate is known)
            ("cutoff", _) => {
                utils::check_f32_between(value, 0., MAX_SAMPLE_RATE).map(Opcode::cutoff)
            }
            ("fil_type", _) => fil_type::from_str(value).map(Opcode::fil_type),
            ("fil_veltrack", _) => {
                utils::check_i16_between(value, -9600, 9600).map(Opcode::fil_veltrack)
            }
            // NOTE: hikey v2 accepts i8, from -1:
            ("hikey", _) => utils::check_u8_between(value, 0, 127).map(Opcode::hikey),
            ("hivel", _) => utils::check_u8_between(value, 0, 127).map(Opcode::hivel),
            ("hirand", _) => utils::check_f32_between(value, 0., 1.).map(Opcode::hirand),
            // NOTE: lokey v2 accepts i8, from -1:
            ("lokey", _) => utils::check_u8_between(value, 0, 127).map(Opcode::lokey),
            ("lovel", _) => utils::check_u8_between(value, 0, 127).map(Opcode::lovel),
            ("loop_mode", _) => loop_mode::from_str(value).map(Opcode::loop_mode),
            ("lorand", _) => utils::check_f32_between(value, 0., 1.).map(Opcode::lorand),
            ("off_by", _) => utils::check_u32_between(value, 0, u32::MAX).map(Opcode::off_by),
            ("offset", _) => utils::check_u32_between(value, 0, u32::MAX).map(Opcode::offset),
            ("on_loccN", _) => utils::check_i8_between(value, 0, 127).map(Opcode::on_loccN),
            ("on_hiccN", _) => utils::check_i8_between(value, 0, 127).map(Opcode::on_hiccN),
            ("pan", _) => utils::check_f32_between(value, 0., 100.).map(Opcode::pan),
            ("pitch_keycenter", _) => {
                utils::check_u8_between(value, 0, 127).map(Opcode::pitch_keycenter)
            }
            ("pitch_keytrack", _) => {
                utils::check_i16_between(value, -1200, 1200).map(Opcode::pitch_keytrack)
            }
            ("pitch_random", _) => {
                utils::check_u16_between(value, 0, 9600).map(Opcode::pitch_random)
            }
            ("rt_decay", _) => utils::check_f32_between(value, 0., 200.).map(Opcode::rt_decay),
            ("sample", _) => Some(Opcode::sample(PathBuf::from(utils::fix_path_separators(
                value,
            )))),
            ("seq_lenght", _) => utils::check_u8_between(value, 1, 100).map(Opcode::seq_length),
            ("seq_position", _) => utils::check_u8_between(value, 1, 100).map(Opcode::seq_position),
            ("trigger", _) => trigger::from_str(value).map(Opcode::trigger),
            ("sw_hikey", _) => utils::check_u8_between(value, 0, 127).map(Opcode::sw_hikey),
            ("sw_last", _) => utils::check_u8_between(value, 0, 127).map(Opcode::sw_last),
            ("sw_lokey", _) => utils::check_u8_between(value, 0, 127).map(Opcode::sw_lokey),
            ("tune", _) => utils::check_i8_between(value, -100, 100).map(Opcode::tune),
            ("volume", _) => utils::check_f32_between(value, -144., 6.).map(Opcode::volume),
            ("xfin_hivel", _) => utils::check_u8_between(value, 0, 127).map(Opcode::xfin_hivel),
            ("xfin_lovel", _) => utils::check_u8_between(value, 0, 127).map(Opcode::xfin_lovel),
            ("xfout_hivel", _) => utils::check_u8_between(value, 0, 127).map(Opcode::xfout_hivel),
            ("xfout_lovel", _) => utils::check_u8_between(value, 0, 127).map(Opcode::xfout_lovel),

            // v2
            ("sw_default", _) => utils::check_u8_between(value, 0, 127).map(Opcode::sw_default),
            ("default_path", _) => Some(Opcode::default_path(utils::fix_path_separators(value))),

            // aria
            ("ampeg_dynamic", _) => {
                utils::check_u8_between(value, 0, 127).map(Opcode::ampeg_dynamic)
            }
            ("group_label", _) => Some(Opcode::group_label(value.to_string())),
            ("sw_label", _) => Some(Opcode::sw_label(value.to_string())),

            _ => None,
        }
    }
}

/// Token for parsing SFZ format elements like headers and tokens
///
#[derive(Logos, Debug, PartialEq)]
pub(crate) enum SfzToken {
    /// Parses a Header
    ///
    #[regex("<[a-zA-Z]+>", Header::parse_header)]
    Header(Header),

    /// Parses an Opcode, including the value
    ///
    /// Opcodes and assigned opcode values are separated by the equal to sign (=),
    /// without spaces between the opcode and the sign.
    ///
    #[regex("[a-zA-Z0-9_]+=.+", Opcode::parse_opcode)]
    Opcode(Opcode),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    WhiteSpace,

    #[regex(r"[//.*$]", logos::skip)]
    Comments,

    #[error]
    Error,
}

/// Returns the correct parameters from an opcode name
///
///
/// Some opcode names contains numbers that must not be interpreted as parameters.
/// It makes sure to filter out false positives,
/// as parameters,
#[derive(Logos, Debug, PartialEq)]
pub(crate) enum OpcodeParameter {
    /// Skip numbers that must not be recognized as parameters,
    /// since they are part of the opcode's name.
    ///
    #[regex(r"(fil2_|_vel2|effect[0-4]|cutoff2|resonance2|md5)", logos::skip)]
    FalsePositive,

    /// Returns the real numeric parameter
    ///
    // TODO: IMPROVE (should be {1,3}, range 0-¿127? CHECK)
    // #[regex("[0-9]{1,3}")] // ← constricted digit repetition
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Parameter(u8),

    #[error]
    Error,
}

#[cfg(test)]
mod tests_opcodes {
    use super::*;
    use logos::Logos;

    #[test]
    fn test_opcode_ampeg_attack() {
        let mut lex = SfzToken::lexer("ampeg_attack=0.001");
        assert_eq!(
            lex.next(),
            Some(SfzToken::Opcode(Opcode::ampeg_attack(0.001)))
        );
    }

    #[test]
    fn test_opcode_pan() {
        let mut lex = SfzToken::lexer("pan=100");
        assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::pan(100.0))));

        let mut lex = SfzToken::lexer("pan=0");
        assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::pan(0.0))));

        let mut lex = SfzToken::lexer("pan=67.353");
        assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::pan(67.353))));

        let mut lex = SfzToken::lexer("pan=101.0");
        assert_eq!(lex.next(), Some(SfzToken::Error));
        //
        // let mut lex = SfzToken::lexer("pan=-1.0");
        // assert_eq!(lex.next(), Some(SfzToken::Error));
    }

    #[test]
    fn test_opcode_sample() {
        let mut lex = SfzToken::lexer("sample=MOHorn mute_A#1_v1_1.wav");
        assert_eq!(
            lex.next(),
            Some(SfzToken::Opcode(Opcode::sample(PathBuf::from(
                "MOHorn mute_A#1_v1_1.wav"
            ))))
        );

        // The equal sign is also supported in the filename
        let mut lex = SfzToken::lexer("sample=equal_sign_=_doesn't_fail.wav");
        assert_eq!(
            lex.next(),
            Some(SfzToken::Opcode(Opcode::sample(PathBuf::from(
                "equal_sign_=_doesn't_fail.wav"
            ))))
        );
    }
}

#[cfg(test)]
mod tests_token {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_sfz_simple() {
        let mut lex = SfzToken::lexer(
            "<region>
sample=MOHorn_mute_A#1_v1_1.wav
lokey=46
hikey=48
pitch_keycenter=46
lovel=0
hivel=62
volume=17",
        );

        assert_eq!(lex.next(), Some(SfzToken::Header(Header::Region)));
        assert_eq!(lex.span(), 0..8);

        assert_eq!(
            lex.next(),
            Some(SfzToken::Opcode(Opcode::sample(PathBuf::from(
                "MOHorn_mute_A#1_v1_1.wav"
            ))))
        );
        assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::lokey(46))));
        // assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::hikey(48))));
        // assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::pitch_keycenter(46))));
        // assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::lovel(0))));
        // assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::hivel(62))));
        // assert_eq!(lex.next(), Some(SfzToken::Opcode(Opcode::volume(17.0))));
    }
}

#[cfg(test)]
mod tests_parameters {

    use super::{Opcode, Regex};

    #[test]
    fn test_parse_all_opcodes() {
        // DATA
        //
        // [999] indicates the current number opcodes at 2020-09-11
        // N and NN gets treated equally as being 1 parameter.

        // file names are suffixed with the number of opcodes contained
        static OPCODES_SFZ_V1: &str = include_str!("data/opcodes-v1_184.txt");
        static OPCODES_SFZ_V2: &str = include_str!("data/opcodes-v2_166.txt");
        static OPCODES_SFZ_ARIA_EXT: &str = include_str!("data/opcodes-aria-extension_78.txt");
        static OPCODES_SFZ_CAKEWALK_V2: &str = include_str!("data/opcodes-cakewalk-v2_162.txt");
        let opcodes_total = format!(
            "{}{}{}{}", // the list of all the opcodes
            OPCODES_SFZ_V1, OPCODES_SFZ_V2, OPCODES_SFZ_ARIA_EXT, OPCODES_SFZ_CAKEWALK_V2,
        );
        let mut opcodes_params = String::new(); // …with any parameters
        let mut opcodes_params_3 = String::new(); // …with 3 parameters
        let mut opcodes_params_2 = String::new(); // …with 2 parameters
        let mut opcodes_params_1 = String::new(); // …with 1 parameter
        let mut opcodes_no_params = String::new(); // …with no parameters

        let mut opcodes_total_count: u16 = 0; // [590] the total number of opcodes
        let mut opcodes_params_count: u16 = 0; // [244] …with any parameters
        let mut opcodes_params_3_count: u16 = 0; //  [15] …with 3 parameters
        let mut opcodes_params_2_count: u16 = 0; //  [93] …with 2 parameters
        let mut opcodes_params_1_count: u16 = 0; // [136] …with 1 parameter
        let mut opcodes_no_params_count: u16 = 0; // [346] …with no parameters

        // Every opcode with parameters indicates them as uppercase N, X or Y,
        // having always an N firstly, in either case.
        let re_par1 = Regex::new(r"N").unwrap();
        //let _re_par1b = Regex::new(r"NN").unwrap();
        let re_par2 = Regex::new(r"X").unwrap();
        let re_par3 = Regex::new(r"Y").unwrap();

        // CLASSIFICATION

        for o in opcodes_total.lines() {
            opcodes_total_count += 1;

            // opcodes with any number of parameters
            if re_par1.is_match(o) {
                opcodes_params_count += 1;
                opcodes_params = format!("{}\n{}", opcodes_params, o);

                if re_par2.is_match(o) {
                    // opcodes with 3 parameters
                    if re_par3.is_match(o) {
                        opcodes_params_3_count += 1;
                        opcodes_params_3 = format!("{}\n{}", opcodes_params_3, o);

                    // opcodes with only 2 parameters
                    } else {
                        opcodes_params_2_count += 1;
                        opcodes_params_2 = format!("{}\n{}", opcodes_params_2, o);
                    }

                // opcodes with only 1 parameter
                } else {
                    opcodes_params_1_count += 1;
                    opcodes_params_1 = format!("{}\n{}", opcodes_params_1, o);
                }

            // opcodes with no parameters
            } else {
                opcodes_no_params_count += 1;
                opcodes_no_params = format!("{}\n{}", opcodes_no_params, o);
            }
        }

        // Delete leading & trailing whitespace including newlines
        opcodes_no_params = opcodes_no_params.trim().to_string();
        opcodes_params_1 = opcodes_params_1.trim().to_string();
        opcodes_params_2 = opcodes_params_2.trim().to_string();
        opcodes_params_3 = opcodes_params_3.trim().to_string();

        // DEBUG
        // println!("total: {}\nwith params: {} (1={}, 2={}, 3={}), no params: {}",
        //     opcodes_total_count,
        //     opcodes_params_count,
        //     opcodes_params_1_count,
        //     opcodes_params_2_count,
        //     opcodes_params_3_count,
        //     opcodes_no_params_count,
        // );

        // Check the counting checks up
        assert_eq!(
            opcodes_params_count,
            opcodes_params_1_count + opcodes_params_2_count + opcodes_params_3_count
        );
        assert_eq!(
            opcodes_total_count,
            opcodes_params_count + opcodes_no_params_count
        );

        // TEST opcode parsing

        // Test parsing the opcodes with 3 parameters
        for o in opcodes_params_3.lines() {
            let o_new = Opcode::numerize_pars(&o);
            let (o_parsed, params) = Opcode::parse_name(&o_new);
            assert_eq!(&o, &o_parsed);
            assert_eq!(params, vec![11, 22, 33]);
        }

        // Test parsing the opcodes with 2 parameters
        for o in opcodes_params_2.lines() {
            let o_new = Opcode::numerize_pars(&o);
            let (o_parsed, params) = Opcode::parse_name(&o_new);
            assert_eq!(&o, &o_parsed);
            assert_eq!(params, vec![11, 22]);
        }

        // Test parsing the opcodes with 1 parameters
        for o in opcodes_params_1.lines() {
            let o_new = Opcode::numerize_pars(&o);
            let (o_parsed, params) = Opcode::parse_name(&o_new);
            assert_eq!(&o, &o_parsed);
            assert_eq!(params, vec![11]);
        }

        // Test parsing the opcodes with no parameters
        for o in opcodes_no_params.lines() {
            let o_new = Opcode::numerize_pars(&o);
            let (o_parsed, params) = Opcode::parse_name(&o_new);
            assert_eq!(&o, &o_parsed);
            assert_eq!(params, vec![]);
        }
    }
}
