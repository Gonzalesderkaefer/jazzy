#![allow(unused_imports)]



mod machine;
mod menu;
mod config;
mod utils;





fn main() {
    match machine::machine::Machine::get() {
        Ok(machine) => {
            println!("{:?}", machine);
        }
        Err(_) => return,
    };
}

