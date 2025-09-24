#![allow(unused_imports)]



mod machine;
mod menu;
mod config;
mod utils;





fn main() {
    match utils::fileutils::move_dir("/home/user1/.config/", "/home/user1/Documents/code/rust/configstuff", machine::transfer::Transfer::Copy) {
        Ok(_) => {}
        Err(error) => println!("{error}")
    };
}

