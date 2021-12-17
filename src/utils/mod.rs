//! General utilites
//!
#![allow(dead_code, unused_imports)]

mod misc;
mod parse;

pub(crate) use misc::{fix_path_separators, print_type};
pub(crate) use parse::*;
