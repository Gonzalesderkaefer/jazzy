// Libraries
use std::path::Path;
use std::fs; use std::io; /// This struct represents an Error for this module
pub struct FileUtilErr {}




/// Copy a directory from `from` to `to`.
///
/// The following cases will be considered:  
///  * `to` exists, then `from` will be created as `to`'s child directory
///  * `to` does not exist, then to will become the new dir name i.e it will
///     be created and the contents of `from` will be copied into `to`
///  * `to` does not exist, then no action is taken
pub fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    // Save from and to as (refs to) paths
    let from_path = from.as_ref();
    let to_path = to.as_ref();

    // Check if from exists
    if ! from_path.exists() {
        return Ok(());
    }

    // Check if to exists
    if ! to_path.exists() {
        // Create to
        let create_dir_result = fs::create_dir_all(to_path);
        match (create_dir_result) {
            Ok(none) => {},
            Err(e) => { return Err(e); },
        }
        // call the function recursively
        let copy_result = copy_dir(from_path, to_path);
        return copy_result;
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

        // Get the basename of the opened dir
        let opened_basename = opened_entry.file_name();

        // Build new source directory
        let mut source_dir_path = from_path.to_path_buf();
        source_dir_path.push(&opened_basename);

        // Build new destination directory
        let mut dest_dir_path = to_path.to_path_buf();
        dest_dir_path.push(&opened_basename);

        // check if opened_entry is a directory
        if opened_file_type.is_dir() {
            // Call the function recursively
            match copy_dir(source_dir_path.as_path(), dest_dir_path.as_path()) {
                Ok(()) => {},
                Err(e) => { return Err(e); },
            }
        } else {
            match fs::copy(source_dir_path.as_path(), dest_dir_path.as_path()) {
                Ok(num) => {},
                Err(e) => { return Err(e); },
            }
        }

    }




    return Ok(());

}
