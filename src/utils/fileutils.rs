// Libraries
use std::path::Path;
use std::fs;

/// Copy a directory from `from` to `to`.
///
/// The following cases will be considered:  
///  * `to` exists, then `from` will be created as `to`'s child directory
///  * `to` does not exist, then to will become the new dir name i.e it will
///     be created and the contents of `from` will be copied into `to`
///  * `to` and `from` point to the same directory, then no action is taken
pub fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) {
    // Save from and to as (refs to) paths
    let from_path = from.as_ref();
    let to_path = to.as_ref();

    // Check if to exists
    if ! to_path.exists() {
        // Create to and recursively call function
        fs::create_dir_all(to_path);
        copy_dir(from_path, to_path);
    }

}
