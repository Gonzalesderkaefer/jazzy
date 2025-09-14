
// Create modules
mod config;
mod utils;
mod machine;
mod menu;

use utils::fileutils as fu;

fn main() {

    let mut self_as_string = String::from("Sway");

    // Insert square brackets
    self_as_string.insert(0, '[');
    self_as_string.insert(2, ']');

    println!("{self_as_string}");
}
