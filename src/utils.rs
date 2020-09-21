//! General utilites
//!
#![allow(dead_code, unused_imports)]

mod misc;
mod parse;

pub(crate) use misc::{print_type, fix_path_separators};
pub(crate) use parse::*;
