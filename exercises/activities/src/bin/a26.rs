// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    println!("Hello, world!");
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("🚀 ~ file: main.rs ~ line 7 ~ fnmain ~ local {}", local.format("%a %b %e %T %Y").to_string());
    println!("🚀 ~ file: main.rs ~ line 6 ~ fnmain ~ utc {}", utc);
}
