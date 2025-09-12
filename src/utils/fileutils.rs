// Libraries
use std::path::Path;
use std::fs;
use std::io;

/// This struct represents an Error for this module
pub struct FileUtilErr {}




/// Copy a directory from `from` to `to`.
///
/// The following cases will be considered:  
///  * `to` exists, then `from` will be created as `to`'s child directory
///  * `to` does not exist, then to will become the new dir name i.e it will
///     be created and the contents of `from` will be copied into `to`
///  * `to` and `from` point to the same directory, then no action is taken
pub fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    // Save from and to as (refs to) paths
    let from_path = from.as_ref();
    let to_path = to.as_ref();

    // Check if to exists
    if ! to_path.exists() {
        // Create to and recursively call function
        fs::create_dir_all(to_path);
    }

    // Open from_dir
    let from_dir = match (fs::read_dir(from_path)) {
        Ok(dir) => dir,
        Err(e) => { return Err(e); },
    };

    // Iterate through `from_dir`
    for from_entries in from_dir {
        // Open directory entry as entry
        let opened_entry = match (from_entries) {
            Ok(entry) => entry,
            Err(e) => { return Err(e); },
        };

        // Get the file_type of the opened_entry
        let opened_file_type = match (opened_entry.file_type()) {
            Ok (file_type) => file_type,
            Err(e) => { return Err(e); },
        };

        // check if opened_entry is a directory
        if is_dir(opened_file_type) {
        }
    }




    return Ok(());

}
