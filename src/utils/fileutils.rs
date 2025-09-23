use std::{path::Path};
use std::error::Error;
use std::{fs, io, fmt};

use crate::machine::transfer::Transfer;



/// This module's Error type
#[derive(Debug)]
pub enum FileUtilErr {
    // copy_dir
    IO (io::Error),
    RootSrc,
    NoSource (String),
    InvalidSrc,

    // create_file_and_write
    WriteToDir,
}
impl fmt::Display for FileUtilErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Self::IO (err) => {
                return write!(f, "Internal IO Error: {}", err);
            },
            Self::RootSrc => {
                return write!(f, "Root cannot be the source when copying directories");
            },
            Self::NoSource (source) => {
                return write!(f, "No source: \"{source}\"");
            },
            Self::InvalidSrc => {
                return write!(f, "Source is not valid unicode");
            },
            Self::WriteToDir => {
                return write!(f, "Can't write to a directory");
            },
        }
    }
}
impl Error for FileUtilErr {}


/// Copy a directory from `from` to `to`.
///
/// The following cases will be considered:  
///  * `to` exists, then `from` will be created as `to`'s child directory
///  * `to` does not exist, then to will become the new dir name i.e it will
///     be created and the contents of `from` will be copied into `to`
///  * `to` does not exist, then no action is taken
pub fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(source: P, dest: Q) -> Result<(), FileUtilErr> {
    // Save from and to as (refs to) paths
    let source_path = source.as_ref();
    let mut dest_path = dest.as_ref();

    // Needed when editing dest_path
    let dest_buffer: std::path::PathBuf;

    // Check if from exists
    if ! source_path.exists() {
        // Get source path as &str
        let source_path_str = match source_path.to_str() {
            Some(str) => str,
            None => { return Err(FileUtilErr::InvalidSrc); }
        };
        return Err(FileUtilErr::NoSource(String::from(source_path_str)));
    }

    // Check if to exists
    if dest_path.exists() {
        // Get basename of source
        let source_basename = match source_path.file_name() {
            Some(basename) => basename,
            None => {
                return Err(FileUtilErr::RootSrc);
            }
        };
        // append basename of source to destination
        dest_buffer = dest_path.join(source_basename);

        // change value in dest_path
        dest_path = dest_buffer.as_path();
    }

    // create the destination path
    let create_dir_result = fs::create_dir_all(dest_path);
    // Check its result, and return if it failed
    match create_dir_result {
        Ok(_) => {},
        Err(e) => {
            return Err(FileUtilErr::IO(e)); 
        },
    }


    // Open source_dir
    let from_dir = match fs::read_dir(source_path) {
        Ok(dir) => dir,
        Err(e) => { 
            return Err(FileUtilErr::IO(e)); 
        },
    };

    // Iterate through source_dir
    for from_entries in from_dir {
        // Open directory entry as entry
        let opened_entry = match from_entries {
            Ok(entry) => entry,
            Err(e) => { 
                return Err(FileUtilErr::IO(e)); 
            },
        };

        // Get the file_type of the opened_entry
        let opened_file_type = match opened_entry.file_type() {
            Ok (file_type) => file_type,
            Err(e) => {
                return Err(FileUtilErr::IO(e));
            },
        };

        // Get the basename of the opened entry
        let opened_basename = opened_entry.file_name();

        // Build new source path
        let mut source_dir_path = source_path.to_path_buf();
        source_dir_path.push(&opened_basename);

        // Build new destination path
        let mut dest_dir_path = dest_path.to_path_buf();
        dest_dir_path.push(&opened_basename);

        // check if opened_entry is a directory
        if opened_file_type.is_dir() {
            // Call the function recursively
            match copy_dir(source_dir_path.as_path(), dest_dir_path.as_path()) {
                Ok(()) => {},
                Err(e) => {
                    return Err(e);
                },
            }
        } else {
            match fs::copy(source_dir_path.as_path(), dest_dir_path.as_path()) {
                Ok(_) => {},
                Err(e) => {
                    return Err(FileUtilErr::IO(e));
                },
            }
        }
    }
    return Ok(());
}




/// Create a file and write `contents` to it.
///
/// This fucntion creates a file including its parent directory structure writes `contents` to the
/// file. If the file already exists, no action is taken.
pub fn create_and_write<P: AsRef<Path>, C: AsRef<[u8]>>(new_file: P, contents: C) -> Result<(), FileUtilErr> {
    // Check if new_file already exists
    if new_file.as_ref().exists() {
        return Ok(());
    }

    // Check if path is a directory
    if new_file.as_ref().is_dir() {
         return Err(FileUtilErr::WriteToDir);
    }

    // Get the parent of the new file
    let parent_of_new_file = match new_file.as_ref().parent() {
        Some(parent) => parent,
        None => { return Err(FileUtilErr::WriteToDir); },
    };

    // Create the parent if necessary
    if ! parent_of_new_file.exists() {
        match fs::create_dir_all(parent_of_new_file) {
            Ok(()) => {},
            Err(e) => { return Err(FileUtilErr::IO(e)); },
        }
    }

    // finally write the file
    match fs::write(new_file, contents) {
        Ok(_) => {},
        Err(e) => { return Err(FileUtilErr::IO(e)); }
    }

    return Ok(());
}

/// for each item in `src/` move them to `dest/` where `src/` and `dest/` are directories
/// if `dest/` does not exist it will be created
pub fn move_dir<P: AsRef<Path>>(src: P, dest: P, method: Transfer) {
}
