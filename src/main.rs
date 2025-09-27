#![allow(unused_imports)]
mod machine;
mod menu;
mod config;
mod utils;




use std::{env::home_dir, path::{Path, PathBuf}};
use utils::fileutils as fu;
use machine::transfer;
use config::config as cfg;
use config::custom as cstm;

fn main() {
    run();
}


fn run() {
    // Get the home directory
    let home_dir = match std::env::home_dir() {
        Some(home) => home,
        None => todo!(),
    };


    // Get the machine
    let machine = match machine::machine::Machine::get() {
        Ok(mach) => mach,
        Err(_) => todo!(),
    };

    // Update the machine
    match machine.update() {
        Ok(_) => {}
        Err(_) => todo!(),
    }

    // Install the packages
    match machine.install() {
        Ok(_) => {},
        Err(_) => todo!(),
    }

    // move the config files
    movedir(&home_dir, cfg::CFGSRC, cfg::CFGDEST, &machine.transfer);

    // move the scripts
    movedir(&home_dir, cfg::BINSRC, cfg::BINDEST, &machine.transfer);

    // Create files
    for file in cstm::CUSTOMIZED {
        match fu::create_and_write_user(file.0, file.1, file.2) {
            Ok(_) => {},
            Err(_) => todo!(),
        }
    }

    // Setup the machine
    machine.setup();

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
