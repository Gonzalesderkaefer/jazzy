#![allow(unused_imports)]



mod machine;
mod menu;
mod config;
mod utils;





fn main() {
    let _ = utils::command::cmd("ls", &["-la", "-t"]);
}

