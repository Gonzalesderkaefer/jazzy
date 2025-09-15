
// Create modules
mod config;
mod utils;
mod machine;
mod menu;


// Remove these later:
use utils::fileutils as fu;
use config::packages as pkg;



pub const ARR: &[&'static str] = &[
    "Davey,"
];

pub const ARR1: &[&'static str] = &[
    "how",
    "you",
    "doin'?",
];




// macro_rules! concat {
//     (&[$ty:ty]: $($s:expr),+) => {{
//         const LEN: usize = $( $s.len() + )* 0;
//         const ARR: [$ty; LEN] = {
//             let mut arr: [$ty; LEN] = [""; LEN];
//             let mut base: usize = 0;
//             $({
//                 let mut i = 0;
//                 while i < $s.len() {
//                     arr[base + i] = $s[i];
//                     i += 1;
//                 }
//                 base += $s.len();
//             })*
//             if base != LEN { panic!("invalid length"); }
//             arr
//         };
//         &ARR
//     }}
// }




// macro_rules! concat {
//     (&[$ty:ty]: $($s:expr),+) => {{
//         const LEN: usize = $( $s.len() + )* 0;
//         const ARR: [$ty; LEN] = {
//             let mut arr: [$ty; LEN] = [""; LEN];
//             let mut base: usize = 0;
//             $({
//                 let mut i = 0;
//                 while i < $s.len() {
//                     arr[base + i] = $s[i];
//                     i += 1;
//                 }
//                 base += $s.len();
//             })*
//             if base != LEN { panic!("invalid length"); }
//             arr
//         };
//         &ARR
//     }}
// }



pub const C: &'static [&'static str] = concat!(&[&str]: config::packages::DEB_GUI, config::packages::_DEB_XORG);





fn main() {
    for s in C {
        println!("{s}");
    }

}
