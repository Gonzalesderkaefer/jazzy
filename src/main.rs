#![allow(unused_imports)]
mod machine;
mod menu;
mod config;
mod utils;




use std::{env::home_dir, io, path::{Path, PathBuf}, result};
use utils::fileutils as fu;
use machine::transfer;
use config::config as cfg;
use config::custom as cstm;

fn main() {
    run();
}


fn run() -> Result<(), JazzyErr>{
    // Get the home directory
    let home_dir = match std::env::home_dir() {
        Some(home) => home,
        None => return Err(JazzyErr::NoHome(line!(), file!())),
    };


    // Get the machine
    let machine = match machine::machine::Machine::get() {
        Ok(mach) => mach,
        Err(error) => return Err(JazzyErr::MachineErr(error)),
    };

    // Update the machine
    match machine.update() {
        Ok(_) => {}
        Err(error) => return Err(JazzyErr::MachineErr(error)),
    }

    // Install the packages
    match machine.install() {
        Ok(_) => {},
        Err(error) => return Err(JazzyErr::MachineErr(error)),
    }

    // move the config files
    movedir(&home_dir, cfg::CFGSRC, cfg::CFGDEST, &machine.transfer);

    // move the scripts
    movedir(&home_dir, cfg::BINSRC, cfg::BINDEST, &machine.transfer);

    // Create files
    for file in cstm::CUSTOMIZED {
        match fu::create_and_write_user(file.0, file.1, file.2) {
            Ok(_) => {},
            Err(error) => return Err(JazzyErr::FileUtil(error)),
        }
    }

    // Setup the machine
    machine.setup();

    return Ok(());

}

/// Moves `src` to `dest` according to `method`. `src`  and `dest` need to be relative to
/// `home_dir`.
pub fn movedir<P: AsRef<Path>>(home_dir: &PathBuf, src: P, dest: P, method: &transfer::Transfer) {
    // Create full src path
    let mut src_path_buf = home_dir.clone();
    src_path_buf.push(src);

    // Create full dest path
    let mut dest_path_buf = home_dir.clone();
    dest_path_buf.push(dest);

    match fu::move_dir(src_path_buf, dest_path_buf, method.clone()) {
        Ok(_) => {},
        Err(_) => todo!(),
    }
}






/// The main error type for this program. Functions in this module and setup_callback functions
/// for the machine-sub types will return this error.
#[derive(Debug)]
pub enum JazzyErr {
    IO (io::Error),
    MachineErr (machine::machine::MachineError),
    FileUtil (fu::FileUtilErr),
    NoHome (u32, &'static str),
}
impl std::error::Error for JazzyErr {}

impl std::fmt::Display for JazzyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            JazzyErr::IO(error) => return write!(f, "Internal IO Error at: {error}"),
            JazzyErr::MachineErr(error) => return write!(f, "Machine Error {error}"),
            JazzyErr::FileUtil(error) => return write!(f, "File Error: {error}"),
            JazzyErr::NoHome(line, file) => return write!(f, "No $HOME found at: {line}, {file}"),
        }
    }
}






