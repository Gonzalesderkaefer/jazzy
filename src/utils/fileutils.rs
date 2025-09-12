// Libraries
use std::{path::Path};
use std::error::Error;
use std::{fs, io, fmt}; 



/// This enum is used to signal the type of error in this module
#[derive(Debug)]
enum ErrEnum {
    IO (io::Error),
    RootSrc
}

/// This modules Error type
#[derive(Debug)]
pub struct FileUtilErr {
    error: ErrEnum,
}
impl fmt::Display for FileUtilErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.error {
            ErrEnum::IO (err) => {
                return write!(f, "Internal IO Error: {}", err);
            },
            ErrEnum::RootSrc => {
                return write!(f, "Root cannot be the source when copying directories");
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
        return Ok(());
    }

    // Check if to exists
    if dest_path.exists() {
        // Get basename of source
        let source_basename = match source_path.file_name() {
            Some(basename) => basename,
            None => {
                return Err(FileUtilErr {
                    error: ErrEnum::RootSrc,
                });
            }
        };
        // append basename of source to destination
        dest_buffer = dest_path.join(source_basename);

        // change value in dest_path
        dest_path = dest_buffer.as_path();
    }

    // create the destination path
    let create_dir_result = fs::create_dir_all(dest_path);
    match create_dir_result {
        Ok(_) => {},
        Err(e) => { 
            return Err(FileUtilErr {
                error: ErrEnum::IO(e),
            }); 
        },
    }


    // Open from_dir
    let from_dir = match fs::read_dir(source_path) {
        Ok(dir) => dir,
        Err(e) => { 
            return Err(FileUtilErr {
                error: ErrEnum::IO(e),
            }); 
        },
    };

    // Iterate through `from_dir`
    for from_entries in from_dir {
        // Open directory entry as entry
        let opened_entry = match from_entries {
            Ok(entry) => entry,
            Err(e) => { 
                return Err(FileUtilErr {
                    error: ErrEnum::IO(e),
                }); 
            },
        };

        // Get the file_type of the opened_entry
        let opened_file_type = match opened_entry.file_type() {
            Ok (file_type) => file_type,
            Err(e) => {
                return Err(FileUtilErr {
                    error: ErrEnum::IO(e),
                });
            },
        };

        // Get the basename of the opened dir
        let opened_basename = opened_entry.file_name();

        // Build new source directory
        let mut source_dir_path = source_path.to_path_buf();
        source_dir_path.push(&opened_basename);

        // Build new destination directory
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
                    return Err(FileUtilErr {
                        error: ErrEnum::IO(e),
                    });
                },
            }
        }
    }
    return Ok(());
}
