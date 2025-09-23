#![allow(unused_imports)]



mod machine;
mod menu;
mod config;
mod utils;





fn main() {
    match machine::machine::Machine::get() {
        Ok(machine) => {
            println!("{:?}", machine.all_packages);
        }
        Err(_) => return,
    };
}

