//! General utilites

use std::path::PathBuf;

mod misc;
mod parse;

pub(crate) use misc::{print_type, fix_path_separators};
pub(crate) use parse::*;
