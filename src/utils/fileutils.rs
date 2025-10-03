use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::fs::File;
use std::{path::{Path, PathBuf}};
use std::error::Error;
use std::{fs, io, fmt};

use crate::machine::transfer::Transfer;



/// This module's Error type
#[derive(Debug)]
pub enum FileUtilErr {
    // copy_dir
    IO (io::Error, u32, &'static str),
    RootSrc (u32, &'static str),
    NoSource (String, u32, &'static str),
    InvalidSrc (u32, &'static str),

    // create_file_and_write
    WriteToDir (u32, &'static str),

    // create_file_and_write_user
    NoHome (u32, &'static str),


}
impl fmt::Display for FileUtilErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Self::IO (err, line, file) => {
                return write!(f, "Internal IO Error at {line} in {file}: {}", err);
            },
            Self::RootSrc(line, file) => {
                return write!(f, "Root cannot be the source when copying directories at {line} in {file}");
            },
            Self::NoSource (source, line, file) => {
                return write!(f, "No source: \"{source}\" at {line} in {file}");
            },
            Self::InvalidSrc (line, file) => {
                return write!(f, "Source is not valid unicode at {line} in {file}");
            },
            Self::WriteToDir (line, file) => {
                return write!(f, "Can't write to a directory at {line} in {file}");
            },
            Self::NoHome (line, file) => {
                return write!(f, "No home directory found at {line} in {file}");
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
            None => return Err(FileUtilErr::InvalidSrc(line!(), file!())),
        };
        return Err(FileUtilErr::NoSource(String::from(source_path_str), line!(), file!()));
    }

    // Check if to exists
    if dest_path.exists() {
        // Get basename of source
        let source_basename = match source_path.file_name() {
            Some(basename) => basename,
            None => {
                return Err(FileUtilErr::RootSrc(line!(), file!()));
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
            return Err(FileUtilErr::IO(e, line!(), file!())); 
        },
    }


    // Open source_dir
    let from_dir = match fs::read_dir(source_path) {
        Ok(dir) => dir,
        Err(e) => { 
            return Err(FileUtilErr::IO(e, line!(), file!())); 
        },
    };

    // Iterate through source_dir
    for from_entries in from_dir {
        // Open directory entry as entry
        let opened_entry = match from_entries {
            Ok(entry) => entry,
            Err(e) => { 
                return Err(FileUtilErr::IO(e, line!(), file!())); 
            },
        };

        // Get the file_type of the opened_entry
        let opened_file_type = match opened_entry.file_type() {
            Ok (file_type) => file_type,
            Err(e) => {
                return Err(FileUtilErr::IO(e, line!(), file!()));
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
        } else if opened_file_type.is_symlink() {
            // Get the target of the link
            let target_path = match fs::read_link(source_dir_path) {
                Ok(path) => path,
                Err(error) => return Err(FileUtilErr::IO(error, line!(), file!())),
            };

            // Create the new symlink
            match std::os::unix::fs::symlink(target_path, dest_dir_path) {
                Ok(_) => {},
                Err(error) => return Err(FileUtilErr::IO(error, line!(), file!()))
            }



        } else if opened_file_type.is_file() {
            match fs::copy(source_dir_path.as_path(), dest_dir_path.as_path()) {
                Ok(_) => {},
                Err(e) => {
                    return Err(FileUtilErr::IO(e, line!(), file!()));
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
#[allow(unused)]
pub fn create_and_write<P: AsRef<Path>, C: AsRef<[u8]>>(new_file: P, contents: C, mode: u32) -> Result<(), FileUtilErr> {
    // Check if new_file already exists
    if new_file.as_ref().exists() {
        return Ok(());
    }

    // Check if path is a directory
    if new_file.as_ref().is_dir() {
         return Err(FileUtilErr::WriteToDir(line!(), file!()));
    }

    // Get the parent of the new file
    let parent_of_new_file = match new_file.as_ref().parent() {
        Some(parent) => parent,
        None => return Err(FileUtilErr::WriteToDir(line!(), file!())), // This will probably never happen because `new_file` is not a directory
    };

    // Create the parent if necessary
    if ! parent_of_new_file.exists() {
        match fs::create_dir_all(parent_of_new_file) {
            Ok(()) => {},
            Err(e) => { return Err(FileUtilErr::IO(e, line!(), file!())); },
        }
    }

    // finally write the file
    match fs::write(&new_file, contents) {
        Ok(_) => {},
        Err(e) => return Err(FileUtilErr::IO(e, line!(), file!())),
    }

    // Set permissions for the file
    match fs::set_permissions(new_file, fs::Permissions::from_mode(mode)) {
        Ok(_) => {},
        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!())),
    }



    return Ok(());
}









/// Create a file and write `contents` to it.
///
/// This fucntion does what [create_and_write] does but `new_file` is relative to
/// `/home/<username>`
pub fn create_and_write_user<P: AsRef<Path>, C: AsRef<[u8]>>(new_file: P, contents: C, mode: u32) -> Result<(), FileUtilErr> {

    // Get the home directory
    let mut home_dir = match std::env::home_dir() {
        Some(home) => home,
        None => return Err(FileUtilErr::NoHome(line!(), file!())),
    };

    let full_path = {
        // Push file path to home_dir
        home_dir.push(&new_file);
        home_dir
    };


    // Check if new_file already exists
    if full_path.as_path().exists() {
        // Create new backup path for the file
        let mut backup_path_os_string = full_path.clone().into_os_string();
        backup_path_os_string.push(".bac");
        let backup_path = PathBuf::from(backup_path_os_string);

        // Move the file
        match fs::rename(&full_path, backup_path) {
            Ok(_) => {},
            Err(error) => return Err(FileUtilErr::IO(error, line!(), file!()))
        }
    }

    // Check if path is a directory
    if full_path.as_path().is_dir() {
         return Err(FileUtilErr::WriteToDir(line!(), file!()));
    }

    // Get the parent of the new file
    let parent_of_new_file = match full_path.as_path().parent() {
        Some(parent) => parent,
        None => return Err(FileUtilErr::WriteToDir(line!(), file!())), // This will probably never happen because `new_file` is not a directory
    };

    // Create the parent if necessary
    if ! parent_of_new_file.exists() {
        match fs::create_dir_all(parent_of_new_file) {
            Ok(()) => {},
            Err(e) => { return Err(FileUtilErr::IO(e, line!(), file!())); },
        }
    }

    // finally write the file
    match fs::write(&full_path, contents) {
        Ok(_) => {},
        Err(e) => return Err(FileUtilErr::IO(e, line!(), file!())),
    }

    // Set permissions for the file
    match fs::set_permissions(full_path, fs::Permissions::from_mode(mode)) {
        Ok(_) => {},
        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!())),
    }



    return Ok(());
}







/// Get all files that are in this directory
#[allow(dead_code)]
pub fn sub_paths<P: AsRef<Path>>(directory: P, paths: &mut Vec<String>) -> Result<(), FileUtilErr>{
    // Check if the directory exists
    if ! directory.as_ref().exists() {
        match directory.as_ref().to_str() {
            Some(directory_name) => return Err(FileUtilErr::NoSource(String::from(directory_name), line!(), file!())),
            None => todo!(), // I don't know, what to do here
        }
    }

    // Read directory
    let read_directory = match fs::read_dir(directory) {
        Ok(read_dir) => read_dir,
        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!())),
    };

    for entry in read_directory {
        // Open directory entry as entry
        let opened_entry = match entry {
            Ok(entry) => entry,
            Err(e) => { 
                return Err(FileUtilErr::IO(e, line!(), file!())); 
            },
        };

        // Add entry to paths
        match opened_entry.path().to_str() {
            Some(full_path) => paths.push(String::from(full_path)),
            None => {},
        }

        // Get the file_type of the opened_entry
        let opened_file_type = match opened_entry.file_type() {
            Ok (file_type) => file_type,
            Err(e) => {
                return Err(FileUtilErr::IO(e, line!(), file!()));
            },
        };

        // See if opened element is a directory then call recursively
        if opened_file_type.is_dir() {
            match sub_paths(opened_entry.path(), paths) {
                Ok(_) => {},
                Err(error) => return Err(error),
            }
        };
    }
    return Ok(());
}










/// for each item in `src/` move them to `dest/` where `src/` and `dest/` are directories
/// if `dest/` does not exist it will be created
pub fn move_dir<P: AsRef<Path>>(src: P, dest: P, method: Transfer, hide: bool) -> Result<(), FileUtilErr> {
    // Check if nothing needs to be done
    if let Transfer::None = method {
        return Ok(());
    }

    // Check if dest exists
    match fs::exists(&dest) {
        Ok(value) => if !value {
            // Create and check for error
            match fs::create_dir_all(&dest) {
                Ok(_) => {},
                Err(error) => return Err(FileUtilErr::IO(error, line!(), file!()))
            };
        }
        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!())),
    };



    // Open source_dir
    let source_dir = match fs::read_dir(&src) {
        Ok(dir) => dir,
        Err(e) => { 
            return Err(FileUtilErr::IO(e, line!(), file!())); 
        },
    };



    // Iterate through source_dir
    for src_entries in source_dir {
        // Open directory entry as entry
        let opened_entry = match src_entries {
            Ok(entry) => entry,
            Err(e) => { 
                return Err(FileUtilErr::IO(e, line!(), file!())); 
            },
        };

        // Get the file_type of the opened_entry
        let opened_file_type = match opened_entry.file_type() {
            Ok (file_type) => file_type,
            Err(e) => {
                return Err(FileUtilErr::IO(e, line!(), file!()));
            },
        };

        // Get the basename of the opened entry
        let opened_basename = opened_entry.file_name();

        // Build new source path
        let mut source_dir_path = src.as_ref().to_path_buf();
        source_dir_path.push(&opened_basename);

        // Build new destination path
        let mut dest_dir_path = dest.as_ref().to_path_buf();

        // Check if destination should be hidden
        if hide {
            let mut hidden = OsString::new();
            hidden.push(".");
            hidden.push(opened_basename);
            dest_dir_path.push(hidden);
        } else {
            dest_dir_path.push(opened_basename);
        }


        match method {
            Transfer::Link => {
                if dest_dir_path.as_path().exists() {
                    match std::os::unix::fs::symlink(source_dir_path, dest_dir_path) {
                        Ok(_) => {},
                        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!()))
                    }
                }
            },
            Transfer::Copy => {
                // Check if source is directory
                if opened_file_type.is_dir() {
                    copy_dir(source_dir_path, dest_dir_path)?;
                } else if opened_file_type.is_symlink() {
                    // Get the target of the link
                    let target_path = match fs::read_link(source_dir_path) {
                        Ok(path) => path,
                        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!())),
                    };

                    // Create the new symlink
                    match std::os::unix::fs::symlink(target_path, dest_dir_path) {
                        Ok(_) => {},
                        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!()))
                    }



                }else if opened_file_type.is_file() {
                    match fs::copy(&source_dir_path, dest_dir_path) {
                        Ok(_) => {},
                        Err(error) => return Err(FileUtilErr::IO(error, line!(), file!())),
                    };
                }
            },
            Transfer::None => return Ok(())
        }
    }
    Ok(())
}
