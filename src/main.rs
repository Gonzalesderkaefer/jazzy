#![allow(unused_imports)]



mod machine;
mod menu;
mod config;
mod utils;





fn main() {
    let this_machine = match machine::machine::Machine::get() {
        Ok(machine) => {
            machine
        }
        Err(_) => return,
    };

    this_machine.update();
    this_machine.install();


}

