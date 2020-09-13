use std::path::PathBuf;

/// println! the name of a type
pub fn print_type<T: ?Sized>(_: &T) {
    println!("{}", core::any::type_name::<T>())
}

/// This function makes it possible to interpret a Windows path correctly
/// from Linux, and viceversa.
///
/// NOTE: This only works well with relative paths.
///
// https://udoprog.github.io/rust/2017-11-05/portability-concerns-with-path.html
// https://users.rust-lang.org/t/parsing-cross-platform-paths/18726
pub(crate) fn fix_path_separators(path: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(str::replace(path, r"/", r"\"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        PathBuf::from(str::replace(path, r"\", r"/"))
    }
}
