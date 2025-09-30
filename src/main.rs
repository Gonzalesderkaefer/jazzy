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

use crate::utils::command as cmd;

fn main() {
    match run() {
        Ok(_) => {}
        Err(error) => println!("{error}"),
    };
}


fn run() -> Result<(), JazzyErr>{
    // Get the home directory
    let home_dir = match std::env::home_dir() {
        Some(home) => home,
        None => return Err(JazzyErr::NoHome(line!(), file!())),
    };

    let home_str = match home_dir.to_str() {
        Some(string) => string,
        None => return Err(JazzyErr::NoHome(line!(), file!())),
    };


    // Get the machine
    let machine = match machine::machine::Machine::get() {
        Ok(mach) => mach,
        Err(error) => return Err(JazzyErr::MachineErr(error, line!(), file!())),
    };

    // Update the machine
    match machine.update() {
        Ok(_) => {}
        Err(error) => return Err(JazzyErr::MachineErr(error, line!(), file!())),
    }

    // Install the packages
    match machine.install() {
        Ok(_) => {},
        Err(error) => return Err(JazzyErr::MachineErr(error, line!(), file!())),
    }

    // move the config files
    movedir(&home_dir, cfg::CFGSRC, cfg::CFGDEST, &machine.transfer, false)?;

    // move the scripts
    movedir(&home_dir, cfg::BINSRC, cfg::BINDEST, &machine.transfer, false)?;

    // move the shell configuratons
    movedir(&home_dir, cfg::SHELLCFG, home_str, &machine.transfer, true)?;


    // Create files
    for file in cstm::CUSTOMIZED {
        match fu::create_and_write_user(file.0, file.1, file.2) {
            Ok(_) => {},
            Err(error) => return Err(JazzyErr::FileUtil(error, line!(), file!())),
        }
    }

    // Setup the machine
    (machine.display_server.setup_callback)();
    (machine.distro.setup_callback)();
    (machine.gui.setup_callback)();

    // Run final commands

    // Gsettings stuff for theming
    match cmd::cmd("gsettings", &["set", "org.gnome.desktop.interface", "gtk-theme", "\'Adwaita-dark\'"]) {
        Ok(_) => {},
        Err(error) => return Err(JazzyErr::Command(error, line!(), file!())),
    };
    match cmd::cmd("gsettings", &["set", "org.gnome.desktop.interface", "color-scheme", "\'prefer-dark\'"]) {
        Ok(_) => {},
        Err(error) => return Err(JazzyErr::Command(error, line!(), file!())),
    };
    match cmd::cmd("gsettings", &["set", "org.gnome.desktop.interface", "icon-theme", "\'Papirus-Dark\'"]) {
        Ok(_) => {},
        Err(error) => return Err(JazzyErr::Command(error, line!(), file!())),
    };
    // Clear the screen
    print!("{}[2J", 27 as char);

    println!("{}Changing the shell to zsh. You will be prompted for your user password{}",
        FgColor!(Green), FgColor!());

    match cmd::cmd("chsh", &["-s", "/usr/bin/zsh"]) {
        Ok(_) => {},
        Err(error) => return Err(JazzyErr::Command(error, line!(), file!())),
    };




    return Ok(());

}

/// Moves `src` to `dest` according to `method`. `src`  and `dest` need to be relative to
/// `home_dir`.
pub fn movedir<P: AsRef<Path>>(home_dir: &PathBuf, src: P, dest: P, method: &transfer::Transfer, hide: bool) -> Result<(), JazzyErr>{
    // Create full src path
    let mut src_path_buf = home_dir.clone();
    src_path_buf.push(src);

    // Create full dest path
    let mut dest_path_buf = home_dir.clone();
    dest_path_buf.push(dest);

    match fu::move_dir(src_path_buf, dest_path_buf, method.clone(), hide) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(JazzyErr::FileUtil(error, line!(), file!())),
    }
}






/// The main error type for this program. Functions in this module and setup_callback functions
/// for the machine-sub types will return this error.
#[derive(Debug)]
pub enum JazzyErr {
    IO (io::Error),
    MachineErr (machine::machine::MachineError, u32, &'static str),
    FileUtil (fu::FileUtilErr, u32, &'static str),
    NoHome (u32, &'static str),
    Command (cmd::CommandError, u32, &'static str)
}
impl std::error::Error for JazzyErr {}

impl std::fmt::Display for JazzyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            JazzyErr::IO(error) => return write!(f, "Internal IO Error at: {error}"),
            JazzyErr::MachineErr(error, line, file) => return write!(f, "Machine Error at {line} in {file} :{error}"),
            JazzyErr::FileUtil(error, line, file) => return write!(f, "File Error at {line} in {file} : {error}"),
            JazzyErr::NoHome(line, file) => return write!(f, "No $HOME found at: {line}, {file}"),
            JazzyErr::Command(error, line, file) => return write!(f, "Error running command at {line}, {file}: {error}"),
        }
    }
}






